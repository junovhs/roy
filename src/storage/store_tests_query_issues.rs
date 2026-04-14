//! Tests for issue query APIs.

use super::{make_store, seed_session, IssueQuery, IssueRecord};

fn insert_issue(store: &super::RoyStore, sid: u64, kind: &str, command: Option<&str>, ts: u64) -> i64 {
    store
        .insert_issue(
            sid,
            IssueRecord {
                kind,
                message: "test message",
                command,
                ts,
            },
        )
        .unwrap()
}

// ── query_issues_by ───────────────────────────────────────────────────────────

#[test]
fn query_issues_no_filter_returns_all() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_issue(&store, sid, "parse_error", Some("ls"), 10);
    insert_issue(&store, sid, "not_found", None, 20);
    insert_issue(&store, sid, "parse_error", Some("cd"), 30);

    let rows = store
        .query_issues_by(sid, &IssueQuery::default())
        .unwrap();
    assert_eq!(rows.len(), 3);
}

#[test]
fn query_issues_filter_by_kind() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_issue(&store, sid, "parse_error", None, 10);
    insert_issue(&store, sid, "not_found", None, 20);
    insert_issue(&store, sid, "parse_error", None, 30);

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                kind: Some("parse_error"),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.kind == "parse_error"));
}

#[test]
fn query_issues_filter_by_command() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_issue(&store, sid, "parse_error", Some("ls"), 10);
    insert_issue(&store, sid, "parse_error", Some("cd"), 20);
    insert_issue(&store, sid, "not_found", Some("ls"), 30);

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                command: Some("ls"),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.command == Some("ls".to_string())));
}

#[test]
fn query_issues_open_only_excludes_resolved() {
    let store = make_store();
    let sid = seed_session(&store);
    let id1 = insert_issue(&store, sid, "parse_error", None, 10);
    insert_issue(&store, sid, "not_found", None, 20);
    store.resolve_issue(id1, 99).unwrap();

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                open_only: true,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].resolved_at.is_none());
}

#[test]
fn query_issues_not_open_only_includes_resolved() {
    let store = make_store();
    let sid = seed_session(&store);
    let id = insert_issue(&store, sid, "parse_error", None, 10);
    store.resolve_issue(id, 99).unwrap();

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                open_only: false,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].resolved_at, Some(99));
}

#[test]
fn query_issues_filter_since() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_issue(&store, sid, "parse_error", None, 5);
    insert_issue(&store, sid, "parse_error", None, 15);
    insert_issue(&store, sid, "parse_error", None, 25);

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                since: Some(15),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.ts >= 15));
}

#[test]
fn query_issues_filter_until() {
    let store = make_store();
    let sid = seed_session(&store);
    insert_issue(&store, sid, "parse_error", None, 10);
    insert_issue(&store, sid, "parse_error", None, 20);
    insert_issue(&store, sid, "parse_error", None, 30);

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                until: Some(20),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.ts <= 20));
}

#[test]
fn query_issues_combined_kind_and_open_only() {
    let store = make_store();
    let sid = seed_session(&store);
    let id1 = insert_issue(&store, sid, "parse_error", None, 10);
    insert_issue(&store, sid, "parse_error", None, 20);
    insert_issue(&store, sid, "not_found", None, 30);
    store.resolve_issue(id1, 99).unwrap();

    let rows = store
        .query_issues_by(
            sid,
            &IssueQuery {
                kind: Some("parse_error"),
                open_only: true,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 1, "one open parse_error must remain");
    assert_eq!(rows[0].kind, "parse_error");
    assert!(rows[0].resolved_at.is_none());
}

#[test]
fn query_issues_pagination() {
    let store = make_store();
    let sid = seed_session(&store);
    for ts in 0..6_u64 {
        insert_issue(&store, sid, "parse_error", None, ts);
    }

    let page1 = store
        .query_issues_by(
            sid,
            &IssueQuery {
                limit: Some(2),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page1.len(), 2);

    let page2 = store
        .query_issues_by(
            sid,
            &IssueQuery {
                limit: Some(2),
                offset: Some(2),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(page2.len(), 2);
    assert_ne!(page1[0].id, page2[0].id);
}

#[test]
fn query_issues_empty_session() {
    let store = make_store();
    let sid = seed_session(&store);
    let rows = store
        .query_issues_by(sid, &IssueQuery::default())
        .unwrap();
    assert!(rows.is_empty());
}
