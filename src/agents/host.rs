#![allow(dead_code)]

use std::io::{BufRead, BufReader};
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

    let mut child = Command::new(&meta.install_path)
        .env("PATH", &controlled_path)
        .env("HOME", std::env::var("HOME").unwrap_or_default())
        .env("ROY_SESSION_ID", config.session_id.to_string())
        .envs(config.env_overrides)
        .current_dir(&config.workspace_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AgentError::launch_failed(e.to_string()))?;

    let pid = child.id();
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| AgentError::io_error("child stdout was not piped"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| AgentError::io_error("child stderr was not piped"))?;

    let queue: Arc<Mutex<Vec<SupervisionEvent>>> = Arc::new(Mutex::new(Vec::new()));

    spawn_line_reader(Arc::clone(&queue), stdout, config.session_id, label, false);
    spawn_line_reader(Arc::clone(&queue), stderr, config.session_id, label, true);

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

fn spawn_line_reader<R>(
    queue: Arc<Mutex<Vec<SupervisionEvent>>>,
    reader: R,
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
            let buf = BufReader::new(reader);
            for line in buf.lines().map_while(Result::ok) {
                let event = if is_stderr {
                    SupervisionEvent::ErrorLine { text: line }
                } else {
                    SupervisionEvent::OutputLine { text: line }
                };
                if let Ok(mut q) = queue.lock() {
                    q.push(event);
                }
            }
        })
        .ok();
}
