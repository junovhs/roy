//! Tests for artifact query APIs.

use crate::session::{ArtifactBody, ArtifactKind, SessionArtifact, SessionEvent};
use std::path::PathBuf;

use super::{make_store, seed_session, ArtifactQuery};

fn push_artifact(store: &super::RoyStore, session_id: u64, kind: &str, ts: u64) {
    let artifact = SessionArtifact {
        name: format!("art-{ts}"),
        kind: match kind {
            "diff" => ArtifactKind::Diff,
            _ => ArtifactKind::ValidationRun,
        },
        summary: format!("{kind} summary"),
        body: ArtifactBody::ValidationRun {
            command: "cargo check".into(),
            cwd: PathBuf::from("/tmp"),
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
        },
    };
    let event = SessionEvent::ArtifactCreated { artifact, ts };
    store.append_event(session_id, &event).unwrap();
}

// ── query_artifacts ───────────────────────────────────────────────────────────

#[test]
fn query_artifacts_no_filter_returns_all() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 10);
    push_artifact(&store, sid, "validation", 20);
    push_artifact(&store, sid, "diff", 30);

    let rows = store
        .query_artifacts(sid, &ArtifactQuery::default())
        .unwrap();
    assert_eq!(rows.len(), 3);
}

#[test]
fn query_artifacts_filter_by_kind() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 10);
    push_artifact(&store, sid, "diff", 20);
    push_artifact(&store, sid, "diff", 30);

    let rows = store
        .query_artifacts(
            sid,
            &ArtifactQuery {
                kind: Some("diff"),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.kind == "diff"));
}

#[test]
fn query_artifacts_filter_since() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 5);
    push_artifact(&store, sid, "validation", 15);
    push_artifact(&store, sid, "validation", 25);

    let rows = store
        .query_artifacts(
            sid,
            &ArtifactQuery {
                since: Some(15),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.created_at >= 15));
}

#[test]
fn query_artifacts_filter_until() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 10);
    push_artifact(&store, sid, "validation", 20);
    push_artifact(&store, sid, "validation", 30);

    let rows = store
        .query_artifacts(
            sid,
            &ArtifactQuery {
                until: Some(20),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.created_at <= 20));
}

#[test]
fn query_artifacts_filter_since_and_until() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 10);
    push_artifact(&store, sid, "validation", 20);
    push_artifact(&store, sid, "validation", 30);
    push_artifact(&store, sid, "validation", 40);

    let rows = store
        .query_artifacts(
            sid,
            &ArtifactQuery {
                since: Some(15),
                until: Some(35),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2, "ts 20 and 30 must match");
}

#[test]
fn query_artifacts_pagination_limit() {
    let store = make_store();
    let sid = seed_session(&store);
    for ts in 0..10_u64 {
        push_artifact(&store, sid, "validation", ts);
    }

    let page1 = store
        .query_artifacts(
            sid,
            &ArtifactQuery {
                limit: Some(3),
                offset: Some(0),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page1.len(), 3);

    let page2 = store
        .query_artifacts(
            sid,
            &ArtifactQuery {
                limit: Some(3),
                offset: Some(3),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page2.len(), 3);
    // Pages must not overlap.
    assert_ne!(page1[0].name, page2[0].name);
}

#[test]
fn query_artifacts_empty_session_returns_empty() {
    let store = make_store();
    let sid = seed_session(&store);
    let rows = store
        .query_artifacts(sid, &ArtifactQuery::default())
        .unwrap();
    assert!(rows.is_empty());
}

// ── count_artifacts ───────────────────────────────────────────────────────────

#[test]
fn count_artifacts_all_kinds() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 1);
    push_artifact(&store, sid, "diff", 2);
    push_artifact(&store, sid, "diff", 3);

    assert_eq!(store.count_artifacts(sid, None).unwrap(), 3);
}

#[test]
fn count_artifacts_by_kind() {
    let store = make_store();
    let sid = seed_session(&store);
    push_artifact(&store, sid, "validation", 1);
    push_artifact(&store, sid, "diff", 2);
    push_artifact(&store, sid, "diff", 3);

    assert_eq!(store.count_artifacts(sid, Some("diff")).unwrap(), 2);
    assert_eq!(store.count_artifacts(sid, Some("validation")).unwrap(), 1);
    assert_eq!(store.count_artifacts(sid, Some("unknown_kind")).unwrap(), 0);
}

#[test]
fn count_artifacts_empty_session() {
    let store = make_store();
    let sid = seed_session(&store);
    assert_eq!(store.count_artifacts(sid, None).unwrap(), 0);
}

#[test]
fn count_artifacts_scoped_to_session() {
    let store = make_store();
    let sid1 = seed_session(&store);
    push_artifact(&store, sid1, "diff", 1);
    push_artifact(&store, sid1, "diff", 2);

    // Create second session
    let s2 = crate::session::Session::new(9999, std::path::PathBuf::from("/other"), 0);
    store.save_session(&s2).unwrap();
    push_artifact(&store, s2.id, "diff", 3);

    assert_eq!(store.count_artifacts(sid1, None).unwrap(), 2, "session 1 only");
    assert_eq!(store.count_artifacts(s2.id, None).unwrap(), 1, "session 2 only");
}
