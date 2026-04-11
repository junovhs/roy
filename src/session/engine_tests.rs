use std::path::PathBuf;

use super::*;
use crate::session::{ArtifactBody, ArtifactKind};

fn tmp() -> PathBuf {
    std::env::temp_dir()
}

fn session() -> Session {
    Session::new(1, tmp(), 0)
}

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

#[test]
fn push_appends_event() {
    let mut s = session();
    s.push(SessionEvent::UserInput {
        text: "pwd".to_string(),
        ts: 1,
    });
    assert_eq!(s.len(), 2);
    assert_eq!(s.events()[1].kind_str(), "user_input");
}

#[test]
fn events_returns_ordered_slice() {
    let mut s = session();
    s.push(SessionEvent::UserInput {
        text: "a".to_string(),
        ts: 1,
    });
    s.push(SessionEvent::CommandOutput {
        text: "b".to_string(),
        is_error: false,
        ts: 2,
    });
    let kinds: Vec<&str> = s.events().iter().map(|e| e.kind_str()).collect();
    assert_eq!(kinds, &["session_started", "user_input", "command_output"]);
}

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
    s.push(SessionEvent::UserInput {
        text: "help".to_string(),
        ts: 3,
    });
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
    s.push(SessionEvent::HostNotice {
        message: "ready".to_string(),
        ts: 2,
    });

    let artifacts = s.artifacts();
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].name, "check");
}

#[test]
fn replay_iterates_all_events() {
    let mut s = session();
    s.push(SessionEvent::HostNotice {
        message: "ready".to_string(),
        ts: 5,
    });
    let replayed: Vec<&SessionEvent> = s.replay().collect();
    assert_eq!(replayed.len(), 2);
}

#[test]
fn replay_preserves_timestamp_order() {
    let mut s = session();
    s.push(SessionEvent::UserInput {
        text: "x".to_string(),
        ts: 10,
    });
    s.push(SessionEvent::AgentOutput {
        text: "y".to_string(),
        ts: 20,
    });
    let ts: Vec<u64> = s.replay().map(|e| e.timestamp()).collect();
    assert_eq!(ts, vec![0, 10, 20]);
}

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
