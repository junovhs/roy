//! Typed result container and format-specific renderers for ROY command output.
//!
//! ## Design
//!
//! Execution produces a [`CommandResult`] holding a typed [`NounValue`].
//! Renderers consume that container and produce the output format — **the value
//! itself is never re-executed or mutated by a renderer**.
//!
//! ```text
//! execute()  →  CommandResult { value: NounValue, … }
//!                           ↓                  ↓
//!                   human::render()      json::render()
//!                    (terminal text)       (JSON string)
//! ```
//!
//! ## Dependency direction
//!
//! `src/render/` imports [`crate::commands::plan::RenderMode`] and
//! [`crate::session::SessionArtifact`]. It does **not** import the shell
//! runtime, keeping it testable in isolation.

// Typed container is complete; builtins migrating progressively per REND-01.
#![allow(dead_code)]

use std::path::PathBuf;

use crate::commands::plan::RenderMode;
use crate::session::SessionArtifact;

#[path = "human.rs"]
pub mod human;

#[path = "json.rs"]
pub mod json;

// ── typed value ───────────────────────────────────────────────────────────────

/// The typed output produced by command execution.
///
/// Variants correspond to [`crate::commands::plan::ResultType`]; the split
/// exists to carry concrete data rather than just a type tag.
#[derive(Debug, Clone, PartialEq)]
pub enum NounValue {
    /// Raw file content with its source path.
    FileContent { path: PathBuf, content: String },
    /// A list of file (or directory) paths.
    FileList(Vec<PathBuf>),
    /// Key-value environment variable pairs, in display order.
    EnvMap(Vec<(String, String)>),
    /// Generic text output (help text, validation output, etc.).
    Text(String),
    /// Process exit status code.
    ExitStatus(i32),
    /// No meaningful output (write, cd, exit side-effect commands).
    Empty,
}

// ── diagnostics ───────────────────────────────────────────────────────────────

/// Severity of a diagnostic message attached to a result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// A diagnostic message attached to a [`CommandResult`].
///
/// Diagnostics carry non-fatal information (warnings, hints, deprecation
/// notices) alongside the result. Fatal errors are returned as `Err` before
/// a `CommandResult` is constructed.
///
/// Stable error codes are added in LANG-10.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    /// Optional stable machine-readable code (e.g. `"E-NOUN-CARD"`) for future
    /// use by LANG-10. `None` until that issue lands.
    pub code: Option<&'static str>,
}

impl Diagnostic {
    /// Construct an informational diagnostic.
    pub fn info(message: impl Into<String>) -> Self {
        Self { severity: Severity::Info, message: message.into(), code: None }
    }

    /// Construct a warning diagnostic.
    pub fn warning(message: impl Into<String>) -> Self {
        Self { severity: Severity::Warning, message: message.into(), code: None }
    }

    /// Construct an error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        Self { severity: Severity::Error, message: message.into(), code: None }
    }
}

// ── command result ────────────────────────────────────────────────────────────

/// A fully-typed result ready for rendering.
///
/// Carries the execution output, its schema identity, how it should be
/// rendered, any non-fatal diagnostics, and any session artifacts produced
/// as a side-effect.
///
/// The same `CommandResult` can be passed to [`human::render`] and
/// [`json::render`] without re-executing the command.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandResult {
    /// The typed execution output.
    pub value: NounValue,
    /// Stable key for the JSON schema envelope (matches [`crate::nouns::NounDescriptor::schema_key`]).
    pub schema_key: &'static str,
    /// Requested output render mode.
    pub render_mode: RenderMode,
    /// Non-fatal diagnostics produced during execution.
    pub diagnostics: Vec<Diagnostic>,
    /// Session artifacts produced as a side-effect of this command.
    pub artifacts: Vec<SessionArtifact>,
}

impl CommandResult {
    /// Convenience constructor for a clean text result with no diagnostics.
    pub fn text(value: impl Into<String>, schema_key: &'static str) -> Self {
        Self {
            value: NounValue::Text(value.into()),
            schema_key,
            render_mode: RenderMode::Human,
            diagnostics: Vec::new(),
            artifacts: Vec::new(),
        }
    }

    /// Convenience constructor for an env-map result.
    pub fn env_map(pairs: Vec<(String, String)>) -> Self {
        Self {
            value: NounValue::EnvMap(pairs),
            schema_key: "env",
            render_mode: RenderMode::Human,
            diagnostics: Vec::new(),
            artifacts: Vec::new(),
        }
    }

    /// Convenience constructor for a file list result.
    pub fn file_list(paths: Vec<PathBuf>) -> Self {
        Self {
            value: NounValue::FileList(paths),
            schema_key: "files",
            render_mode: RenderMode::Human,
            diagnostics: Vec::new(),
            artifacts: Vec::new(),
        }
    }

    /// Set the render mode, returning `self` for chaining.
    pub fn with_render_mode(mut self, mode: RenderMode) -> Self {
        self.render_mode = mode;
        self
    }
}

#[cfg(test)]
#[path = "render_tests.rs"]
mod tests;
