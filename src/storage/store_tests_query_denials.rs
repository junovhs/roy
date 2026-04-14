//! Tests for denial query APIs.

use super::{make_store, seed_session, DenialQuery, DenialRecord};

fn insert_denial(store: &super::RoyStore, sid: u64, command: &str, ts: u64) {
    store
        .insert_denial(
            sid,
            DenialRecord {
                command,
                args: &[],
                reason: "blocked",
                suggestion: None,
                ts,
            },
        )
        .unwrap();
}

// ── query_denials_by ──────────────────────────────────────────────────────────

#[test]
fn query_denials_no_filter_returns_all() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_denial(&store, sid, "bash", 10);
    insert_denial(&store, sid, "curl", 20);
    insert_denial(&store, sid, "bash", 30);

    let rows = store
        .query_denials_by(sid, &DenialQuery::default())
        .unwrap();
    assert_eq!(rows.len(), 3);
}

#[test]
fn query_denials_filter_by_command() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_denial(&store, sid, "bash", 10);
    insert_denial(&store, sid, "curl", 20);
    insert_denial(&store, sid, "bash", 30);

    let rows = store
        .query_denials_by(
            sid,
            &DenialQuery {
                command: Some("bash"),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.command == "bash"));
}

#[test]
fn query_denials_unknown_command_returns_empty() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_denial(&store, sid, "bash", 10);

    let rows = store
        .query_denials_by(
            sid,
            &DenialQuery {
                command: Some("rm"),
                ..Default::default()
            },
        )
        .unwrap();
    assert!(rows.is_empty());
}

#[test]
fn query_denials_filter_since() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_denial(&store, sid, "bash", 5);
    insert_denial(&store, sid, "bash", 15);
    insert_denial(&store, sid, "bash", 25);

    let rows = store
        .query_denials_by(
            sid,
            &DenialQuery {
                since: Some(15),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.ts >= 15));
}

#[test]
fn query_denials_filter_until() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_denial(&store, sid, "bash", 10);
    insert_denial(&store, sid, "bash", 20);
    insert_denial(&store, sid, "bash", 30);

    let rows = store
        .query_denials_by(
            sid,
            &DenialQuery {
                until: Some(20),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.ts <= 20));
}

#[test]
fn query_denials_time_window() {
    let store = make_store();
    let sid = seed_session(&store);
    for ts in [10_u64, 20, 30, 40, 50] {
        insert_denial(&store, sid, "bash", ts);
    }

    let rows = store
        .query_denials_by(
            sid,
            &DenialQuery {
                since: Some(20),
                until: Some(40),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 3, "ts 20, 30, 40 must match");
}

#[test]
fn query_denials_pagination() {
    let store = make_store();
    let sid = seed_session(&store);
    for ts in 0..8_u64 {
        insert_denial(&store, sid, "bash", ts);
    }

    let page1 = store
        .query_denials_by(
            sid,
            &DenialQuery {
                limit: Some(3),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page1.len(), 3);

    let page2 = store
        .query_denials_by(
            sid,
            &DenialQuery {
                limit: Some(3),
                offset: Some(3),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page2.len(), 3);
    assert_ne!(page1[0].id, page2[0].id);
}

#[test]
fn list_denials_delegates_to_query_denials_by() {
    // Verify list_denials still works (it now calls query_denials_by internally).
    let store = make_store();
    let sid = seed_session(&store);
    insert_denial(&store, sid, "bash", 1);
    insert_denial(&store, sid, "curl", 2);

    let all = store.list_denials(sid).unwrap();
    assert_eq!(all.len(), 2);
}
