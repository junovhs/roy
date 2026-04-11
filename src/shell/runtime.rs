// API is live in tests and wired into Cockpit prompt; full wiring pending SHEL-02.
#![allow(dead_code)]

use std::path::PathBuf;

use super::result::DispatchResult;
use super::traps::COMPAT_TRAPS;
use super::{BufferedIo, ShellEnv, ShellError, ShellIo};

/// ROY shell runtime.
///
/// Owns the controlled shell environment, a session transcript buffer,
/// and the command dispatch table. TOOL-01 will wire the command registry
/// between the built-ins and compatibility traps.
///
/// The `io` field is a [`BufferedIo`] that accumulates all output lines
/// for the session transcript. The UI drains it via [`drain_output`] /
/// [`drain_errors`] to update its display.
pub struct ShellRuntime {
    env: ShellEnv,
    io: BufferedIo,
    last_exit_status: Option<i32>,
}

impl ShellRuntime {
    /// Create a new runtime rooted at `workspace_root`.
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            env: ShellEnv::new(workspace_root),
            io: BufferedIo::new(),
            last_exit_status: None,
        }
    }

    /// Shared reference to the shell environment.
    pub fn env(&self) -> &ShellEnv {
        &self.env
    }

    /// Mutable reference to the shell environment.
    pub fn env_mut(&mut self) -> &mut ShellEnv {
        &mut self.env
    }

    /// Exit code of the most recently dispatched command.
    pub fn last_exit_status(&self) -> Option<i32> {
        self.last_exit_status
    }

    /// Record an exit status (used by command executors and tests).
    pub fn set_exit_status(&mut self, code: i32) {
        self.last_exit_status = Some(code);
    }

    /// Prompt string reflecting current session state.
    ///
    /// Shows `✗` after a non-zero exit, `❯` otherwise.
    pub fn prompt(&self) -> String {
        let indicator = match self.last_exit_status {
            Some(0) | None => "\u{276f}",
            Some(_) => "\u{2717}",
        };
        format!("roy:{} {} ", self.env.cwd().display(), indicator)
    }

    /// Drain accumulated output lines since the last drain.
    ///
    /// Called by the UI to refresh the shell pane display.
    pub fn drain_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.io.output)
    }

    /// Drain accumulated error lines since the last drain.
    pub fn drain_errors(&mut self) -> Vec<String> {
        std::mem::take(&mut self.io.errors)
    }

    /// Dispatch a command through ROY's resolution layers.
    ///
    /// Resolution order:
    /// 1. ROY built-ins: `cd`, `pwd`, `env`/`printenv`, `exit`/`quit`, `help`/`roy`
    /// 2. *(TOOL-01 inserts the ROY-native command registry here)*
    /// 3. Compatibility traps → `Denied` with a helpful suggestion
    /// 4. Everything else → `NotFound`
    ///
    /// Output is written to the internal [`BufferedIo`] transcript AND
    /// returned in the [`DispatchResult`] for immediate UI rendering.
    pub fn dispatch(&mut self, command: &str, args: &[&str]) -> DispatchResult {
        match command {
            "cd"                 => self.dispatch_cd(args),
            "pwd"                => self.dispatch_pwd(),
            "env" | "printenv"   => self.dispatch_env(args),
            "exit" | "quit"      => self.dispatch_exit(args),
            "help" | "roy" | "?" => self.dispatch_help(),
            _ => {
                if let Some(&(_, msg)) = COMPAT_TRAPS.iter().find(|&&(n, _)| n == command) {
                    self.io.write_error(msg);
                    return DispatchResult::Denied {
                        command: command.to_string(),
                        suggestion: Some(msg.to_string()),
                    };
                }
                let msg = format!("roy: {command}: command not found");
                self.io.write_error(&msg);
                DispatchResult::NotFound { command: command.to_string() }
            }
        }
    }

    // ── built-in handlers ────────────────────────────────────────────────────

    fn dispatch_cd(&mut self, args: &[&str]) -> DispatchResult {
        let Some(&raw) = args.first() else {
            return DispatchResult::CwdChanged { to: self.env.cwd().to_path_buf() };
        };

        match self.env.chdir(std::path::Path::new(raw)) {
            Ok(()) => {
                self.last_exit_status = Some(0);
                DispatchResult::CwdChanged { to: self.env.cwd().to_path_buf() }
            }
            Err(ShellError::DirNotFound(p)) => {
                let msg = format!("cd: {}: no such directory", p.display());
                self.io.write_error(&msg);
                self.last_exit_status = Some(1);
                DispatchResult::Executed { output: msg, exit_code: 1 }
            }
            Err(ShellError::NotADirectory(p)) => {
                let msg = format!("cd: {}: not a directory", p.display());
                self.io.write_error(&msg);
                self.last_exit_status = Some(1);
                DispatchResult::Executed { output: msg, exit_code: 1 }
            }
            Err(e) => {
                let msg = format!("cd: {e}");
                self.io.write_error(&msg);
                self.last_exit_status = Some(1);
                DispatchResult::Executed { output: msg, exit_code: 1 }
            }
        }
    }

    fn dispatch_pwd(&mut self) -> DispatchResult {
        let output = self.env.cwd().display().to_string();
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed { output, exit_code: 0 }
    }

    fn dispatch_env(&mut self, args: &[&str]) -> DispatchResult {
        let snap = self.env.snapshot();
        let filter = args.first().copied();
        let mut lines: Vec<String> = snap
            .iter()
            .filter(|(k, _)| filter.is_none_or(|f| k.contains(f)))
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        lines.sort();
        let output = lines.join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed { output, exit_code: 0 }
    }

    fn dispatch_exit(&mut self, args: &[&str]) -> DispatchResult {
        let code: i32 = args.first().and_then(|s| s.parse().ok()).unwrap_or(0);
        self.last_exit_status = Some(code);
        DispatchResult::Exit { code }
    }

    fn dispatch_help(&mut self) -> DispatchResult {
        let output = [
            "ROY \u{2014} controlled shell host",
            "",
            "Built-in commands:",
            "  cd [path]    change working directory",
            "  pwd          print working directory",
            "  env [key]    print environment (filtered by key substring if given)",
            "  exit [n]     exit session with code n (default 0)",
            "  help         show this help",
            "",
            "ROY-native commands: pending TOOL-01",
            "Policy engine:       pending POL-01",
            "Embedded agents:     pending AGEN-01",
        ]
        .join("\n");
        self.io.write_line(&output);
        self.last_exit_status = Some(0);
        DispatchResult::Executed { output, exit_code: 0 }
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────
// Split across two sidecar files to keep each under the token advisory limit.

#[cfg(test)]
#[path = "runtime_tests_builtins.rs"]
mod tests_builtins;

#[cfg(test)]
#[path = "runtime_tests_policy.rs"]
mod tests_policy;
