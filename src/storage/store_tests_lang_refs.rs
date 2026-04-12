use super::{make_store, seed_session, NamedRefRecord, Session};

#[test]
fn upsert_and_load_named_ref() {
    let store = make_store();
    let sid = seed_session(&store);
    store
        .upsert_named_ref(
            sid,
            NamedRefRecord {
                name: "last",
                kind: "artifact",
                target_id: "art:1",
                created_at: 100,
            },
        )
        .unwrap();
    let named_ref = store
        .load_named_ref(sid, "last")
        .unwrap()
        .expect("must find ref");
    assert_eq!(named_ref.name, "last");
    assert_eq!(named_ref.kind, "artifact");
    assert_eq!(named_ref.target_id, "art:1");
    assert_eq!(named_ref.created_at, 100);
}

#[test]
fn upsert_named_ref_replaces_existing() {
    let store = make_store();
    let sid = seed_session(&store);
    for (target_id, created_at) in [("art:1", 100_u64), ("art:2", 200_u64)] {
        store
            .upsert_named_ref(
                sid,
                NamedRefRecord {
                    name: "last",
                    kind: "artifact",
                    target_id,
                    created_at,
                },
            )
            .unwrap();
    }

    let named_ref = store
        .load_named_ref(sid, "last")
        .unwrap()
        .expect("must find ref");
    assert_eq!(
        named_ref.target_id, "art:2",
        "upsert must replace existing ref"
    );
}

#[test]
fn load_named_ref_returns_none_for_missing() {
    let store = make_store();
    let sid = seed_session(&store);
    assert!(store.load_named_ref(sid, "no-such-ref").unwrap().is_none());
}

#[test]
fn list_named_refs_returns_all_for_session() {
    let store = make_store();
    let sid = seed_session(&store);
    for (name, target_id, created_at) in [("last", "art:1", 10_u64), ("main-diff", "art:2", 20_u64)]
    {
        store
            .upsert_named_ref(
                sid,
                NamedRefRecord {
                    name,
                    kind: "artifact",
                    target_id,
                    created_at,
                },
            )
            .unwrap();
    }

    assert_eq!(store.list_named_refs(sid).unwrap().len(), 2);
}

#[test]
fn named_refs_scoped_to_session() {
    let store = make_store();
    let s1 = Session::new(1, super::PathBuf::from("/tmp"), 0);
    let s2 = Session::new(2, super::PathBuf::from("/tmp"), 0);
    store.save_session(&s1).unwrap();
    store.save_session(&s2).unwrap();
    store
        .upsert_named_ref(
            s1.id,
            NamedRefRecord {
                name: "last",
                kind: "file",
                target_id: "f1",
                created_at: 1,
            },
        )
        .unwrap();
    assert!(
        store.list_named_refs(s2.id).unwrap().is_empty(),
        "refs must not leak across sessions"
    );
}
