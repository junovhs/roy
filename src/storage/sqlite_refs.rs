//! Named-ref storage APIs. Sidecar to `sqlite.rs`.

use rusqlite::params;

use super::{RoyStore, StoreError};

// ── types ─────────────────────────────────────────────────────────────────────

/// A session-scoped named ref (e.g. "last", "main-diff").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedRef {
    pub id: i64,
    pub session_id: u64,
    pub name: String,
    pub kind: String,
    pub target_id: String,
    pub created_at: u64,
}

/// Parameters for upserting a named ref.
pub struct NamedRefRecord<'a> {
    pub name: &'a str,
    pub kind: &'a str,
    pub target_id: &'a str,
    pub created_at: u64,
}

// ── impl ──────────────────────────────────────────────────────────────────────

impl RoyStore {
    /// Upsert a named ref — replaces any existing ref with the same session + name.
    pub fn upsert_named_ref(
        &self,
        session_id: u64,
        r: NamedRefRecord<'_>,
    ) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO named_refs (session_id, name, kind, target_id, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                session_id as i64,
                r.name,
                r.kind,
                r.target_id,
                r.created_at as i64
            ],
        )?;
        Ok(())
    }

    /// Load a single named ref by session and name.
    pub fn load_named_ref(
        &self,
        session_id: u64,
        name: &str,
    ) -> Result<Option<NamedRef>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, name, kind, target_id, created_at
             FROM named_refs WHERE session_id = ?1 AND name = ?2",
        )?;
        let rows = stmt
            .query_map(params![session_id as i64, name], |row| {
                Ok(NamedRef {
                    id: row.get(0)?,
                    session_id: row.get::<_, i64>(1)? as u64,
                    name: row.get(2)?,
                    kind: row.get(3)?,
                    target_id: row.get(4)?,
                    created_at: row.get::<_, i64>(5)? as u64,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows.into_iter().next())
    }

    /// List all named refs for a session, ordered by creation time.
    pub fn list_named_refs(&self, session_id: u64) -> Result<Vec<NamedRef>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, name, kind, target_id, created_at
             FROM named_refs WHERE session_id = ?1 ORDER BY created_at, id",
        )?;
        let rows = stmt
            .query_map(params![session_id as i64], |row| {
                Ok(NamedRef {
                    id: row.get(0)?,
                    session_id: row.get::<_, i64>(1)? as u64,
                    name: row.get(2)?,
                    kind: row.get(3)?,
                    target_id: row.get(4)?,
                    created_at: row.get::<_, i64>(5)? as u64,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}
