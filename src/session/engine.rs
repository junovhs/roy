// Live in tests; UI wiring pending SES-02.
#![allow(dead_code)]

use std::path::PathBuf;

use super::artifacts::SessionArtifact;
use super::events::{SessionEvent, Timestamp};

/// A ROY shell session — owns the ordered event ledger.
///
/// A `Session` is created when a shell pane opens and accumulates every
/// typed event that occurs: user inputs, command invocations, policy
/// outcomes, agent outputs, and host notices. It is the durable
/// structure underneath the terminal transcript.
pub struct Session {
    /// Unique identifier for this session (ms since UNIX epoch at creation).
    pub id: u64,
    /// Workspace root at session start.
    pub workspace_root: PathBuf,
    /// Ordered event ledger.
    events: Vec<SessionEvent>,
}

impl Session {
    /// Open a new session rooted at `workspace_root`.
    ///
    /// Automatically pushes a [`SessionEvent::SessionStarted`] event so
    /// the ledger always begins with a known anchor point.
    pub fn new(id: u64, workspace_root: PathBuf, now: Timestamp) -> Self {
        let mut s = Self { id, workspace_root, events: Vec::new() };
        s.events.push(SessionEvent::SessionStarted { ts: now });
        s
    }

    /// Append an event to the ledger.
    pub fn push(&mut self, event: SessionEvent) {
        self.events.push(event);
    }

    /// Ordered slice of all events since the session started.
    pub fn events(&self) -> &[SessionEvent] {
        &self.events
    }

    /// All events whose `kind_str()` matches `kind`.
    ///
    /// Useful for filtering: `session.events_of_kind("command_denied")`.
    pub fn events_of_kind<'a>(&'a self, kind: &str) -> Vec<&'a SessionEvent> {
        self.events.iter().filter(|e| e.kind_str() == kind).collect()
    }

    /// All promoted artifacts in chronological order.
    pub fn artifacts(&self) -> Vec<&SessionArtifact> {
        self.events
            .iter()
            .filter_map(|event| match event {
                SessionEvent::ArtifactCreated { artifact, .. } => Some(artifact),
                _ => None,
            })
            .collect()
    }

    /// Iterator over all events in chronological order — for replay.
    pub fn replay(&self) -> impl Iterator<Item = &SessionEvent> {
        self.events.iter()
    }

    /// Total number of events including the initial `SessionStarted`.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// True only when no events exist (should not happen after `new`).
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Close the session, appending a [`SessionEvent::SessionEnded`].
    pub fn end(&mut self, exit_code: i32, now: Timestamp) {
        self.events.push(SessionEvent::SessionEnded { exit_code, ts: now });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::session::{ArtifactBody, ArtifactKind};

    fn tmp() -> PathBuf { std::env::temp_dir() }

    fn session() -> Session {
        Session::new(1, tmp(), 0)
    }

    // ── construction ─────────────────────────────────────────────────────────

    #[test]
    fn new_session_has_started_event() {
        let s = session();
        assert_eq!(s.events()[0].kind_str(), "session_started");
    }

    #[test]
    fn new_session_len_is_one() {
        assert_eq!(session().len(), 1);
    }

    #[test]
    fn new_session_is_not_empty() {
        assert!(!session().is_empty());
    }

    // ── push ─────────────────────────────────────────────────────────────────

    #[test]
    fn push_appends_event() {
        let mut s = session();
        s.push(SessionEvent::UserInput { text: "pwd".to_string(), ts: 1 });
        assert_eq!(s.len(), 2);
        assert_eq!(s.events()[1].kind_str(), "user_input");
    }

    #[test]
    fn events_returns_ordered_slice() {
        let mut s = session();
        s.push(SessionEvent::UserInput { text: "a".to_string(), ts: 1 });
        s.push(SessionEvent::CommandOutput { text: "b".to_string(), is_error: false, ts: 2 });
        let kinds: Vec<&str> = s.events().iter().map(|e| e.kind_str()).collect();
        assert_eq!(kinds, &["session_started", "user_input", "command_output"]);
    }

    // ── filter ────────────────────────────────────────────────────────────────

    #[test]
    fn events_of_kind_returns_matching() {
        let mut s = session();
        s.push(SessionEvent::CommandDenied {
            command: "bash".to_string(),
            suggestion: None,
            ts: 1,
        });
        s.push(SessionEvent::CommandDenied {
            command: "grep".to_string(),
            suggestion: None,
            ts: 2,
        });
        s.push(SessionEvent::UserInput { text: "help".to_string(), ts: 3 });
        let denied = s.events_of_kind("command_denied");
        assert_eq!(denied.len(), 2);
    }

    #[test]
    fn events_of_kind_returns_empty_for_no_match() {
        let s = session();
        assert!(s.events_of_kind("artifact_created").is_empty());
    }

    #[test]
    fn artifacts_returns_promoted_items_only() {
        let mut s = session();
        s.push(SessionEvent::ArtifactCreated {
            artifact: SessionArtifact {
                name: "check".to_string(),
                kind: ArtifactKind::ValidationRun,
                summary: "cargo check passed".to_string(),
                body: ArtifactBody::Note {
                    text: "ok".to_string(),
                },
            },
            ts: 1,
        });
        s.push(SessionEvent::HostNotice { message: "ready".to_string(), ts: 2 });

        let artifacts = s.artifacts();
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].name, "check");
    }

    // ── replay ───────────────────────────────────────────────────────────────

    #[test]
    fn replay_iterates_all_events() {
        let mut s = session();
        s.push(SessionEvent::HostNotice { message: "ready".to_string(), ts: 5 });
        let replayed: Vec<&SessionEvent> = s.replay().collect();
        assert_eq!(replayed.len(), 2);
    }

    #[test]
    fn replay_preserves_timestamp_order() {
        let mut s = session(); // ts=0
        s.push(SessionEvent::UserInput { text: "x".to_string(), ts: 10 });
        s.push(SessionEvent::AgentOutput { text: "y".to_string(), ts: 20 });
        let ts: Vec<u64> = s.replay().map(|e| e.timestamp()).collect();
        assert_eq!(ts, vec![0, 10, 20]);
    }

    // ── end ───────────────────────────────────────────────────────────────────

    #[test]
    fn end_appends_session_ended_event() {
        let mut s = session();
        s.end(0, 100);
        let last = s.events().last().unwrap();
        assert_eq!(last.kind_str(), "session_ended");
    }

    #[test]
    fn end_records_exit_code() {
        let mut s = session();
        s.end(42, 200);
        match s.events().last().unwrap() {
            SessionEvent::SessionEnded { exit_code, .. } => assert_eq!(*exit_code, 42),
            other => panic!("expected SessionEnded, got {other:?}"),
        }
    }
}
