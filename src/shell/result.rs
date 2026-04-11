// Live in tests and dispatch return types; binary wiring pending SHEL-02.
#![allow(dead_code)]

/// Outcome of dispatching a command through the shell runtime.
#[derive(Debug, PartialEq)]
pub enum DispatchResult {
    /// Command executed and produced output with an exit code.
    Executed { output: String, exit_code: i32 },
    /// Command is known but blocked — ROY policy or compatibility trap.
    Denied { command: String, suggestion: Option<String> },
    /// Command is not in the ROY registry.
    NotFound { command: String },
    /// Working directory changed successfully.
    CwdChanged { to: std::path::PathBuf },
    /// Session exit requested.
    Exit { code: i32 },
}
