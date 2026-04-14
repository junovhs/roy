//! CI-enforced schema coverage tests.
//!
//! The core invariant: every noun surface registered in [`NounRegistry`]
//! must have a corresponding [`SchemaEntry`] in [`SchemaRegistry`].
//! Missing coverage fails loudly here so it cannot ship undetected.

use crate::nouns::NounRegistry;

use super::SchemaRegistry;

fn noun_reg() -> NounRegistry {
    NounRegistry::new()
}
fn schema_reg() -> SchemaRegistry {
    SchemaRegistry::new()
}

// ── invariant: no duplicate schema names ──────────────────────────────────────

#[test]
fn schema_names_are_unique() {
    let schemas = schema_reg().all();
    let mut seen = std::collections::HashSet::new();
    for s in schemas {
        assert!(
            seen.insert(s.name),
            "duplicate schema name '{}' in registry",
            s.name
        );
    }
}

// ── invariant: no empty required fields ──────────────────────────────────────

#[test]
fn all_schemas_have_non_empty_required_fields() {
    for s in schema_reg().all() {
        assert!(!s.name.is_empty(), "schema has empty name");
        assert!(!s.summary.is_empty(), "schema '{}' has empty summary", s.name);
        assert!(!s.display_name.is_empty(), "schema '{}' has empty display_name", s.name);
        assert!(s.version >= 1, "schema '{}' must have version >= 1", s.name);
        assert!(!s.example.is_empty(), "schema '{}' has empty example", s.name);
        for f in s.fields {
            assert!(!f.name.is_empty(), "schema '{}' has field with empty name", s.name);
            assert!(!f.type_name.is_empty(), "schema '{}' field '{}' has empty type_name", s.name, f.name);
        }
    }
}

// ── coverage: every registered noun must have a schema ───────────────────────

/// This is the key CI gate: if a new noun is registered but no schema is
/// written, this test fails and blocks the build.
#[test]
fn every_noun_schema_key_has_a_registry_entry() {
    let nouns = noun_reg().all();
    let schemas = schema_reg();
    for noun in nouns {
        assert!(
            schemas.lookup(noun.schema_key).is_some(),
            "noun '{}' has schema_key '{}' but no SchemaEntry registered — \
             add a SchemaEntry to src/schema_registry/schema_data.rs",
            noun.name,
            noun.schema_key
        );
    }
}

// ── lookup ────────────────────────────────────────────────────────────────────

#[test]
fn lookup_known_schema_succeeds() {
    let r = schema_reg();
    let s = r.lookup("env").expect("'env' schema must exist");
    assert_eq!(s.name, "env");
    assert_eq!(s.version, 1);
}

#[test]
fn lookup_unknown_schema_returns_none() {
    assert!(schema_reg().lookup("nonexistent_schema_xyz").is_none());
}

#[test]
fn lookup_all_registered_names() {
    let r = schema_reg();
    for s in r.all() {
        let found = r.lookup(s.name).expect("lookup of registered name must succeed");
        assert_eq!(found.name, s.name);
    }
}

// ── list_line and full_description render ────────────────────────────────────

#[test]
fn list_line_contains_name_version_and_summary() {
    let s = schema_reg().lookup("env").unwrap();
    let line = s.list_line();
    assert!(line.contains("env"), "list_line missing name");
    assert!(line.contains("v1"), "list_line missing version");
    assert!(line.contains("environment"), "list_line missing summary keyword");
}

#[test]
fn full_description_contains_example() {
    let s = schema_reg().lookup("sessions").unwrap();
    let desc = s.full_description();
    assert!(desc.contains("sessions since"), "full_description missing example");
}

#[test]
fn full_description_lists_fields_when_present() {
    let s = schema_reg().lookup("sessions").unwrap();
    let desc = s.full_description();
    assert!(desc.contains("started_at"), "full_description missing field 'started_at'");
}

#[test]
fn full_description_no_fields_section_when_empty() {
    let s = schema_reg().lookup("files").unwrap();
    let desc = s.full_description();
    assert!(!desc.contains("fields:"), "empty fields list should not show 'fields:' header");
}
