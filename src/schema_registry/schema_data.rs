//! Static schema table — one [`SchemaEntry`] per registered noun surface.
//!
//! To add a new schema: append to `SCHEMAS` and run `cargo test` to verify the
//! noun-coverage invariant in `schema_tests.rs`.

use super::{FieldDef, SchemaEntry};

pub(super) fn schema_entries() -> &'static [SchemaEntry] {
    SCHEMAS
}

static SCHEMAS: &[SchemaEntry] = &[
    // ── workspace-backed ──────────────────────────────────────────────────────
    SchemaEntry {
        name: "file",
        version: 1,
        display_name: "File",
        summary: "Raw content of a single workspace file",
        fields: &[
            FieldDef {
                name: "path",
                type_name: "string",
                description: "Workspace-relative path of the file",
                optional: false,
            },
            FieldDef {
                name: "content",
                type_name: "string",
                description: "Full text content of the file",
                optional: false,
            },
        ],
        singular_behavior: "Returns content of the named file (e.g. `read main.rs`)",
        plural_behavior: "N/A — use `files` for multi-file results",
        example: "read src/main.rs",
    },
    SchemaEntry {
        name: "files",
        version: 1,
        display_name: "FileList",
        summary: "A list of file paths matching workspace filters",
        fields: &[],
        singular_behavior: "N/A — use `file` for single-file content",
        plural_behavior: "Returns a newline-separated list of matching paths",
        example: "ls src/",
    },
    SchemaEntry {
        name: "dir",
        version: 1,
        display_name: "Directory",
        summary: "A single directory path",
        fields: &[],
        singular_behavior: "Returns or changes to the named directory (e.g. `cd`, `pwd`)",
        plural_behavior: "N/A — use `files` for directory listings",
        example: "pwd",
    },
    // ── session-backed ────────────────────────────────────────────────────────
    SchemaEntry {
        name: "session",
        version: 1,
        display_name: "Session",
        summary: "The current shell session state",
        fields: &[],
        singular_behavior: "Refers to the active session (e.g. `exit`)",
        plural_behavior: "N/A — use `sessions` for ledger queries",
        example: "exit 0",
    },
    SchemaEntry {
        name: "sessions",
        version: 1,
        display_name: "SessionList",
        summary: "Historical sessions in the session ledger",
        fields: &[
            FieldDef {
                name: "id",
                type_name: "u64",
                description: "Session ID (milliseconds since epoch)",
                optional: false,
            },
            FieldDef {
                name: "workspace_root",
                type_name: "string",
                description: "Absolute workspace root path",
                optional: false,
            },
            FieldDef {
                name: "started_at",
                type_name: "u64",
                description: "Session start time (milliseconds since epoch)",
                optional: false,
            },
            FieldDef {
                name: "ended_at",
                type_name: "u64",
                description: "Session end time, if closed",
                optional: true,
            },
            FieldDef {
                name: "exit_code",
                type_name: "i32",
                description: "Exit code, if the session ended normally",
                optional: true,
            },
        ],
        singular_behavior: "N/A — use `session` for the current session",
        plural_behavior: "Returns paginated session history from the ledger",
        example: "sessions since 1hr",
    },
    // ── agent-backed ──────────────────────────────────────────────────────────
    SchemaEntry {
        name: "agent",
        version: 1,
        display_name: "Agent",
        summary: "An embedded agent process handle",
        fields: &[],
        singular_behavior: "Launches or refers to an embedded agent (e.g. `claude`)",
        plural_behavior: "N/A — ROY manages a single active agent",
        example: "claude",
    },
    // ── meta-backed ───────────────────────────────────────────────────────────
    SchemaEntry {
        name: "env",
        version: 1,
        display_name: "EnvMap",
        summary: "The controlled process environment variable map",
        fields: &[],
        singular_behavior: "N/A — env is always a full key-value map",
        plural_behavior: "Returns all environment variables, optionally filtered",
        example: "env PATH",
    },
    SchemaEntry {
        name: "help",
        version: 1,
        display_name: "Help",
        summary: "Command help and discovery surface",
        fields: &[],
        singular_behavior: "N/A — help always lists available commands",
        plural_behavior: "Returns formatted help text or command listing",
        example: "help",
    },
    SchemaEntry {
        name: "schema",
        version: 1,
        display_name: "Schema",
        summary: "A single noun or result schema descriptor",
        fields: &[
            FieldDef {
                name: "name",
                type_name: "string",
                description: "Stable schema key",
                optional: false,
            },
            FieldDef {
                name: "version",
                type_name: "u32",
                description: "Schema version number",
                optional: false,
            },
            FieldDef {
                name: "summary",
                type_name: "string",
                description: "One-line description",
                optional: false,
            },
            FieldDef {
                name: "fields",
                type_name: "object[]",
                description: "Field definitions for the JSON value object",
                optional: false,
            },
        ],
        singular_behavior: "Returns full details for the named schema (e.g. `read schema env`)",
        plural_behavior: "N/A — use `schemas` for listing",
        example: "read schema env",
    },
    SchemaEntry {
        name: "schemas",
        version: 1,
        display_name: "SchemaList",
        summary: "All registered noun and result schemas",
        fields: &[],
        singular_behavior: "N/A — use `schema <name>` for a single schema",
        plural_behavior: "Returns a summary listing of all registered schemas",
        example: "schemas",
    },
];
