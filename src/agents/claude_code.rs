// Live in tests; UI wiring and session-ledger integration pending downstream.
#![allow(dead_code)]

//! Concrete adapter for hosting Claude Code inside the ROY shell.
//!
//! Spawns `claude` with PATH prepended by ROY's bin dir (so blocked commands
//! are absent before any OS fallback), then drives stdout/stderr/exit via
//! supervision threads whose events are collected via [`AgentHandle::drain_pending`].

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use super::adapter::{
    AgentAdapter, AgentAuthMethod, AgentError, AgentHandle, AgentKind, AgentMeta, LaunchConfig,
    SupervisionEvent,
};

// ── adapter ───────────────────────────────────────────────────────────────────

/// Adapter for hosting Claude Code inside the ROY shell.
///
/// Use [`ClaudeCodeAdapter::discover()`] in production to auto-detect the
/// installed `claude` binary, or [`ClaudeCodeAdapter::from_path()`] in tests.
#[derive(Debug)]
pub struct ClaudeCodeAdapter {
    meta: AgentMeta,
}

impl ClaudeCodeAdapter {
    /// Locate `claude` in the system PATH and read its version string.
    ///
    /// Returns [`AgentError`] with kind `NotInstalled` if the binary is absent.
    pub fn discover() -> Result<Self, AgentError> {
        let install_path = which("claude")?;
        let version = probe_version(&install_path).unwrap_or_else(|_| "unknown".to_string());
        Ok(Self {
            meta: AgentMeta {
                kind: AgentKind::ClaudeCode,
                version,
                install_path,
            },
        })
    }

    /// Build the adapter from a known binary path, bypassing auto-discovery.
    ///
    /// No filesystem check is performed — intended for tests.
    pub fn from_path(install_path: PathBuf, version: impl Into<String>) -> Self {
        Self {
            meta: AgentMeta {
                kind: AgentKind::ClaudeCode,
                version: version.into(),
                install_path,
            },
        }
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    fn meta(&self) -> &AgentMeta {
        &self.meta
    }

    /// Claude Code uses OAuth device flow for first-time authentication.
    ///
    /// When `ANTHROPIC_API_KEY` is present in `LaunchConfig::env_overrides`
    /// it is forwarded to the subprocess and no interactive login is needed.
    fn auth_method(&self) -> AgentAuthMethod {
        AgentAuthMethod::OauthDevice
    }

    /// Spawn Claude Code with a controlled PATH and start supervision threads.
    ///
    /// Sets `PATH=<roy_bin>:<sys_PATH>`, `ROY_SESSION_ID`, and any caller
    /// `env_overrides`. Returns a handle with `AgentStarted` recorded; caller
    /// must poll [`AgentHandle::drain_pending`] to collect subsequent events.
    fn launch(&self, config: LaunchConfig) -> Result<AgentHandle, AgentError> {
        let roy_bin = crate::shell::ShellEnv::roy_path();
        let sys_path = std::env::var("PATH").unwrap_or_default();
        let controlled_path = format!("{roy_bin}:{sys_path}");

        let mut child = Command::new(&self.meta.install_path)
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

        spawn_line_reader(Arc::clone(&queue), stdout, config.session_id, false);
        spawn_line_reader(Arc::clone(&queue), stderr, config.session_id, true);

        let exit_q = Arc::clone(&queue);
        let sid = config.session_id;
        thread::Builder::new()
            .name(format!("roy-claude-exit-{sid}"))
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

        let mut handle = AgentHandle::new(self.meta.clone(), config.session_id);
        handle.push_event(SupervisionEvent::AgentStarted { pid });
        handle.set_pending(queue);
        Ok(handle)
    }
}

// ── supervision ───────────────────────────────────────────────────────────────

/// Spawn a reader thread that pushes lines from `reader` into `queue`.
///
/// `is_stderr = true` → [`SupervisionEvent::ErrorLine`];
/// `is_stderr = false` → [`SupervisionEvent::OutputLine`].
fn spawn_line_reader<R>(
    queue: Arc<Mutex<Vec<SupervisionEvent>>>,
    reader: R,
    session_id: u64,
    is_stderr: bool,
) where
    R: std::io::Read + Send + 'static,
{
    let name = if is_stderr {
        format!("roy-claude-stderr-{session_id}")
    } else {
        format!("roy-claude-stdout-{session_id}")
    };
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

// ── binary discovery ──────────────────────────────────────────────────────────

/// Search `$PATH` for an executable file named `name`.
///
/// Uses the *system* PATH (not ROY's controlled PATH) because this runs in the
/// host process before the agent subprocess is spawned.
fn which(name: &str) -> Result<PathBuf, AgentError> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    for dir in path_var.split(':') {
        let candidate = PathBuf::from(dir).join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(AgentError::not_installed(name))
}

/// Run `binary --version` and return trimmed stdout.
fn probe_version(binary: &PathBuf) -> Result<String, AgentError> {
    Command::new(binary)
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .map_err(|e| AgentError::launch_failed(e.to_string()))
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[path = "claude_code_tests.rs"]
mod tests;
