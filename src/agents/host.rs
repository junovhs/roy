#![allow(dead_code)]
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use portable_pty::{native_pty_system, CommandBuilder, PtySize};

use super::adapter::{AgentError, AgentHandle, AgentMeta, LaunchConfig, SupervisionEvent};

/// Terminal dimensions reported to the hosted agent.
const PTY_COLS: u16 = 220;
const PTY_ROWS: u16 = 50;

pub(super) fn launch_supervised_agent(
    meta: &AgentMeta,
    config: LaunchConfig,
    label: &str,
) -> Result<AgentHandle, AgentError> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize { rows: PTY_ROWS, cols: PTY_COLS, pixel_width: 0, pixel_height: 0 })
        .map_err(|e| AgentError::launch_failed(e.to_string()))?;

    let roy_bin = crate::shell::ShellEnv::roy_path();
    let sys_path = std::env::var("PATH").unwrap_or_default();

    let mut cmd = CommandBuilder::new(&meta.install_path);
    cmd.env("PATH", format!("{roy_bin}:{sys_path}"));
    cmd.env("HOME", std::env::var("HOME").unwrap_or_default());
    cmd.env("ROY_SESSION_ID", config.session_id.to_string());
    cmd.env("COLUMNS", PTY_COLS.to_string());
    cmd.env("LINES", PTY_ROWS.to_string());
    cmd.env("TERM", "xterm-256color");
    for (k, v) in config.env_overrides {
        cmd.env(k, v);
    }
    cmd.cwd(&config.workspace_root);

    let child = pair.slave
        .spawn_command(cmd)
        .map_err(|e| AgentError::launch_failed(e.to_string()))?;

    let pid = child.process_id().unwrap_or(0);

    // Extract reader (dup of master fd) before dropping pair.
    let master_reader = pair.master
        .try_clone_reader()
        .map_err(|e| AgentError::io_error(e.to_string()))?;
    let master_writer = pair.master
        .take_writer()
        .map_err(|e| AgentError::io_error(e.to_string()))?;
    // pair.master and pair.slave drop here; reader/writer keep the PTY alive.

    let queue: Arc<Mutex<Vec<SupervisionEvent>>> = Arc::new(Mutex::new(Vec::new()));

    spawn_pty_reader(Arc::clone(&queue), master_reader, config.session_id, label);

    let exit_q = Arc::clone(&queue);
    let sid = config.session_id;
    let exit_label = label.to_string();
    let mut child = child;
    thread::Builder::new()
        .name(format!("roy-{exit_label}-exit-{sid}"))
        .spawn(move || {
            let code = child.wait().map(|s| s.exit_code() as i32).unwrap_or(-1);
            if let Ok(mut q) = exit_q.lock() {
                q.push(SupervisionEvent::ProcessExited { code });
            }
        })
        .map_err(|e| AgentError::io_error(e.to_string()))?;

    let mut handle = AgentHandle::new(meta.clone(), config.session_id);
    handle.push_event(SupervisionEvent::AgentStarted { pid });
    handle.set_stdin(Arc::new(Mutex::new(Box::new(master_writer) as Box<dyn Write + Send>)));
    handle.set_pending(queue);
    Ok(handle)
}

pub(super) fn discover_binary(name: &str) -> Result<PathBuf, AgentError> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    for dir in path_var.split(':') {
        let candidate = PathBuf::from(dir).join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(AgentError::not_installed(name))
}

pub(super) fn probe_version(binary: &PathBuf) -> Result<String, AgentError> {
    std::process::Command::new(binary)
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .map_err(|e| AgentError::launch_failed(e.to_string()))
}

fn spawn_pty_reader<R>(
    queue: Arc<Mutex<Vec<SupervisionEvent>>>,
    mut reader: R,
    session_id: u64,
    label: &str,
) where
    R: std::io::Read + Send + 'static,
{
    let name = format!("roy-{label}-pty-{session_id}");
    thread::Builder::new()
        .name(name)
        .spawn(move || {
            let mut buf = [0_u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(mut q) = queue.lock() {
                            q.push(SupervisionEvent::OutputChunk {
                                bytes: buf[..n].to_vec(),
                                is_stderr: false,
                            });
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                }
            }
        })
        .ok();
}
