#![allow(dead_code)]
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use super::adapter::{AgentError, AgentHandle, AgentMeta, LaunchConfig, SupervisionEvent};

pub(super) fn launch_supervised_agent(
    meta: &AgentMeta,
    config: LaunchConfig,
    label: &str,
) -> Result<AgentHandle, AgentError> {
    let roy_bin = crate::shell::ShellEnv::roy_path();
    let sys_path = std::env::var("PATH").unwrap_or_default();
    let controlled_path = format!("{roy_bin}:{sys_path}");
    let script_path = discover_binary("script")?;

    let mut cmd = Command::new(script_path);
    cmd.env("PATH", &controlled_path)
        .env("HOME", std::env::var("HOME").unwrap_or_default())
        .env("ROY_SESSION_ID", config.session_id.to_string())
        .envs(config.env_overrides)
        .current_dir(&config.workspace_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    configure_pty_wrapper(&mut cmd, meta);

    let mut child = cmd
        .spawn()
        .map_err(|e| AgentError::launch_failed(e.to_string()))?;

    let pid = child.id();
    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| AgentError::io_error("child stdin was not piped"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| AgentError::io_error("child stdout was not piped"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| AgentError::io_error("child stderr was not piped"))?;

    let queue: Arc<Mutex<Vec<SupervisionEvent>>> = Arc::new(Mutex::new(Vec::new()));

    spawn_chunk_reader(Arc::clone(&queue), stdout, config.session_id, label, false);
    spawn_chunk_reader(Arc::clone(&queue), stderr, config.session_id, label, true);

    let exit_q = Arc::clone(&queue);
    let sid = config.session_id;
    let exit_label = label.to_string();
    thread::Builder::new()
        .name(format!("roy-{exit_label}-exit-{sid}"))
        .spawn(move || {
            let code = match child.wait().map(|s| s.code()) {
                Ok(Some(c)) => c,
                _ => -1,
            };
            if let Ok(mut q) = exit_q.lock() {
                q.push(SupervisionEvent::ProcessExited { code });
            }
        })
        .map_err(|e| AgentError::io_error(e.to_string()))?;

    let mut handle = AgentHandle::new(meta.clone(), config.session_id);
    handle.push_event(SupervisionEvent::AgentStarted { pid });
    handle.set_stdin(Arc::new(Mutex::new(Box::new(stdin))));
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
    Command::new(binary)
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .map_err(|e| AgentError::launch_failed(e.to_string()))
}

fn spawn_chunk_reader<R>(
    queue: Arc<Mutex<Vec<SupervisionEvent>>>,
    mut reader: R,
    session_id: u64,
    label: &str,
    is_stderr: bool,
) where
    R: std::io::Read + Send + 'static,
{
    let stream = if is_stderr { "stderr" } else { "stdout" };
    let name = format!("roy-{label}-{stream}-{session_id}");
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
                                is_stderr,
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

fn configure_pty_wrapper(cmd: &mut Command, meta: &AgentMeta) {
    if cfg!(target_os = "macos") {
        cmd.arg("-q").arg("/dev/null").arg(&meta.install_path);
        return;
    }

    let command = shell_escape(meta.install_path.to_string_lossy().as_ref());
    cmd.arg("-q")
        .arg("-e")
        .arg("-c")
        .arg(command)
        .arg("/dev/null");
}

fn shell_escape(raw: &str) -> String {
    if raw.is_empty() {
        return "''".to_string();
    }

    let mut escaped = String::with_capacity(raw.len() + 2);
    escaped.push('\'');
    for ch in raw.chars() {
        if ch == '\'' {
            escaped.push_str("'\"'\"'");
        } else {
            escaped.push(ch);
        }
    }
    escaped.push('\'');
    escaped
}
