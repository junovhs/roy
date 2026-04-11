//! Integration tests for RoyStore — save/load roundtrip through SQLite.

use std::path::PathBuf;
use crate::session::{ArtifactBody, ArtifactKind, Session, SessionArtifact, SessionEvent};
use super::RoyStore;

fn session_with_events() -> Session {
    let root = PathBuf::from("/tmp/ws");
    let mut s = Session::new(1000, root, 0);
    s.push(SessionEvent::UserInput { text: "pwd".to_string(), ts: 1 });
    s.push(SessionEvent::CommandInvoked {
        command: "pwd".to_string(),
        args: vec![],
        ts: 2,
    });
    s.push(SessionEvent::CommandOutput {
        text: "/tmp/ws".to_string(),
        is_error: false,
        ts: 3,
    });
    s.push(SessionEvent::CommandDenied {
        command: "bash".to_string(),
        suggestion: Some("ROY does not provide a bash surface.".to_string()),
        ts: 4,
    });
    s.push(SessionEvent::ArtifactCreated {
        artifact: SessionArtifact {
            name: "check".to_string(),
            kind: ArtifactKind::ValidationRun,
            summary: "cargo check passed".to_string(),
            body: ArtifactBody::ValidationRun {
                command: "cargo check --quiet --offline".to_string(),
                cwd: PathBuf::from("/tmp/ws"),
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
            },
        },
        ts: 5,
    });
    s
}

// ── open / schema ─────────────────────────────────────────────────────────────

#[test]
fn open_memory_succeeds() {
    RoyStore::open_memory().expect("in-memory store must open");
}

#[test]
fn schema_is_idempotent() {
    let store = RoyStore::open_memory().unwrap();
    // Applying migrations twice must not fail.
    drop(store);
    let store2 = RoyStore::open_memory().unwrap();
    drop(store2);
}

// ── save / load roundtrip ─────────────────────────────────────────────────────

#[test]
fn save_and_reload_session_events() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    store.save_session(&session).expect("save_session must succeed");

    let loaded = store.load_events(session.id).expect("load_events must succeed");
    assert_eq!(
        loaded.len(),
        session.events().len(),
        "loaded event count must match saved"
    );
}

#[test]
fn events_reload_in_chronological_order() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    store.save_session(&session).unwrap();
    let loaded = store.load_events(session.id).unwrap();
    let ts: Vec<u64> = loaded.iter().map(|e| e.timestamp()).collect();
    assert!(ts.windows(2).all(|w| w[0] <= w[1]), "events must be ordered: {ts:?}");
}

#[test]
fn user_input_event_roundtrips() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    store.save_session(&session).unwrap();
    let loaded = store.load_events(session.id).unwrap();
    assert!(
        loaded.iter().any(|e| matches!(e, SessionEvent::UserInput { .. })),
        "UserInput must survive roundtrip"
    );
}

#[test]
fn command_denied_event_preserves_suggestion() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    store.save_session(&session).unwrap();
    let loaded = store.load_events(session.id).unwrap();
    let denied = loaded.iter().find(|e| matches!(e, SessionEvent::CommandDenied { .. }));
    let Some(SessionEvent::CommandDenied { suggestion, .. }) = denied else {
        panic!("CommandDenied must survive roundtrip");
    };
    assert!(suggestion.is_some(), "suggestion must be preserved");
}

// ── append_event ──────────────────────────────────────────────────────────────

#[test]
fn append_event_adds_to_existing_session() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    let initial_count = session.events().len();
    store.save_session(&session).unwrap();

    let extra = SessionEvent::HostNotice { message: "append test".to_string(), ts: 99 };
    store.append_event(session.id, &extra).unwrap();

    let loaded = store.load_events(session.id).unwrap();
    assert_eq!(loaded.len(), initial_count + 1);
    assert!(
        matches!(loaded.last(), Some(SessionEvent::HostNotice { .. })),
        "appended event must appear last"
    );
}

#[test]
fn artifact_refs_roundtrip_through_artifacts_table() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    store.save_session(&session).unwrap();

    let refs = store.load_artifact_refs(session.id).unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].kind, "validation");
    assert!(refs[0].summary.contains("cargo check"));
}

#[test]
fn save_session_replaces_existing_event_and_artifact_rows() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();

    store.save_session(&session).unwrap();
    store.save_session(&session).unwrap();

    let loaded = store.load_events(session.id).unwrap();
    let refs = store.load_artifact_refs(session.id).unwrap();
    assert_eq!(loaded.len(), session.events().len(), "save_session must stay idempotent");
    assert_eq!(refs.len(), 1, "artifact refs must not duplicate");
}

// ── close_session ─────────────────────────────────────────────────────────────

#[test]
fn close_session_does_not_corrupt_events() {
    let store = RoyStore::open_memory().unwrap();
    let session = session_with_events();
    store.save_session(&session).unwrap();
    store.close_session(session.id, 0, 9999).unwrap();
    let loaded = store.load_events(session.id).unwrap();
    assert_eq!(loaded.len(), session.events().len());
}

// ── empty session ─────────────────────────────────────────────────────────────

#[test]
fn load_events_for_unknown_session_returns_empty() {
    let store = RoyStore::open_memory().unwrap();
    let loaded = store.load_events(99999).unwrap();
    assert!(loaded.is_empty());
}
