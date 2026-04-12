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
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, command, args, reason, suggestion, redirect, ts
             FROM structured_denials WHERE session_id = ?1 ORDER BY ts, id",
        )?;
        let rows = stmt
            .query_map(params![session_id as i64], |row| {
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
            })?
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

// ── pending approvals ─────────────────────────────────────────────────────────

/// An approval-pending change record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredApproval {
    pub id: i64,
    pub session_id: u64,
    pub command: String,
    pub args: Vec<String>,
    pub reason: String,
    pub requested_at: u64,
    pub resolved_at: Option<u64>,
    pub resolution: Option<String>,
}

/// Parameters for recording a pending approval.
pub struct ApprovalRecord<'a> {
    pub command: &'a str,
    pub args: &'a [&'a str],
    pub reason: &'a str,
    pub requested_at: u64,
}

impl RoyStore {
    /// Record a pending-approval entry; returns the row id.
    pub fn insert_pending_approval(
        &self,
        session_id: u64,
        approval: ApprovalRecord<'_>,
    ) -> Result<i64, StoreError> {
        let args_json = serde_json::to_string(approval.args)?;
        self.conn.execute(
            "INSERT INTO pending_approvals (session_id, command, args, reason, requested_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                session_id as i64,
                approval.command,
                args_json,
                approval.reason,
                approval.requested_at as i64
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Resolve an approval with the given outcome.
    pub fn resolve_approval(
        &self,
        id: i64,
        resolved_at: u64,
        resolution: &str,
    ) -> Result<(), StoreError> {
        self.conn.execute(
            "UPDATE pending_approvals SET resolved_at = ?1, resolution = ?2 WHERE id = ?3",
            params![resolved_at as i64, resolution, id],
        )?;
        Ok(())
    }

    /// List only unresolved approvals for a session.
    pub fn list_pending_approvals(
        &self,
        session_id: u64,
    ) -> Result<Vec<StoredApproval>, StoreError> {
        self.query_approvals(session_id, true)
    }

    /// List all approvals (pending and resolved) for a session.
    pub fn list_all_approvals(&self, session_id: u64) -> Result<Vec<StoredApproval>, StoreError> {
        self.query_approvals(session_id, false)
    }

    fn query_approvals(
        &self,
        session_id: u64,
        pending_only: bool,
    ) -> Result<Vec<StoredApproval>, StoreError> {
        let sql = if pending_only {
            "SELECT id, session_id, command, args, reason, requested_at, resolved_at, resolution
             FROM pending_approvals WHERE session_id = ?1 AND resolved_at IS NULL
             ORDER BY requested_at, id"
        } else {
            "SELECT id, session_id, command, args, reason, requested_at, resolved_at, resolution
             FROM pending_approvals WHERE session_id = ?1
             ORDER BY requested_at, id"
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt
            .query_map(params![session_id as i64], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, Option<i64>>(6)?,
                    row.get::<_, Option<String>>(7)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        rows.into_iter()
            .map(
                |(
                    id,
                    stored_session_id,
                    command,
                    args_json,
                    reason,
                    requested_at,
                    resolved_at,
                    resolution,
                )| {
                    let args = serde_json::from_str(&args_json)?;
                    Ok(StoredApproval {
                        id,
                        session_id: stored_session_id as u64,
                        command,
                        args,
                        reason,
                        requested_at: requested_at as u64,
                        resolved_at: resolved_at.map(|v| v as u64),
                        resolution,
                    })
                },
            )
            .collect()
    }
}
