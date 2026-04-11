-- ROY v0.1 initial schema
-- Stores sessions, shell traces, installed agents, workspaces, and artifacts.

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- Core shell sessions
CREATE TABLE IF NOT EXISTS sessions (
    id      INTEGER PRIMARY KEY,   -- ms since UNIX epoch (Session.id)
    workspace_root TEXT NOT NULL,
    started_at     INTEGER NOT NULL,
    ended_at       INTEGER,        -- NULL while session is open
    exit_code      INTEGER         -- NULL while session is open
);

-- Typed session events (ordered by ts within a session)
CREATE TABLE IF NOT EXISTS session_events (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    kind       TEXT    NOT NULL,   -- event.kind_str() for cheap SQL filtering
    payload    TEXT    NOT NULL,   -- full JSON (serde_json serialised SessionEvent)
    ts         INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_session_events_lookup
    ON session_events (session_id, ts);

-- Declared workspaces with their policy bindings
CREATE TABLE IF NOT EXISTS workspaces (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    path           TEXT    NOT NULL UNIQUE,
    label          TEXT,
    policy_profile TEXT    NOT NULL DEFAULT 'permissive'
);

-- Installed agent binaries known to ROY
CREATE TABLE IF NOT EXISTS installed_agents (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    kind         TEXT    NOT NULL,
    version      TEXT    NOT NULL,
    install_path TEXT    NOT NULL,
    registered_at INTEGER NOT NULL
);

-- Agent process runs linked to shell sessions
CREATE TABLE IF NOT EXISTS agent_sessions (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id    INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    agent_kind    TEXT    NOT NULL,
    agent_version TEXT    NOT NULL,
    state         TEXT    NOT NULL DEFAULT 'initializing',
    started_at    INTEGER NOT NULL,
    ended_at      INTEGER,
    exit_code     INTEGER
);

-- Significant outputs promoted to first-class artifacts
CREATE TABLE IF NOT EXISTS artifacts (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    name       TEXT    NOT NULL,
    kind       TEXT    NOT NULL,   -- diff | validation | denied_trace | …
    summary    TEXT    NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL
);
