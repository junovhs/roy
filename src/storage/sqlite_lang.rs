//! Denial and approval storage APIs. Sidecar to `sqlite.rs`.

use rusqlite::params;

use super::{RoyStore, StoreError};

// ── structured denials ────────────────────────────────────────────────────────

/// A structured denial record with full redirect/hint payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredDenial {
    pub id: i64,
    pub session_id: u64,
    pub command: String,
    pub args: Vec<String>,
    pub reason: String,
    pub suggestion: Option<String>,
    pub redirect: Option<String>,
    pub ts: u64,
}

/// Parameters for inserting a denial.
///
/// `redirect` is reserved for POL-03 structured redirect payloads; leave as
/// `None` until that feature is implemented.
pub struct DenialRecord<'a> {
    pub command: &'a str,
    pub args: &'a [&'a str],
    pub reason: &'a str,
    pub suggestion: Option<&'a str>,
    pub ts: u64,
}

impl RoyStore {
    /// Record a structured denial for a session.
    pub fn insert_denial(
        &self,
        session_id: u64,
        denial: DenialRecord<'_>,
    ) -> Result<(), StoreError> {
        let args_json = serde_json::to_string(denial.args)?;
        self.conn.execute(
            "INSERT INTO structured_denials (session_id, command, args, reason, suggestion, ts)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                session_id as i64,
                denial.command,
                args_json,
                denial.reason,
                denial.suggestion,
                denial.ts as i64
            ],
        )?;
        Ok(())
    }

    /// List all denials for a session, ordered by time.
    pub fn list_denials(&self, session_id: u64) -> Result<Vec<StoredDenial>, StoreError> {
        self.query_denials_by(
            session_id,
            &DenialQuery {
                command: None,
                since: None,
                until: None,
                limit: None,
                offset: None,
            },
        )
    }

    /// Filtered denial query with optional command/time constraints and pagination.
    ///
    /// `limit` defaults to 1000 when `None`; `offset` defaults to 0.
    pub fn query_denials_by(
        &self,
        session_id: u64,
        q: &DenialQuery<'_>,
    ) -> Result<Vec<StoredDenial>, StoreError> {
        let limit = q.limit.unwrap_or(1000) as i64;
        let offset = q.offset.unwrap_or(0) as i64;
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, command, args, reason, suggestion, redirect, ts
             FROM structured_denials
             WHERE session_id = ?1
               AND (?2 IS NULL OR command = ?2)
               AND (?3 IS NULL OR ts >= ?3)
               AND (?4 IS NULL OR ts <= ?4)
             ORDER BY ts, id
             LIMIT ?5 OFFSET ?6",
        )?;
        let rows = stmt
            .query_map(
                params![
                    session_id as i64,
                    q.command,
                    q.since.map(|t| t as i64),
                    q.until.map(|t| t as i64),
                    limit,
                    offset,
                ],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, i64>(7)?,
                    ))
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;

        rows.into_iter()
            .map(
                |(id, stored_session_id, command, args_json, reason, suggestion, redirect, ts)| {
                    let args = serde_json::from_str(&args_json)?;
                    Ok(StoredDenial {
                        id,
                        session_id: stored_session_id as u64,
                        command,
                        args,
                        reason,
                        suggestion,
                        redirect,
                        ts: ts as u64,
                    })
                },
            )
            .collect()
    }
}

// ── denial query type ─────────────────────────────────────────────────────────

/// Filtered query for structured denials within a session.
///
/// All filter fields are optional; unset fields match any value.
#[derive(Debug, Default)]
pub struct DenialQuery<'a> {
    /// Restrict to denials of this command name (e.g. `"bash"`, `"curl"`).
    pub command: Option<&'a str>,
    /// Lower-bound timestamp (inclusive).
    pub since: Option<u64>,
    /// Upper-bound timestamp (inclusive).
    pub until: Option<u64>,
    /// Maximum rows. `None` → capped at 1000.
    pub limit: Option<u64>,
    /// Rows to skip.
    pub offset: Option<u64>,
}

// Pending-approval APIs live in sqlite_approvals.rs to keep this file within token limits.
