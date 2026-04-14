//! Static noun table — one entry per ROY noun surface.

use super::{Cardinality, NounBacking, NounDescriptor, NounKind};

/// All registered noun surfaces, in surface-name order.
///
/// Each [`NounDescriptor::name`] must be unique. Each
/// [`NounDescriptor::schema_key`] must also be unique. Tests in
/// `noun_tests.rs` enforce both invariants.
pub(super) fn noun_entries() -> &'static [NounDescriptor] {
    NOUNS
}

static NOUNS: &[NounDescriptor] = &[
    // ── workspace-backed ──────────────────────────────────────────────────────
    NounDescriptor {
        name: "file",
        kind: NounKind::FilePath,
        cardinality: Cardinality::Singular,
        backing: NounBacking::Workspace,
        summary: "A single file in the workspace",
        schema_key: "file",
    },
    NounDescriptor {
        name: "files",
        kind: NounKind::FileSet,
        cardinality: Cardinality::Plural,
        backing: NounBacking::Workspace,
        summary: "A set of files across the workspace",
        schema_key: "files",
    },
    NounDescriptor {
        name: "dir",
        kind: NounKind::DirPath,
        cardinality: Cardinality::Singular,
        backing: NounBacking::Workspace,
        summary: "A single directory in the workspace",
        schema_key: "dir",
    },
    // ── session-backed ────────────────────────────────────────────────────────
    NounDescriptor {
        name: "session",
        kind: NounKind::Session,
        cardinality: Cardinality::Singular,
        backing: NounBacking::Session,
        summary: "The current shell session",
        schema_key: "session",
    },
    NounDescriptor {
        name: "sessions",
        kind: NounKind::Session,
        cardinality: Cardinality::Plural,
        backing: NounBacking::Session,
        summary: "Historical sessions in the session ledger",
        schema_key: "sessions",
    },
    // ── agent-backed ──────────────────────────────────────────────────────────
    NounDescriptor {
        name: "agent",
        kind: NounKind::AgentTarget,
        cardinality: Cardinality::Singular,
        backing: NounBacking::Agent,
        summary: "An embedded agent process (e.g. Claude Code, Codex)",
        schema_key: "agent",
    },
    // ── meta-backed ───────────────────────────────────────────────────────────
    NounDescriptor {
        name: "env",
        kind: NounKind::EnvMap,
        cardinality: Cardinality::Plural,
        backing: NounBacking::Meta,
        summary: "The process environment variable map",
        schema_key: "env",
    },
    NounDescriptor {
        name: "help",
        kind: NounKind::Help,
        cardinality: Cardinality::Plural,
        backing: NounBacking::Meta,
        summary: "Command help and discovery surface",
        schema_key: "help",
    },
    NounDescriptor {
        name: "schema",
        kind: NounKind::SchemaRef,
        cardinality: Cardinality::Singular,
        backing: NounBacking::Meta,
        summary: "A single noun/result schema descriptor",
        schema_key: "schema",
    },
    NounDescriptor {
        name: "schemas",
        kind: NounKind::SchemaSet,
        cardinality: Cardinality::Plural,
        backing: NounBacking::Meta,
        summary: "All registered noun and result schemas",
        schema_key: "schemas",
    },
];
