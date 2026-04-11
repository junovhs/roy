// Live in tests and wired via Session::push; UI wiring pending SES-02.
#![allow(dead_code)]

use std::path::PathBuf;

/// Milliseconds since UNIX epoch — simple, copy-friendly, SQLite-friendly.
pub type Timestamp = u64;

/// All typed events that can occur in a ROY shell session.
///
/// Every variant carries a `ts` (timestamp) so the ledger is replayable
/// in chronological order without a separate timestamp column.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionEvent {
    /// The embedded agent or user submitted a line of input.
    UserInput { text: String, ts: Timestamp },
    /// Output text produced by an embedded agent.
    AgentOutput { text: String, ts: Timestamp },
    /// A command was dispatched through the ROY runtime.
    CommandInvoked { command: String, args: Vec<String>, ts: Timestamp },
    /// A command was blocked by the compat-trap or policy layer.
    CommandDenied { command: String, suggestion: Option<String>, ts: Timestamp },
    /// A command was not found in the ROY registry.
    CommandNotFound { command: String, ts: Timestamp },
    /// The working directory changed.
    CwdChanged { to: PathBuf, ts: Timestamp },
    /// A significant output was promoted to an artifact.
    ArtifactCreated { name: String, kind: String, ts: Timestamp },
    /// A host-level lifecycle or informational notice.
    HostNotice { message: String, ts: Timestamp },
    /// Emitted once when a session opens.
    SessionStarted { ts: Timestamp },
    /// Emitted once when a session closes.
    SessionEnded { exit_code: i32, ts: Timestamp },
}

impl SessionEvent {
    /// Timestamp of this event.
    pub fn timestamp(&self) -> Timestamp {
        match self {
            Self::UserInput      { ts, .. }
            | Self::AgentOutput      { ts, .. }
            | Self::CommandInvoked   { ts, .. }
            | Self::CommandDenied    { ts, .. }
            | Self::CommandNotFound  { ts, .. }
            | Self::CwdChanged       { ts, .. }
            | Self::ArtifactCreated  { ts, .. }
            | Self::HostNotice       { ts, .. }
            | Self::SessionStarted   { ts }
            | Self::SessionEnded     { ts, .. } => *ts,
        }
    }

    /// Short identifier string for this event variant — useful for filtering
    /// and diagnostics display without pattern-matching on the full enum.
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::UserInput { .. }      => "user_input",
            Self::AgentOutput { .. }    => "agent_output",
            Self::CommandInvoked { .. } => "command_invoked",
            Self::CommandDenied { .. }  => "command_denied",
            Self::CommandNotFound { .. }=> "command_not_found",
            Self::CwdChanged { .. }     => "cwd_changed",
            Self::ArtifactCreated { .. }=> "artifact_created",
            Self::HostNotice { .. }     => "host_notice",
            Self::SessionStarted { .. } => "session_started",
            Self::SessionEnded { .. }   => "session_ended",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_input(ts: Timestamp) -> SessionEvent {
        SessionEvent::UserInput { text: "ls".to_string(), ts }
    }

    fn cmd_denied(ts: Timestamp) -> SessionEvent {
        SessionEvent::CommandDenied {
            command: "bash".to_string(),
            suggestion: Some("use ROY-native commands".to_string()),
            ts,
        }
    }

    #[test]
    fn timestamp_round_trips_user_input() {
        assert_eq!(user_input(42).timestamp(), 42);
    }

    #[test]
    fn timestamp_round_trips_session_started() {
        assert_eq!(SessionEvent::SessionStarted { ts: 99 }.timestamp(), 99);
    }

    #[test]
    fn timestamp_round_trips_session_ended() {
        let ev = SessionEvent::SessionEnded { exit_code: 0, ts: 7 };
        assert_eq!(ev.timestamp(), 7);
    }

    #[test]
    fn kind_str_user_input() {
        assert_eq!(user_input(0).kind_str(), "user_input");
    }

    #[test]
    fn kind_str_command_denied() {
        assert_eq!(cmd_denied(0).kind_str(), "command_denied");
    }

    #[test]
    fn kind_str_command_not_found() {
        let ev = SessionEvent::CommandNotFound { command: "xyz".to_string(), ts: 0 };
        assert_eq!(ev.kind_str(), "command_not_found");
    }

    #[test]
    fn kind_str_cwd_changed() {
        let ev = SessionEvent::CwdChanged { to: PathBuf::from("/tmp"), ts: 0 };
        assert_eq!(ev.kind_str(), "cwd_changed");
    }

    #[test]
    fn kind_str_artifact_created() {
        let ev = SessionEvent::ArtifactCreated {
            name: "patch.diff".to_string(),
            kind: "diff".to_string(),
            ts: 0,
        };
        assert_eq!(ev.kind_str(), "artifact_created");
    }

    #[test]
    fn session_event_is_clone_and_partial_eq() {
        let ev = user_input(1);
        assert_eq!(ev.clone(), ev);
    }
}
