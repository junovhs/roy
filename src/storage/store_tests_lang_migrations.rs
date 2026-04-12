use super::{
    make_file_store, make_store, params, remove_db, seed_session, Connection, Session, SessionEvent,
};

#[test]
fn migration_002_creates_tables() {
    let store = make_store();
    let sid = seed_session(&store);
    store
        .list_named_refs(sid)
        .expect("named_refs table must exist");
    store.list_issues(sid).expect("issues table must exist");
    store
        .list_denials(sid)
        .expect("structured_denials table must exist");
    store
        .list_pending_approvals(sid)
        .expect("pending_approvals table must exist");
}

#[test]
fn migration_is_idempotent_on_existing_store() {
    let (path, store) = make_file_store("migration-idempotent");
    let sid = seed_session(&store);
    store
        .upsert_named_ref(
            sid,
            super::NamedRefRecord {
                name: "last",
                kind: "artifact",
                target_id: "art:1",
                created_at: 10,
            },
        )
        .unwrap();
    drop(store);

    let reopened = super::RoyStore::open(&path).expect("reopen after migration must succeed");
    let refs = reopened
        .list_named_refs(sid)
        .expect("language-state rows must survive reopen");
    assert_eq!(refs.len(), 1);
    remove_db(&path);
}

#[test]
fn v1_session_data_unaffected_by_migration_002() {
    let store = make_store();
    let mut session = Session::new(7, super::PathBuf::from("/tmp"), 0);
    session.push(SessionEvent::UserInput {
        text: "pwd".into(),
        ts: 1,
    });
    store.save_session(&session).unwrap();
    let events = store.load_events(session.id).unwrap();
    assert_eq!(
        events.len(),
        session.events().len(),
        "v1 events must survive migration 002"
    );
}

#[test]
fn opening_v1_database_upgrades_without_losing_existing_rows() {
    let path = super::temp_db_path("migration-upgrade");
    let event = SessionEvent::UserInput {
        text: "pwd".into(),
        ts: 55,
    };
    let payload = serde_json::to_string(&event).expect("event JSON must serialize");

    {
        let conn = Connection::open(&path).expect("seed v1 db");
        conn.execute_batch(include_str!("../../migrations/001_initial.sql"))
            .expect("v1 schema must apply");
        conn.execute(
            "INSERT INTO sessions (id, workspace_root, started_at) VALUES (?1, ?2, ?3)",
            params![7_i64, "/tmp/ws", 0_i64],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO session_events (session_id, kind, payload, ts)
             VALUES (?1, ?2, ?3, ?4)",
            params![7_i64, event.kind_str(), payload, 55_i64],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO artifacts (session_id, name, kind, summary, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![7_i64, "check", "validation", "cargo check passed", 56_i64],
        )
        .unwrap();
    }

    let store = super::RoyStore::open(&path).expect("open must upgrade v1 db");
    let events = store
        .load_events(7)
        .expect("existing events must survive upgrade");
    let artifacts = store
        .load_artifact_refs(7)
        .expect("existing artifacts must survive upgrade");

    assert_eq!(events, vec![event]);
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].summary, "cargo check passed");
    assert!(
        store.load_named_ref(7, "last").unwrap().is_none(),
        "new tables must exist after upgrade"
    );
    remove_db(&path);
}
