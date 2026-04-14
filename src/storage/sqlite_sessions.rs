//! Session metadata query APIs. Sidecar to `sqlite.rs`.

use rusqlite::params;

use super::{RoyStore, StoreError};

// ── types ─────────────────────────────────────────────────────────────────────

/// Metadata for one recorded session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub id: u64,
    pub workspace_root: String,
    pub started_at: u64,
    /// `None` while the session is still open.
    pub ended_at: Option<u64>,
    /// `None` while the session is still open.
    pub exit_code: Option<i32>,
}

/// Bounded query for sessions by time window.
///
/// `limit` defaults to 100 when `None`; `offset` defaults to 0.
/// Results are ordered newest-first (`started_at DESC`).
#[derive(Debug, Default)]
pub struct SessionQuery {
    /// Earliest `started_at` (inclusive).
    pub since: Option<u64>,
    /// Latest `started_at` (inclusive).
    pub until: Option<u64>,
    /// Maximum rows to return. `None` → capped at 100.
    pub limit: Option<u64>,
    /// Rows to skip before returning.
    pub offset: Option<u64>,
}

// ── impl ──────────────────────────────────────────────────────────────────────

impl RoyStore {
    /// Load a single session by id, or `None` if not found.
    pub fn load_session(&self, id: u64) -> Result<Option<SessionRecord>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, workspace_root, started_at, ended_at, exit_code
             FROM sessions WHERE id = ?1",
        )?;
        let rows = stmt
            .query_map(params![id as i64], decode_session_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows.into_iter().next())
    }

    /// List sessions within an optional time window, newest first, with pagination.
    pub fn list_sessions(&self, q: &SessionQuery) -> Result<Vec<SessionRecord>, StoreError> {
        let limit = q.limit.unwrap_or(100) as i64;
        let offset = q.offset.unwrap_or(0) as i64;
        let mut stmt = self.conn.prepare(
            "SELECT id, workspace_root, started_at, ended_at, exit_code
             FROM sessions
             WHERE (?1 IS NULL OR started_at >= ?1)
               AND (?2 IS NULL OR started_at <= ?2)
             ORDER BY started_at DESC
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = stmt
            .query_map(
                params![
                    q.since.map(|t| t as i64),
                    q.until.map(|t| t as i64),
                    limit,
                    offset,
                ],
                decode_session_row,
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}

fn decode_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SessionRecord> {
    Ok(SessionRecord {
        id: row.get::<_, i64>(0)? as u64,
        workspace_root: row.get(1)?,
        started_at: row.get::<_, i64>(2)? as u64,
        ended_at: row.get::<_, Option<i64>>(3)?.map(|v| v as u64),
        exit_code: row.get(4)?,
    })
}
