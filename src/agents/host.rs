#![allow(dead_code)]
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use portable_pty::{native_pty_system, CommandBuilder, PtySize};

use super::adapter::{
    AgentError, AgentHandle, AgentKind, AgentMeta, LaunchConfig, SupervisionEvent,
};

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
        .openpty(PtySize {
            rows: PTY_ROWS,
            cols: PTY_COLS,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| AgentError::launch_failed(e.to_string()))?;

    let sys_path = std::env::var("PATH").unwrap_or_default();
    // Prepend the roy bin dir (Linux only — on Windows it's a Unix path stub
    // that would corrupt the Windows DLL search path).
    let new_path: std::ffi::OsString = {
        #[cfg(not(windows))]
        {
            let roy_bin = crate::shell::ShellEnv::roy_path();
            std::env::join_paths(
                std::iter::once(std::path::PathBuf::from(&roy_bin))
                    .chain(std::env::split_paths(&sys_path)),
            )
            .unwrap_or_else(|_| format!("{roy_bin}:{sys_path}").into())
        }
        #[cfg(windows)]
        {
            sys_path.into()
        }
    };

    let mut cmd = build_command(meta);

    // Seed the child's environment from the parent so that critical OS variables
    // (SystemRoot, USERPROFILE, APPDATA, TEMP, …) are present.  On Windows,
    // portable-pty's CommandBuilder does NOT inherit the parent environment
    // automatically — it passes only the vars that are explicitly set.
    for (key, val) in std::env::vars_os() {
        cmd.env(key, val);
    }

    // Now apply ROY-specific overrides on top of the inherited environment.
    cmd.env("PATH", new_path);
    cmd.env("ROY_SESSION_ID", config.session_id.to_string());
    cmd.env("COLUMNS", PTY_COLS.to_string());
    cmd.env("LINES", PTY_ROWS.to_string());
    cmd.env("TERM", "xterm-256color");
    for (k, v) in config.env_overrides {
        cmd.env(k, v);
    }
    cmd.cwd(&config.workspace_root);

    let child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| AgentError::launch_failed(e.to_string()))?;

    let pid = child.process_id().unwrap_or(0);

    // Extract reader (dup of master fd) before dropping pair.
    let master_reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| AgentError::io_error(e.to_string()))?;
    let master_writer = pair
        .master
        .take_writer()
        .map_err(|e| AgentError::io_error(e.to_string()))?;
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
    handle.set_stdin(Arc::new(Mutex::new(
        Box::new(master_writer) as Box<dyn Write + Send>
    )));
    handle.set_pty_master(pair.master);
    handle.set_pending(queue);
    Ok(handle)
}

fn build_command(meta: &AgentMeta) -> CommandBuilder {
    #[cfg(windows)]
    {
        if matches!(meta.kind, AgentKind::ClaudeCode) {
            let comspec = std::env::var_os("ComSpec").unwrap_or_else(|| "cmd.exe".into());
            let mut cmd = CommandBuilder::new(comspec);
            cmd.arg("/d");
            cmd.arg("/s");
            cmd.arg("/c");
            cmd.arg(&meta.install_path);
            return cmd;
        }
    }

    CommandBuilder::new(&meta.install_path)
}

pub(super) fn discover_binary(name: &str) -> Result<PathBuf, AgentError> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    // On Windows, executables may have .cmd, .exe, or .bat extensions.
    #[cfg(windows)]
    let extensions = &["", ".exe", ".cmd", ".bat"][..];
    #[cfg(not(windows))]
    let extensions = &[""][..];

    for dir in std::env::split_paths(&path_var) {
        for ext in extensions {
            let candidate = dir.join(format!("{name}{ext}"));
            if candidate.is_file() {
                return Ok(candidate);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn claude_launch_uses_cmd_wrapper_on_windows() {
        let meta = AgentMeta {
            kind: AgentKind::ClaudeCode,
            version: "test".to_string(),
            install_path: PathBuf::from(r"C:\Users\name\.local\bin\claude.exe"),
        };
        let cmd = build_command(&meta);
        let argv = cmd
            .get_argv()
            .iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert!(
            argv[0].to_ascii_lowercase().contains("cmd"),
            "expected cmd.exe wrapper, got {argv:?}"
        );
        assert_eq!(argv[1..4], ["/d", "/s", "/c"]);
        assert_eq!(argv[4], r"C:\Users\name\.local\bin\claude.exe");
    }

    #[cfg(not(windows))]
    #[test]
    fn non_windows_launches_binary_directly() {
        let meta = AgentMeta {
            kind: AgentKind::ClaudeCode,
            version: "test".to_string(),
            install_path: PathBuf::from("/usr/local/bin/claude"),
        };
        let cmd = build_command(&meta);
        let argv = cmd
            .get_argv()
            .iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert_eq!(argv, vec!["/usr/local/bin/claude"]);
    }
}
