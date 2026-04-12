-- ROY v0.2 language state
-- Queryable tables for named refs, issues, structured denials, and pending approvals.
-- Migration is idempotent (IF NOT EXISTS on every object).

-- Session-scoped named refs (last, save as, show ref — LANG-07)
CREATE TABLE IF NOT EXISTS named_refs (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    name       TEXT    NOT NULL,   -- ref name: "last", "main-diff", …
    kind       TEXT    NOT NULL,   -- "artifact" | "file" | "command" | "session"
    target_id  TEXT    NOT NULL,   -- opaque pointer (artifact id, path, command, …)
    created_at INTEGER NOT NULL,
    UNIQUE(session_id, name)       -- upsert via INSERT OR REPLACE
);

CREATE INDEX IF NOT EXISTS idx_named_refs_session
    ON named_refs (session_id);

-- Command-processing issues recorded within a session
CREATE TABLE IF NOT EXISTS issues (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id  INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    kind        TEXT    NOT NULL,   -- "parse_error" | "policy_violation" | "not_found" | …
    message     TEXT    NOT NULL,
    command     TEXT,               -- command that triggered the issue, NULL if not applicable
    ts          INTEGER NOT NULL,
    resolved_at INTEGER             -- NULL while unresolved
);

CREATE INDEX IF NOT EXISTS idx_issues_session
    ON issues (session_id, ts);

-- Structured denial records with full redirect/hint payloads
CREATE TABLE IF NOT EXISTS structured_denials (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    command    TEXT    NOT NULL,
    args       TEXT    NOT NULL DEFAULT '[]',  -- JSON array of arg strings
    reason     TEXT    NOT NULL,
    suggestion TEXT,                            -- ROY-world alternative text
    redirect   TEXT,                            -- suggested ROY-native command string
    ts         INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_structured_denials_session
    ON structured_denials (session_id, ts);

-- Approval-pending change records (policy ApprovalPending outcomes)
CREATE TABLE IF NOT EXISTS pending_approvals (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id   INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    command      TEXT    NOT NULL,
    args         TEXT    NOT NULL DEFAULT '[]',  -- JSON array of arg strings
    reason       TEXT    NOT NULL,
    requested_at INTEGER NOT NULL,
    resolved_at  INTEGER,                         -- NULL while pending
    resolution   TEXT                             -- "approved" | "denied" | NULL
);

CREATE INDEX IF NOT EXISTS idx_pending_approvals_session
    ON pending_approvals (session_id, resolved_at);
