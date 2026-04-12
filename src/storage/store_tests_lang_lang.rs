use super::{
    make_file_store, make_store, params, remove_db, seed_session, ApprovalRecord, DenialRecord,
    IssueRecord, NamedRefRecord,
};

#[test]
fn insert_and_list_denial() {
    let store = make_store();
    let sid = seed_session(&store);
    store
        .insert_denial(
            sid,
            DenialRecord {
                command: "bash",
                args: &["-c", "rm -rf /"],
                reason: "ROY does not provide a bash surface",
                suggestion: Some("use built-in commands instead"),
                ts: 77,
            },
        )
        .unwrap();

    let denials = store.list_denials(sid).unwrap();
    assert_eq!(denials.len(), 1);
    assert_eq!(denials[0].command, "bash");
    assert_eq!(denials[0].args, vec!["-c", "rm -rf /"]);
    assert_eq!(
        denials[0].suggestion.as_deref(),
        Some("use built-in commands instead")
    );
    assert!(denials[0].redirect.is_none());
    assert_eq!(denials[0].ts, 77);
}

#[test]
fn denial_without_suggestion() {
    let store = make_store();
    let sid = seed_session(&store);
    store
        .insert_denial(
            sid,
            DenialRecord {
                command: "rm",
                args: &[],
                reason: "blocked",
                suggestion: None,
                ts: 5,
            },
        )
        .unwrap();
    assert!(store.list_denials(sid).unwrap()[0].suggestion.is_none());
}

#[test]
fn insert_and_list_pending_approval() {
    let store = make_store();
    let sid = seed_session(&store);
    store
        .insert_pending_approval(
            sid,
            ApprovalRecord {
                command: "deploy",
                args: &["prod"],
                reason: "high-risk change",
                requested_at: 1000,
            },
        )
        .unwrap();

    let pending = store.list_pending_approvals(sid).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].command, "deploy");
    assert_eq!(pending[0].args, vec!["prod"]);
    assert_eq!(pending[0].reason, "high-risk change");
    assert!(pending[0].resolved_at.is_none());
    assert!(pending[0].resolution.is_none());
}

#[test]
fn resolve_approval_sets_outcome() {
    let store = make_store();
    let sid = seed_session(&store);
    let id = store
        .insert_pending_approval(
            sid,
            ApprovalRecord {
                command: "deploy",
                args: &[],
                reason: "r",
                requested_at: 1,
            },
        )
        .unwrap();

    store.resolve_approval(id, 2000, "approved").unwrap();
    let approvals = store.list_all_approvals(sid).unwrap();
    assert_eq!(approvals[0].resolved_at, Some(2000));
    assert_eq!(approvals[0].resolution.as_deref(), Some("approved"));
}

#[test]
fn list_pending_approvals_excludes_resolved() {
    let store = make_store();
    let sid = seed_session(&store);
    let id = store
        .insert_pending_approval(
            sid,
            ApprovalRecord {
                command: "a",
                args: &[],
                reason: "r",
                requested_at: 1,
            },
        )
        .unwrap();
    store
        .insert_pending_approval(
            sid,
            ApprovalRecord {
                command: "b",
                args: &[],
                reason: "r2",
                requested_at: 2,
            },
        )
        .unwrap();

    store.resolve_approval(id, 3, "denied").unwrap();
    let pending = store.list_pending_approvals(sid).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].command, "b");
}

#[test]
fn list_all_approvals_includes_resolved() {
    let store = make_store();
    let sid = seed_session(&store);
    let id = store
        .insert_pending_approval(
            sid,
            ApprovalRecord {
                command: "a",
                args: &[],
                reason: "r",
                requested_at: 1,
            },
        )
        .unwrap();
    store.resolve_approval(id, 2, "approved").unwrap();
    assert_eq!(store.list_all_approvals(sid).unwrap().len(), 1);
}

#[test]
fn reopened_store_reconstructs_language_state_for_saved_session() {
    let (path, store) = make_file_store("language-reopen");
    let sid = seed_session(&store);

    store
        .upsert_named_ref(
            sid,
            NamedRefRecord {
                name: "last",
                kind: "artifact",
                target_id: "art:7",
                created_at: 10,
            },
        )
        .unwrap();
    let issue_id = store
        .insert_issue(
            sid,
            IssueRecord {
                kind: "parse_error",
                message: "unexpected token",
                command: Some("show ref"),
                ts: 11,
            },
        )
        .unwrap();
    store.resolve_issue(issue_id, 12).unwrap();
    store
        .insert_denial(
            sid,
            DenialRecord {
                command: "bash",
                args: &["-c", "pwd"],
                reason: "blocked shell escape",
                suggestion: Some("use ROY command"),
                ts: 13,
            },
        )
        .unwrap();
    let approval_id = store
        .insert_pending_approval(
            sid,
            ApprovalRecord {
                command: "deploy",
                args: &["prod"],
                reason: "needs approval",
                requested_at: 14,
            },
        )
        .unwrap();
    store.resolve_approval(approval_id, 15, "approved").unwrap();
    drop(store);

    let reopened = super::RoyStore::open(&path).expect("reopen must succeed");
    assert_eq!(reopened.list_named_refs(sid).unwrap()[0].target_id, "art:7");
    assert_eq!(reopened.list_issues(sid).unwrap()[0].resolved_at, Some(12));
    assert_eq!(
        reopened.list_denials(sid).unwrap()[0].args,
        vec!["-c", "pwd"]
    );
    assert_eq!(
        reopened.list_all_approvals(sid).unwrap()[0]
            .resolution
            .as_deref(),
        Some("approved")
    );
    remove_db(&path);
}

#[test]
fn invalid_denial_args_json_returns_error() {
    let (path, store) = make_file_store("invalid-denial-json");
    let sid = seed_session(&store);
    store
        .conn
        .execute(
            "INSERT INTO structured_denials (session_id, command, args, reason, suggestion, ts)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                sid as i64,
                "bash",
                "{bad json",
                "blocked",
                Option::<String>::None,
                1_i64
            ],
        )
        .unwrap();

    let err = store
        .list_denials(sid)
        .expect_err("bad JSON must surface as error");
    assert!(err.to_string().contains("expected"));
    remove_db(&path);
}

#[test]
fn invalid_approval_args_json_returns_error() {
    let (path, store) = make_file_store("invalid-approval-json");
    let sid = seed_session(&store);
    store
        .conn
        .execute(
            "INSERT INTO pending_approvals (session_id, command, args, reason, requested_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![sid as i64, "deploy", "{bad json", "needs approval", 1_i64],
        )
        .unwrap();

    let err = store
        .list_all_approvals(sid)
        .expect_err("bad JSON must surface as error");
    assert!(err.to_string().contains("expected"));
    remove_db(&path);
}
