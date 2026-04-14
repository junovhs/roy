use crate::render::CommandResult;
use crate::session::SessionArtifact;

/// Outcome of dispatching a command through the shell runtime.
#[derive(Debug, PartialEq)]
pub enum DispatchResult {
    /// Command executed and produced output with an exit code.
    Executed {
        output: String,
        exit_code: i32,
        artifacts: Vec<SessionArtifact>,
    },
    /// Command executed and produced a typed, renderable result (REND-01).
    ///
    /// Builtins and ROY-native commands migrate to this variant progressively.
    /// Callers render it with [`crate::render::human::render`] or
    /// [`crate::render::json::render`] depending on the plan's render mode.
    Typed { result: CommandResult },
    /// Command is known but blocked — ROY policy or compatibility trap.
    Denied {
        command: String,
        suggestion: Option<String>,
        artifacts: Vec<SessionArtifact>,
    },
    /// Command is not in the ROY registry.
    NotFound { command: String },
    /// Working directory changed successfully.
    CwdChanged { to: std::path::PathBuf },
    /// Session exit requested.
    Exit { code: i32 },
    /// An embedded agent was launched successfully.
    AgentStarted { agent_id: String, pid: u32 },
}
