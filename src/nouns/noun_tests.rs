//! Registry completeness tests for the noun layer.

use super::{NounKind, NounRegistry};

fn reg() -> NounRegistry {
    NounRegistry::new()
}

// ── invariant: no duplicate names ─────────────────────────────────────────────

#[test]
fn noun_names_are_unique() {
    let nouns = reg().all();
    let mut seen = std::collections::HashSet::new();
    for n in nouns {
        assert!(
            seen.insert(n.name),
            "duplicate noun name '{}' in registry",
            n.name
        );
    }
}

// ── invariant: no duplicate schema keys ──────────────────────────────────────

#[test]
fn schema_keys_are_unique() {
    let nouns = reg().all();
    let mut seen = std::collections::HashSet::new();
    for n in nouns {
        assert!(
            seen.insert(n.schema_key),
            "duplicate schema_key '{}' in registry (nouns '{}' collides)",
            n.schema_key,
            n.name
        );
    }
}

// ── invariant: no empty fields ────────────────────────────────────────────────

#[test]
fn all_nouns_have_non_empty_fields() {
    for n in reg().all() {
        assert!(!n.name.is_empty(), "noun has empty name");
        assert!(!n.summary.is_empty(), "noun '{}' has empty summary", n.name);
        assert!(
            !n.schema_key.is_empty(),
            "noun '{}' has empty schema_key",
            n.name
        );
    }
}

// ── lookup ────────────────────────────────────────────────────────────────────

#[test]
fn lookup_known_noun_succeeds() {
    let r = reg();
    let d = r.lookup("file").expect("'file' must be in registry");
    assert_eq!(d.name, "file");
    assert_eq!(d.kind, NounKind::FilePath);
}

#[test]
fn lookup_unknown_noun_returns_none() {
    assert!(reg().lookup("nonexistent_noun_xyz").is_none());
}

#[test]
fn lookup_all_registered_names() {
    let r = reg();
    for n in r.all() {
        let found = r.lookup(n.name).expect("lookup of registered name must succeed");
        assert_eq!(found.name, n.name);
    }
}

// ── by_kind ───────────────────────────────────────────────────────────────────

#[test]
fn by_kind_session_returns_singular_and_plural() {
    let results = reg().by_kind(NounKind::Session);
    assert_eq!(results.len(), 2, "session kind must have exactly 2 surfaces");
    let names: Vec<_> = results.iter().map(|n| n.name).collect();
    assert!(names.contains(&"session"), "missing 'session'");
    assert!(names.contains(&"sessions"), "missing 'sessions'");
}

#[test]
fn by_kind_unknown_kind_with_no_entries() {
    // AgentTarget should have exactly 1 entry.
    let results = reg().by_kind(NounKind::AgentTarget);
    assert_eq!(results.len(), 1);
}

// ── coverage: every NounKind variant must be represented ─────────────────────

#[test]
fn every_noun_kind_has_at_least_one_descriptor() {
    let r = reg();
    let all_kinds = [
        NounKind::FilePath,
        NounKind::DirPath,
        NounKind::FileSet,
        NounKind::EnvMap,
        NounKind::Session,
        NounKind::AgentTarget,
        NounKind::Help,
        NounKind::SchemaRef,
        NounKind::SchemaSet,
    ];
    for kind in &all_kinds {
        assert!(
            !r.by_kind(*kind).is_empty(),
            "NounKind::{kind:?} has no registered noun descriptor"
        );
    }
}
