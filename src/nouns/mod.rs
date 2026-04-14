//! Shared noun vocabulary, descriptors, and registry for the ROY v0.2 language.
//!
//! The noun layer sits between the AST/plan layer and execution. Every noun
//! surface (file, session, artifact, denial, …) is described by a
//! [`NounDescriptor`] that powers the planner, help generator, and JSON schema
//! system **without requiring any execution**.
//!
//! ## Dependency direction
//!
//! `src/commands/plan.rs` imports [`NounKind`] and [`Cardinality`] from here.
//! This module does **not** import from `commands::plan` — the noun layer is
//! the authority on what kinds of nouns and cardinalities exist.

// Registry is complete for current nouns; future noun kinds added per issue.
#![allow(dead_code)]

#[path = "noun_registry_data.rs"]
mod noun_registry_data;

// ── shared vocabulary ─────────────────────────────────────────────────────────

/// What kind of noun (subject) a command addresses.
///
/// Variants are added here when a new noun family is wired into the plan layer.
/// Downstream consumers that `match` this must handle all variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NounKind {
    /// A single file path (e.g. `read`, `write`).
    FilePath,
    /// A directory path (e.g. `ls`, `cd`, `pwd`).
    DirPath,
    /// A set of files across the workspace (e.g. `check`).
    FileSet,
    /// The process environment (e.g. `env`).
    EnvMap,
    /// The shell session itself (e.g. `exit`).
    Session,
    /// An embedded agent target (e.g. `claude`).
    AgentTarget,
    /// Help / command-listing output (e.g. `help`, `commands`).
    Help,
    /// A single schema descriptor (e.g. `read schema <name>`).
    SchemaRef,
    /// The set of all registered schemas (e.g. `schemas`).
    SchemaSet,
}

/// Whether a command targets one item or a collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cardinality {
    Singular,
    Plural,
}

// ── noun backing ──────────────────────────────────────────────────────────────

/// What backs a noun at runtime — used by REND-01 and the dispatch layer.
///
/// This is purely metadata; no execution happens here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NounBacking {
    /// Backed by the workspace filesystem, repository index, or AST.
    Workspace,
    /// Backed by the session event ledger or storage layer.
    Session,
    /// Backed by internal registry or schema metadata — no external I/O.
    Meta,
    /// Backed by a process or embedded agent handle.
    Agent,
}

// ── noun descriptor ───────────────────────────────────────────────────────────

/// Static, runtime-free descriptor for one noun surface.
///
/// Descriptors are registered in [`NounRegistry`] and consulted by the
/// planner, help generator, and future schema browser — **no I/O is ever
/// performed by this type**.
///
/// A single [`NounKind`] may have multiple named surfaces with different
/// cardinalities (e.g. `"session"` and `"sessions"` both use
/// [`NounKind::Session`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounDescriptor {
    /// Primary surface name used in ROY commands (e.g. `"file"`, `"sessions"`).
    pub name: &'static str,
    /// Noun kind used by the planner and result router.
    pub kind: NounKind,
    /// Natural cardinality of this surface.
    pub cardinality: Cardinality,
    /// What backs this noun at runtime.
    pub backing: NounBacking,
    /// One-line description shown in help and the schema browser.
    pub summary: &'static str,
    /// Stable key for the JSON schema envelope (unique per descriptor; used by
    /// LANG-08 and JSON-01).
    pub schema_key: &'static str,
}

// ── noun registry ─────────────────────────────────────────────────────────────

/// Registry of all known noun surfaces.
///
/// Zero-sized; looks up into the static noun table built in
/// `noun_registry_data.rs`. Matches the [`crate::commands::registry::CommandRegistry`]
/// pattern so discovery and testing are consistent.
pub struct NounRegistry;

impl NounRegistry {
    /// Create the registry (zero-cost; no allocation).
    pub fn new() -> Self {
        Self
    }

    /// Look up a noun descriptor by surface name, or `None` if unknown.
    pub fn lookup(&self, name: &str) -> Option<&'static NounDescriptor> {
        noun_registry_data::noun_entries()
            .iter()
            .find(|n| n.name == name)
    }

    /// All registered noun descriptors, in registration order.
    pub fn all(&self) -> &'static [NounDescriptor] {
        noun_registry_data::noun_entries()
    }

    /// All descriptors whose [`NounDescriptor::kind`] matches `kind`.
    pub fn by_kind(&self, kind: NounKind) -> Vec<&'static NounDescriptor> {
        noun_registry_data::noun_entries()
            .iter()
            .filter(|n| n.kind == kind)
            .collect()
    }
}

impl Default for NounRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "noun_tests.rs"]
mod tests;
