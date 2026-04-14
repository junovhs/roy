//! Tests for session query APIs.
//!
//! ROY session ids are ms-since-epoch, so `session.id == started_at` in the DB.
//! Tests use large, spread ids to produce distinct `started_at` values.

use std::path::PathBuf;

use super::{make_store, Session, SessionQuery};

/// Creates a session where `id == started_at` (ROY convention).
fn insert_session(store: &super::RoyStore, id: u64) {
    let s = Session::new(id, PathBuf::from("/tmp/ws"), 0);
    store.save_session(&s).unwrap();
}

// ── load_session ──────────────────────────────────────────────────────────────

#[test]
fn load_session_returns_saved_session() {
    let store = make_store();
    insert_session(&store, 1_000);

    let rec = store.load_session(1_000).unwrap().expect("session must exist");
    assert_eq!(rec.id, 1_000);
    assert_eq!(rec.workspace_root, "/tmp/ws");
    assert!(rec.ended_at.is_none());
    assert!(rec.exit_code.is_none());
}

#[test]
fn load_session_unknown_id_returns_none() {
    let store = make_store();
    assert!(store.load_session(999_999_999).unwrap().is_none());
}

#[test]
fn load_session_reflects_close() {
    let store = make_store();
    insert_session(&store, 2_000);
    // close_session(session_id, exit_code, ended_at)
    store.close_session(2_000, 0, 99_999).unwrap();

    let rec = store.load_session(2_000).unwrap().expect("session must exist");
    assert_eq!(rec.ended_at, Some(99_999));
    assert_eq!(rec.exit_code, Some(0));
}

// ── list_sessions ─────────────────────────────────────────────────────────────

#[test]
fn list_sessions_no_filter_returns_all_newest_first() {
    let store = make_store();
    insert_session(&store, 100);
    insert_session(&store, 200);
    insert_session(&store, 300);

    let rows = store.list_sessions(&SessionQuery::default()).unwrap();
    assert_eq!(rows.len(), 3);
    // Newest first (started_at DESC = id DESC).
    assert_eq!(rows[0].id, 300);
    assert_eq!(rows[1].id, 200);
    assert_eq!(rows[2].id, 100);
}

#[test]
fn list_sessions_filter_since() {
    let store = make_store();
    insert_session(&store, 100);
    insert_session(&store, 200);
    insert_session(&store, 300);

    let rows = store
        .list_sessions(&SessionQuery {
            since: Some(200),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.started_at >= 200));
}

#[test]
fn list_sessions_filter_until() {
    let store = make_store();
    insert_session(&store, 100);
    insert_session(&store, 200);
    insert_session(&store, 300);

    let rows = store
        .list_sessions(&SessionQuery {
            until: Some(200),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.started_at <= 200));
}

#[test]
fn list_sessions_time_window() {
    let store = make_store();
    for ts in [100_u64, 200, 300, 400, 500] {
        insert_session(&store, ts);
    }

    let rows = store
        .list_sessions(&SessionQuery {
            since: Some(200),
            until: Some(400),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(rows.len(), 3, "ts 200, 300, 400 must match");
}

#[test]
fn list_sessions_pagination() {
    let store = make_store();
    for ts in [100_u64, 200, 300, 400, 500, 600] {
        insert_session(&store, ts);
    }

    let page1 = store
        .list_sessions(&SessionQuery {
            limit: Some(2),
            offset: Some(0),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(page1.len(), 2);

    let page2 = store
        .list_sessions(&SessionQuery {
            limit: Some(2),
            offset: Some(2),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(page2.len(), 2);
    let ids1: Vec<u64> = page1.iter().map(|r| r.id).collect();
    let ids2: Vec<u64> = page2.iter().map(|r| r.id).collect();
    assert!(ids1.iter().all(|id| !ids2.contains(id)));
}

#[test]
fn list_sessions_empty_store() {
    let store = make_store();
    assert!(store.list_sessions(&SessionQuery::default()).unwrap().is_empty());
}
