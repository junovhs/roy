use super::{make_store, seed_session, IssueRecord};

#[test]
fn insert_and_list_issue() {
    let store = make_store();
    let sid = seed_session(&store);
    store
        .insert_issue(
            sid,
            IssueRecord {
                kind: "parse_error",
                message: "unexpected token",
                command: Some("ls"),
                ts: 50,
            },
        )
        .unwrap();

    let issues = store.list_issues(sid).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].kind, "parse_error");
    assert_eq!(issues[0].message, "unexpected token");
    assert_eq!(issues[0].command, Some("ls".to_string()));
    assert!(issues[0].resolved_at.is_none());
}

#[test]
fn resolve_issue_sets_resolved_at() {
    let store = make_store();
    let sid = seed_session(&store);
    let id = store
        .insert_issue(
            sid,
            IssueRecord {
                kind: "not_found",
                message: "bad cmd",
                command: None,
                ts: 1,
            },
        )
        .unwrap();

    store.resolve_issue(id, 999).unwrap();
    assert_eq!(store.list_issues(sid).unwrap()[0].resolved_at, Some(999));
}

#[test]
fn list_open_issues_excludes_resolved() {
    let store = make_store();
    let sid = seed_session(&store);
    let id = store
        .insert_issue(
            sid,
            IssueRecord {
                kind: "parse_error",
                message: "msg",
                command: None,
                ts: 1,
            },
        )
        .unwrap();
    store
        .insert_issue(
            sid,
            IssueRecord {
                kind: "not_found",
                message: "msg2",
                command: None,
                ts: 2,
            },
        )
        .unwrap();

    store.resolve_issue(id, 50).unwrap();
    let open = store.list_open_issues(sid).unwrap();
    assert_eq!(open.len(), 1, "only unresolved issues must appear");
    assert_eq!(open[0].kind, "not_found");
}
