//! Plan layer — validates a parsed [`Command`] against the registry and
//! resolves it into an executable [`Plan`].
//!
//! The planner is pure and side-effect-free, making it safe to use for
//! `--dry` preflight, help suggestions, and policy inspection before any
//! I/O is performed.
//!
//! ## Design contract
//!
//! - [`Planner::plan`] reads only the registry and the parsed command; it
//!   touches no filesystem, no environment, and no global state.
//! - A `Plan` answers every question the dispatch layer needs up-front:
//!   noun kind, cardinality, result type, filter classification, mutation
//!   class, and render mode.
//! - Refiners that are type-incompatible with the result type are rejected
//!   here rather than at runtime.

// Plan types are fully wired; planner is pure and ready for dispatch integration.
#![allow(dead_code)]

use super::ast::{Command, Filter, Refiner};
use super::registry::CommandRegistry;
use super::schema::{Backend, RiskLevel};

// NounKind and Cardinality are defined in the noun layer; re-exported here so
// the plan API stays stable for existing callers.
pub use crate::nouns::{Cardinality, NounKind};

/// The type produced by a command (or, in a multi-stage pipeline, by one stage).
#[derive(Debug, Clone, PartialEq)]
pub enum ResultType {
    /// Raw file content as a text stream.
    FileContent,
    /// A list of file or directory paths.
    FileList,
    /// Key-value environment variable map.
    EnvMap,
    /// Generic text output (help text, validation output, etc.).
    Text,
    /// A process exit-status code.
    ExitStatus,
    /// An embedded agent session handle.
    AgentSession,
    /// No meaningful output (write, cd, exit side-effect commands).
    Empty,
}

/// How a [`Filter`] is applied relative to data materialisation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterClass {
    /// Applied at the source before data is fetched
    /// (e.g. line-range, key:value path predicate).
    SourcePushdown,
    /// Applied after the result is fully materialised
    /// (e.g. about-search, bare-text match).
    PostMaterialization,
}

/// A filter paired with its resolved evaluation classification.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassifiedFilter {
    pub filter: Filter,
    pub class: FilterClass,
}

/// Whether a command modifies any state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MutationClass {
    ReadOnly,
    Mutating,
}

/// Requested output render mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Human,
    Json,
}

// ── plan ─────────────────────────────────────────────────────────────────────

/// A fully resolved, type-checked, executable plan derived from a parsed [`Command`].
///
/// Produced by [`Planner::plan`]. Carries everything the dispatch layer needs
/// without performing any I/O itself.
#[derive(Debug, PartialEq)]
pub struct Plan {
    /// Resolved verb string (lowercased command name).
    pub verb: String,
    /// What kind of noun this command addresses.
    pub noun: NounKind,
    /// Whether this command produces one item or many.
    pub cardinality: Cardinality,
    /// Type produced by the final stage of this plan.
    pub result_type: ResultType,
    /// Filters with their resolved evaluation classification.
    pub classified_filters: Vec<ClassifiedFilter>,
    /// Pipeline refiners to apply after the primary result is produced.
    pub refiners: Vec<Refiner>,
    /// Whether this command modifies any state.
    pub mutation_class: MutationClass,
    /// Output render mode requested by `--json`.
    pub render_mode: RenderMode,
    /// If `--dry` was set, describe what would happen without executing.
    pub dry_run: bool,
    /// If `--ref <name>` was set, bind the result to this session ref name.
    pub ref_name: Option<String>,
    /// Execution backend resolved from the registry.
    pub backend: Backend,
    /// Risk classification from the registry.
    pub risk_level: RiskLevel,
}

// ── plan error ────────────────────────────────────────────────────────────────

/// An error produced when a [`Command`] cannot be resolved into a valid [`Plan`].
#[derive(Debug, Clone, PartialEq)]
pub enum PlanError {
    /// The verb is not registered in the command registry.
    UnknownVerb { verb: String },
    /// The verb is registered but its backend denies execution.
    VerbDenied { verb: String, reason: String },
    /// This verb requires a target argument, but none was supplied.
    MissingTarget { verb: String },
    /// A refiner is incompatible with the result type the command produces.
    RefinerTypeMismatch {
        /// Human-readable description of the failing refiner.
        refiner_desc: String,
        /// The result type the command actually produces.
        result_type: ResultType,
    },
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::UnknownVerb { verb } => {
                write!(f, "unknown command '{verb}' — run `help` to see available commands")
            }
            PlanError::VerbDenied { verb, reason } => {
                write!(f, "'{verb}' is not available: {reason}")
            }
            PlanError::MissingTarget { verb } => {
                write!(f, "'{verb}' requires a target argument")
            }
            PlanError::RefinerTypeMismatch {
                refiner_desc,
                result_type,
            } => {
                write!(
                    f,
                    "refiner '{refiner_desc}' cannot be applied to {result_type:?} — \
                     'sorted by' and 'grouped by' require a record set"
                )
            }
        }
    }
}

// ── planner ───────────────────────────────────────────────────────────────────

/// Pure, side-effect-free command planner.
///
/// Validates a parsed [`Command`] against the registry and resolves it into a
/// [`Plan`] ready for dispatch, or a [`PlanError`] that explains what is wrong.
///
/// # Example
///
/// ```ignore
/// let reg = CommandRegistry::new();
/// let planner = Planner::new(&reg);
/// let cmd = ast::parse("read src/main.rs").unwrap();
/// let plan = planner.plan(&cmd).unwrap();
/// assert_eq!(plan.mutation_class, MutationClass::ReadOnly);
/// ```
pub struct Planner<'r> {
    pub(super) registry: &'r CommandRegistry,
}

impl<'r> Planner<'r> {
    pub fn new(registry: &'r CommandRegistry) -> Self {
        Self { registry }
    }
}

// Planner::plan and private helpers live in the sidecar to stay within token limits.
#[path = "plan_impl.rs"]
mod planner_impl;

#[cfg(test)]
#[path = "plan_tests.rs"]
mod tests;
