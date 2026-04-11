// Live in tests; UI wiring and session manager integration pending DB-02.
#![allow(dead_code)]

//! SQLite-backed ROY store — save, replay, and manage session state on disk.

use std::path::Path;

use rusqlite::{params, Connection};

use crate::session::{Session, SessionArtifact, SessionEvent};

/// Error returned by storage operations.
#[derive(Debug)]
pub struct StoreError(String);

impl StoreError {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for StoreError {}

impl From<rusqlite::Error> for StoreError {
    fn from(e: rusqlite::Error) -> Self {
        Self::new(e.to_string())
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(e.to_string())
    }
}

// ── store ─────────────────────────────────────────────────────────────────────

/// ROY persistence store backed by a local SQLite database.
///
/// One database file per installation. Sessions and their ordered event
/// ledgers are stored using the schema in `migrations/001_initial.sql`.
pub struct RoyStore {
    conn: Connection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredArtifactRef {
    pub name: String,
    pub kind: String,
    pub summary: String,
    pub created_at: u64,
}

impl RoyStore {
    /// Open (or create) the database at `path`, applying all pending migrations.
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.apply_migrations()?;
        Ok(store)
    }

    /// Open a transient in-memory database — used in tests and dev mode.
    pub fn open_memory() -> Result<Self, StoreError> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.apply_migrations()?;
        Ok(store)
    }

    fn apply_migrations(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(include_str!("../../migrations/001_initial.sql"))?;
        Ok(())
    }

    // ── session lifecycle ─────────────────────────────────────────────────────

    /// Create a session record, then persist all of its current events.
    ///
    /// Idempotent: uses `INSERT OR REPLACE` so replaying a session twice is safe.
    pub fn save_session(&self, session: &Session) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sessions (id, workspace_root, started_at)
             VALUES (?1, ?2, ?3)",
            params![
                session.id as i64,
                session.workspace_root.display().to_string(),
                session.id as i64,
            ],
        )?;
        self.conn.execute(
            "DELETE FROM session_events WHERE session_id = ?1",
            params![session.id as i64],
        )?;
        self.conn.execute(
            "DELETE FROM artifacts WHERE session_id = ?1",
            params![session.id as i64],
        )?;
        for event in session.events() {
            self.insert_event(session.id, event)?;
        }
        Ok(())
    }

    /// Append a single event to an existing session record.
    ///
    /// The session row must exist (call [`save_session`] first).
    pub fn append_event(&self, session_id: u64, event: &SessionEvent) -> Result<(), StoreError> {
        self.insert_event(session_id, event)
    }

    /// Mark a session as ended with `exit_code`.
    pub fn close_session(
        &self,
        session_id: u64,
        exit_code: i32,
        ended_at: u64,
    ) -> Result<(), StoreError> {
        self.conn.execute(
            "UPDATE sessions SET ended_at = ?1, exit_code = ?2 WHERE id = ?3",
            params![ended_at as i64, exit_code, session_id as i64],
        )?;
        Ok(())
    }

    // ── replay ────────────────────────────────────────────────────────────────

    /// Load all events for `session_id` in chronological order.
    ///
    /// Returns an empty `Vec` if the session has no recorded events.
    /// Returns `Err` if the payload JSON is malformed.
    pub fn load_events(&self, session_id: u64) -> Result<Vec<SessionEvent>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT payload FROM session_events
             WHERE session_id = ?1
             ORDER BY ts, id",
        )?;
        let events = stmt
            .query_map(params![session_id as i64], |row| {
                let payload: String = row.get(0)?;
                Ok(payload)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        events
            .into_iter()
            .map(|payload| {
                serde_json::from_str::<SessionEvent>(&payload).map_err(StoreError::from)
            })
            .collect()
    }

    pub fn load_artifact_refs(&self, session_id: u64) -> Result<Vec<StoredArtifactRef>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT name, kind, summary, created_at
             FROM artifacts
             WHERE session_id = ?1
             ORDER BY created_at, id",
        )?;
        let rows = stmt
            .query_map(params![session_id as i64], |row| {
                Ok(StoredArtifactRef {
                    name: row.get(0)?,
                    kind: row.get(1)?,
                    summary: row.get(2)?,
                    created_at: row.get::<_, i64>(3)? as u64,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    fn insert_event(&self, session_id: u64, event: &SessionEvent) -> Result<(), StoreError> {
        let payload = serde_json::to_string(event)?;
        self.conn.execute(
            "INSERT INTO session_events (session_id, kind, payload, ts)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                session_id as i64,
                event.kind_str(),
                payload,
                event.timestamp() as i64,
            ],
        )?;
        if let SessionEvent::ArtifactCreated { artifact, ts } = event {
            self.insert_artifact(session_id, artifact, *ts)?;
        }
        Ok(())
    }

    fn insert_artifact(
        &self,
        session_id: u64,
        artifact: &SessionArtifact,
        created_at: u64,
    ) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT INTO artifacts (session_id, name, kind, summary, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                session_id as i64,
                artifact.name.as_str(),
                artifact.kind_str(),
                artifact.summary.as_str(),
                created_at as i64,
            ],
        )?;
        Ok(())
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;
