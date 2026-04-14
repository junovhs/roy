//! Artifact query APIs with filtering and pagination. Sidecar to `sqlite.rs`.

use rusqlite::params;

use super::{RoyStore, StoreError, StoredArtifactRef};

// ── query type ────────────────────────────────────────────────────────────────

/// Bounded query for artifacts within a session.
///
/// All filter fields are optional; unset fields match any value.
/// `limit` defaults to 1000 when `None`; `offset` defaults to 0.
#[derive(Debug, Default)]
pub struct ArtifactQuery<'a> {
    /// Restrict to artifacts of this kind (e.g. `"diff"`, `"validation"`).
    pub kind: Option<&'a str>,
    /// Lower-bound `created_at` timestamp (inclusive).
    pub since: Option<u64>,
    /// Upper-bound `created_at` timestamp (inclusive).
    pub until: Option<u64>,
    /// Maximum rows to return. `None` → capped at 1000.
    pub limit: Option<u64>,
    /// Rows to skip before returning (for pagination).
    pub offset: Option<u64>,
}

// ── impl ──────────────────────────────────────────────────────────────────────

impl RoyStore {
    /// Query artifacts for a session with optional kind/time filtering and pagination.
    ///
    /// Results are ordered by `created_at` ascending, then by rowid.
    pub fn query_artifacts(
        &self,
        session_id: u64,
        q: &ArtifactQuery<'_>,
    ) -> Result<Vec<StoredArtifactRef>, StoreError> {
        let limit = q.limit.unwrap_or(1000) as i64;
        let offset = q.offset.unwrap_or(0) as i64;
        let mut stmt = self.conn.prepare(
            "SELECT name, kind, summary, created_at FROM artifacts
             WHERE session_id = ?1
               AND (?2 IS NULL OR kind    = ?2)
               AND (?3 IS NULL OR created_at >= ?3)
               AND (?4 IS NULL OR created_at <= ?4)
             ORDER BY created_at, id
             LIMIT ?5 OFFSET ?6",
        )?;
        let rows = stmt
            .query_map(
                params![
                    session_id as i64,
                    q.kind,
                    q.since.map(|t| t as i64),
                    q.until.map(|t| t as i64),
                    limit,
                    offset,
                ],
                |row| {
                    Ok(StoredArtifactRef {
                        name: row.get(0)?,
                        kind: row.get(1)?,
                        summary: row.get(2)?,
                        created_at: row.get::<_, i64>(3)? as u64,
                    })
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Count artifacts in a session, optionally restricted to one kind.
    ///
    /// Pass `None` to count all artifact kinds.
    pub fn count_artifacts(
        &self,
        session_id: u64,
        kind: Option<&str>,
    ) -> Result<u64, StoreError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM artifacts
             WHERE session_id = ?1 AND (?2 IS NULL OR kind = ?2)",
            params![session_id as i64, kind],
            |row| row.get(0),
        )?;
        Ok(count as u64)
    }
}
