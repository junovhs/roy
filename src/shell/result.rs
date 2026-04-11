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
}
