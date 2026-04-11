// API is live in tests and wired into Cockpit prompt; full wiring pending SHEL-02.
#![allow(dead_code)]

#[path = "runtime_builtins.rs"]
mod builtins;
#[path = "runtime_native.rs"]
mod native;

use std::path::PathBuf;

use crate::capabilities::CapabilityRuntime;
use crate::commands::schema::Backend;
use crate::commands::{parse_native_request, CommandRegistry};
use crate::policy::{PolicyEngine, PolicyOutcome};
use crate::session::SessionArtifact;
use crate::workspace::WorkspaceBoundary;

use super::result::DispatchResult;
use super::{BufferedIo, ShellEnv, ShellIo};

/// ROY shell runtime.
///
/// Owns the controlled shell environment, a session transcript buffer,
/// and the command registry. Dispatches commands through three layers:
/// 1. Command resolution through the registry
/// 2. Policy evaluation for known commands
/// 3. Built-in handlers or registry-backed denials
///
/// The `io` field is a [`BufferedIo`] that accumulates all output lines
/// for the session transcript. The UI drains it via [`drain_output`] /
/// [`drain_errors`] to update its display.
pub struct ShellRuntime {
    env: ShellEnv,
    io: BufferedIo,
    last_exit_status: Option<i32>,
    registry: CommandRegistry,
    policy: PolicyEngine,
    workspace: WorkspaceBoundary,
}

impl ShellRuntime {
    /// Create a new runtime rooted at `workspace_root`.
    ///
    /// Uses the permissive policy profile by default — policy is in-path
    /// but transparent until explicitly configured (see `set_policy`).
    pub fn new(workspace_root: PathBuf) -> Self {
        let workspace_root = workspace_root.canonicalize().unwrap_or(workspace_root);
        let workspace = WorkspaceBoundary::new(workspace_root.clone());

        Self {
            env: ShellEnv::new(workspace_root),
            io: BufferedIo::new(),
            last_exit_status: None,
            registry: CommandRegistry::new(),
            policy: PolicyEngine::default(),
            workspace,
        }
    }

    /// Workspace root for this runtime session.
    pub fn workspace_root(&self) -> &std::path::Path {
        self.workspace.root()
    }

    /// Name of the active policy profile.
    pub fn policy_name(&self) -> &str {
        self.policy.profile_name()
    }

    /// Shared reference to the command registry — used by diagnostics.
    pub fn registry(&self) -> &CommandRegistry {
        &self.registry
    }

    /// Total number of commands known to the registry.
    pub fn command_count(&self) -> usize {
        self.registry.len()
    }

    /// Number of commands shown in public help listings.
    pub fn public_command_count(&self) -> usize {
        self.registry.public_commands().len()
    }

    /// Replace the active policy profile.
    pub fn set_policy(&mut self, engine: PolicyEngine) {
        self.policy = engine;
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

    fn prompt_indicator(&self) -> &'static str {
        match self.last_exit_status {
            Some(0) | None => "\u{276f}",
            Some(_) => "\u{2717}",
        }
    }

    /// Prompt string reflecting current session state.
    ///
    /// Shows `✗` after a non-zero exit, `❯` otherwise.
    pub fn prompt(&self) -> String {
        self.io.prompt_str(&self.env, self.prompt_indicator())
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
    /// 1. Registry lookup — unknown commands immediately become `NotFound`
    /// 2. Policy gate for known commands
    /// 3. Built-ins or registry-backed deny/pending behavior
    ///
    /// Output is written to the internal [`BufferedIo`] transcript AND
    /// returned in the [`DispatchResult`] for immediate UI rendering.
    pub fn dispatch(&mut self, command: &str, args: &[&str]) -> DispatchResult {
        let Some(schema) = self.registry.resolve(command) else {
            return self.not_found(command);
        };

        match self.policy.evaluate(command, schema.risk_level) {
            PolicyOutcome::Deny { reason } => return self.deny(command, args, reason),
            PolicyOutcome::ApprovalPending { reason, .. } => {
                return self.deny(command, args, reason)
            }
            PolicyOutcome::Allow => {}
        }

        match schema.backend {
            Backend::Builtin => self.dispatch_builtin(command, args),
            Backend::CompatTrap { suggestion } => self.deny(command, args, suggestion),
            Backend::Blocked { reason } => self.deny(command, args, reason),
            Backend::RoyNative => self.dispatch_native(command, args),
        }
    }

    fn dispatch_builtin(&mut self, command: &str, args: &[&str]) -> DispatchResult {
        match command {
            "cd" => self.dispatch_cd(args),
            "pwd" => self.dispatch_pwd(),
            "env" | "printenv" => self.dispatch_env(args),
            "exit" | "quit" => self.dispatch_exit(args),
            "help" | "roy" | "?" => self.dispatch_help(),
            "commands" => self.dispatch_commands(),
            _ => self.not_found(command),
        }
    }

    fn deny(&mut self, command: &str, args: &[&str], message: impl Into<String>) -> DispatchResult {
        let message = message.into();
        self.io.write_error(&message);
        self.last_exit_status = Some(126);
        let artifact = SessionArtifact::denied_command(command, args, message.clone());
        DispatchResult::Denied {
            command: command.to_string(),
            suggestion: Some(message),
            artifacts: vec![artifact],
        }
    }

    fn not_found(&mut self, command: &str) -> DispatchResult {
        let msg =
            format!("roy: {command}: command not found — run `help` to see available commands");
        self.io.write_error(&msg);
        self.last_exit_status = Some(127);
        DispatchResult::NotFound {
            command: command.to_string(),
        }
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────
// Split across sidecar files to keep each under the token advisory limit.

#[cfg(test)]
#[path = "runtime_tests_builtins.rs"]
mod tests_builtins;

#[cfg(test)]
#[path = "runtime_tests_discoverability.rs"]
mod tests_discoverability;

#[cfg(test)]
#[path = "runtime_tests_policy.rs"]
mod tests_policy;

#[cfg(test)]
#[path = "runtime_tests_native.rs"]
mod tests_native;
