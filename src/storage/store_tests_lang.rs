//! Integration tests for v0.2 language-state storage.

pub(super) use super::issues::IssueRecord;
pub(super) use super::lang::{ApprovalRecord, DenialRecord};
pub(super) use super::refs::NamedRefRecord;
pub(super) use super::RoyStore;
pub(super) use crate::session::{Session, SessionEvent};
pub(super) use rusqlite::{params, Connection};
pub(super) use std::path::{Path, PathBuf};
pub(super) use std::time::{SystemTime, UNIX_EPOCH};

fn make_store() -> RoyStore {
    RoyStore::open_memory().expect("in-memory store must open")
}

fn make_file_store(prefix: &str) -> (PathBuf, RoyStore) {
    let path = temp_db_path(prefix);
    let store = RoyStore::open(&path).expect("file-backed store must open");
    (path, store)
}

fn temp_db_path(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "roy-{prefix}-{}-{unique}.sqlite3",
        std::process::id()
    ))
}

fn remove_db(path: &Path) {
    let _ = std::fs::remove_file(path);
}

fn seed_session(store: &RoyStore) -> u64 {
    let session = Session::new(42, PathBuf::from("/tmp/ws"), 0);
    store
        .save_session(&session)
        .expect("seed session must succeed");
    session.id
}

#[path = "store_tests_lang_migrations.rs"]
mod migrations;

#[path = "store_tests_lang_refs.rs"]
mod refs;

#[path = "store_tests_lang_issues.rs"]
mod issues;

#[path = "store_tests_lang_lang.rs"]
mod lang;
