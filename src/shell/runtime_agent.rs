use crate::agents::adapter::{AgentAdapter, LaunchConfig, SupervisionEvent};
use crate::agents::claude_code::ClaudeCodeAdapter;

use super::*;

impl ShellRuntime {
    /// Launch an embedded agent by command name (e.g. `"claude"`).
    ///
    /// Checks prerequisites in order:
    ///
    /// 1. No agent currently running.
    /// 2. Binary discoverable on PATH.
    ///
    /// Then spawns, stores the handle, and returns `AgentStarted`. Claude Code
    /// is allowed to rely on its own persisted login state or interactive auth
    /// flow; ROY should not preemptively block launch on missing API key.
    pub(super) fn dispatch_agent_launch(&mut self, command: &str) -> DispatchResult {
        if self.agent_handle.is_some() {
            let msg =
                "an agent session is already active — exit it before launching another".to_string();
            self.io.write_error(&msg);
            self.last_exit_status = Some(1);
            return DispatchResult::Executed {
                output: msg,
                exit_code: 1,
                artifacts: Vec::new(),
            };
        }

        let adapter = match ClaudeCodeAdapter::discover() {
            Ok(a) => a,
            Err(e) => {
                let msg =
                    format!("{command}: {e}\nInstall Claude Code and ensure `claude` is on PATH.");
                self.io.write_error(&msg);
                self.last_exit_status = Some(127);
                return DispatchResult::Executed {
                    output: msg,
                    exit_code: 127,
                    artifacts: Vec::new(),
                };
            }
        };

        let config = LaunchConfig {
            workspace_root: self.workspace.root().to_path_buf(),
            session_id: 0,
            env_overrides: Vec::new(),
        };

        match adapter.launch(config) {
            Ok(mut handle) => {
                handle.drain_pending();
                let pid = handle
                    .events()
                    .iter()
                    .find_map(|e| {
                        if let SupervisionEvent::AgentStarted { pid } = e {
                            Some(*pid)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);

                let msg = format!("[claude-code · pid {pid}] starting inside ROY shell\u{2026}");
                self.io.write_line(&msg);
                self.last_exit_status = Some(0);
                self.agent_handle = Some(handle);
                DispatchResult::AgentStarted {
                    agent_id: "claude-code".to_string(),
                    pid,
                }
            }
            Err(e) => {
                let msg = format!("{command}: failed to launch — {e}");
                self.io.write_error(&msg);
                self.last_exit_status = Some(1);
                DispatchResult::Executed {
                    output: msg,
                    exit_code: 1,
                    artifacts: Vec::new(),
                }
            }
        }
    }

    /// Drain new supervision events from the running agent.
    ///
    /// Returns `(events, exited)`. When `exited` is true the agent has
    /// terminated and the handle has been cleared. Safe to call every tick
    /// even when no agent is running.
    pub fn poll_agent_events(&mut self) -> (Vec<SupervisionEvent>, bool) {
        let Some(handle) = &mut self.agent_handle else {
            return (Vec::new(), false);
        };
        handle.drain_pending();
        let events = handle.take_events();
        let exited = handle.has_exited();

        if exited {
            self.agent_handle = None;
        }
        (events, exited)
    }

    /// Drain new events from the running agent into output lines.
    ///
    /// Legacy line view kept for tests and plain output paths.
    pub fn poll_agent_lines(&mut self) -> (Vec<String>, bool) {
        let (events, exited) = self.poll_agent_events();
        let mut lines = Vec::new();

        for event in events {
            match event {
                SupervisionEvent::OutputLine { text } | SupervisionEvent::ErrorLine { text } => {
                    lines.push(text);
                }
                SupervisionEvent::OutputChunk { bytes, .. } => {
                    let text = String::from_utf8_lossy(&bytes);
                    lines.extend(text.split('\n').map(str::to_string));
                }
                SupervisionEvent::ProcessExited { code } => {
                    lines.push(format!("[claude-code exited · code {code}]"));
                }
                SupervisionEvent::CommandAttempt { command, args } => {
                    let arg_str = if args.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", args.join(" "))
                    };
                    lines.push(format!("[attempt] {command}{arg_str}"));
                }
                SupervisionEvent::AgentStarted { .. } => {}
            }
        }

        (lines, exited)
    }

    /// Forward one submitted line to the active embedded agent (appends \r\n).
    pub fn send_agent_input(&mut self, line: &str) -> Result<(), String> {
        self.send_agent_raw(format!("{line}\r").as_bytes())
    }

    /// Forward raw PTY bytes to the active embedded agent without modification.
    ///
    /// Used for keystroke passthrough: arrow keys, ctrl chords, escape sequences.
    pub fn send_agent_raw(&mut self, bytes: &[u8]) -> Result<(), String> {
        let Some(handle) = &self.agent_handle else {
            return Err("no embedded agent is active".to_string());
        };
        handle.send_raw_bytes(bytes).map_err(|e| e.to_string())
    }
}
