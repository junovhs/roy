//! Issue storage APIs. Sidecar to `sqlite.rs`.

use rusqlite::params;

use super::{RoyStore, StoreError};

// ── types ─────────────────────────────────────────────────────────────────────

/// A command-processing issue recorded within a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredIssue {
    pub id: i64,
    pub session_id: u64,
    pub kind: String,
    pub message: String,
    pub command: Option<String>,
    pub ts: u64,
    pub resolved_at: Option<u64>,
}

/// Parameters for inserting a new issue.
pub struct IssueRecord<'a> {
    pub kind: &'a str,
    pub message: &'a str,
    pub command: Option<&'a str>,
    pub ts: u64,
}

// ── impl ──────────────────────────────────────────────────────────────────────

impl RoyStore {
    /// Record a new issue; returns the row id.
    pub fn insert_issue(&self, session_id: u64, issue: IssueRecord<'_>) -> Result<i64, StoreError> {
        self.conn.execute(
            "INSERT INTO issues (session_id, kind, message, command, ts)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                session_id as i64,
                issue.kind,
                issue.message,
                issue.command,
                issue.ts as i64
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Mark an issue as resolved.
    pub fn resolve_issue(&self, id: i64, resolved_at: u64) -> Result<(), StoreError> {
        self.conn.execute(
            "UPDATE issues SET resolved_at = ?1 WHERE id = ?2",
            params![resolved_at as i64, id],
        )?;
        Ok(())
    }

    /// List all issues for a session, ordered by time.
    pub fn list_issues(&self, session_id: u64) -> Result<Vec<StoredIssue>, StoreError> {
        self.query_issues(session_id, false)
    }

    /// List only unresolved issues for a session.
    pub fn list_open_issues(&self, session_id: u64) -> Result<Vec<StoredIssue>, StoreError> {
        self.query_issues(session_id, true)
    }

    fn query_issues(
        &self,
        session_id: u64,
        open_only: bool,
    ) -> Result<Vec<StoredIssue>, StoreError> {
        let sql = if open_only {
            "SELECT id, session_id, kind, message, command, ts, resolved_at
             FROM issues WHERE session_id = ?1 AND resolved_at IS NULL ORDER BY ts, id"
        } else {
            "SELECT id, session_id, kind, message, command, ts, resolved_at
             FROM issues WHERE session_id = ?1 ORDER BY ts, id"
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt
            .query_map(params![session_id as i64], |row| {
                Ok(StoredIssue {
                    id: row.get(0)?,
                    session_id: row.get::<_, i64>(1)? as u64,
                    kind: row.get(2)?,
                    message: row.get(3)?,
                    command: row.get(4)?,
                    ts: row.get::<_, i64>(5)? as u64,
                    resolved_at: row.get::<_, Option<i64>>(6)?.map(|v| v as u64),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}
