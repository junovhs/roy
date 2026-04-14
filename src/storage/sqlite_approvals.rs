//! Pending-approval storage APIs. Sidecar to `sqlite.rs`.

use rusqlite::params;

use super::{RoyStore, StoreError};

// ── types ─────────────────────────────────────────────────────────────────────

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

// ── impl ──────────────────────────────────────────────────────────────────────

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
