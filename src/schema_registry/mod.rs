//! Internal schema registry — machine-readable descriptions of every noun
//! surface, projection, and result envelope exposed by ROY.
//!
//! ## Purpose
//!
//! Provides the substrate for:
//! - The `schemas` and `schema <name>` user-facing commands
//! - Stable `--json` envelope versioning (JSON-01)
//! - Help generation (HELP-02)
//! - CI coverage checks (every registered noun must have a schema entry)
//!
//! ## Stability contract
//!
//! Each [`SchemaEntry::version`] starts at `1`. A bump signals a **breaking**
//! change to the JSON envelope for that schema. Minor/additive changes (new
//! optional fields) do not bump the version.
//!
//! ## Dependency direction
//!
//! This module imports [`crate::nouns::NounRegistry`] to validate coverage.
//! It does **not** import the shell runtime.

// Schema data and coverage test fully wired; command dispatch in LANG-08.
#![allow(dead_code)]

#[path = "schema_data.rs"]
mod schema_data;

// ── types ─────────────────────────────────────────────────────────────────────

/// Description of a single field in a noun or result schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    /// Field name (stable key in JSON output).
    pub name: &'static str,
    /// Logical type string: `"string"`, `"u64"`, `"bool"`, `"string[]"`,
    /// `"object"`, etc.
    pub type_name: &'static str,
    /// One-line description of this field.
    pub description: &'static str,
    /// Whether this field may be absent from the JSON envelope.
    pub optional: bool,
}

/// A registered schema entry describing one noun surface or result type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaEntry {
    /// Stable name — matches [`crate::nouns::NounDescriptor::schema_key`] for
    /// noun schemas, or a projection/result name for non-noun schemas.
    pub name: &'static str,
    /// Schema version. Incremented on breaking JSON shape changes only.
    pub version: u32,
    /// Display name shown in help and listing output (e.g. `"File"`).
    pub display_name: &'static str,
    /// One-line description.
    pub summary: &'static str,
    /// Field definitions for the JSON `"value"` object.
    /// Empty for scalar schemas (e.g. `Text`, `ExitStatus`).
    pub fields: &'static [FieldDef],
    /// Human-readable description of the singular form's behaviour.
    pub singular_behavior: &'static str,
    /// Human-readable description of the plural form's behaviour.
    pub plural_behavior: &'static str,
    /// A sample command demonstrating this schema in action.
    pub example: &'static str,
}

impl SchemaEntry {
    /// Format a short one-line summary for `schemas` listing output.
    pub fn list_line(&self) -> String {
        format!("{:<12}  v{}  {}", self.name, self.version, self.summary)
    }

    /// Format the full schema description for `read schema <name>` output.
    pub fn full_description(&self) -> String {
        let mut lines = vec![
            format!("schema: {} (v{})", self.name, self.version),
            format!("name:   {}", self.display_name),
            format!("        {}", self.summary),
            String::new(),
            format!("singular: {}", self.singular_behavior),
            format!("plural:   {}", self.plural_behavior),
            String::new(),
            format!("example:  {}", self.example),
        ];
        if !self.fields.is_empty() {
            lines.push(String::new());
            lines.push("fields:".to_string());
            for f in self.fields {
                let opt = if f.optional { " (optional)" } else { "" };
                lines.push(format!("  {:<16} {}  {}{opt}", f.name, f.type_name, f.description));
            }
        }
        lines.join("\n")
    }
}

// ── registry ──────────────────────────────────────────────────────────────────

/// Registry of all ROY noun and result schemas.
///
/// Zero-sized; backed by the static table in `schema_data.rs`.
pub struct SchemaRegistry;

impl SchemaRegistry {
    /// Create the registry (zero-cost; no allocation).
    pub fn new() -> Self {
        Self
    }

    /// Look up a schema by its stable name, or `None` if unknown.
    pub fn lookup(&self, name: &str) -> Option<&'static SchemaEntry> {
        schema_data::schema_entries()
            .iter()
            .find(|s| s.name == name)
    }

    /// All registered schema entries, in registration order.
    pub fn all(&self) -> &'static [SchemaEntry] {
        schema_data::schema_entries()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "schema_tests.rs"]
mod tests;
