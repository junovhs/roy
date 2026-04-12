use std::path::PathBuf;

use super::{build_unified_diff, diff_ops, ArtifactBody, ArtifactKind, DiffOp, SessionArtifact};

#[test]
fn diff_artifact_for_new_file_uses_dev_null_origin() {
    let artifact = SessionArtifact::diff(
        PathBuf::from("src/lib.rs"),
        None,
        "pub fn answer() -> u32 { 42 }".to_string(),
    )
    .expect("new file should create a diff artifact");

    assert_eq!(artifact.kind, ArtifactKind::Diff);
    assert_eq!(
        artifact.name, "lib.rs",
        "short_name should use the filename, not full path"
    );
    let ArtifactBody::Diff { diff, .. } = artifact.body else {
        panic!("expected diff payload");
    };
    assert!(diff.contains("--- /dev/null"));
    assert!(diff.contains("+++ b/src/lib.rs"));
}

#[test]
fn diff_artifact_skips_noop_write() {
    let artifact = SessionArtifact::diff(
        PathBuf::from("note.txt"),
        Some("same".to_string()),
        "same".to_string(),
    );
    assert!(artifact.is_none(), "no-op writes should not spam artifacts");
}

#[test]
fn denied_command_artifact_keeps_args() {
    let artifact = SessionArtifact::denied_command(
        "grep",
        &["-r", "needle", "."],
        "use ROY-native commands".to_string(),
    );

    assert_eq!(artifact.kind, ArtifactKind::DeniedCommandTrace);
    let ArtifactBody::DeniedCommandTrace { args, .. } = artifact.body else {
        panic!("expected denied trace payload");
    };
    assert_eq!(args, vec!["-r", "needle", "."]);
}

// ── diff_ops / build_unified_diff ─────────────────────────────────────────────

fn op(d: DiffOp<'_>) -> (&'_ str, &'_ str) {
    match d {
        DiffOp::Keep(s) => ("=", s),
        DiffOp::Add(s) => ("+", s),
        DiffOp::Remove(s) => ("-", s),
    }
}

#[test]
fn diff_ops_empty_to_lines_all_add() {
    let ops: Vec<_> = diff_ops(&[], &["a", "b"]).into_iter().map(op).collect();
    assert_eq!(ops, vec![("+", "a"), ("+", "b")]);
}

#[test]
fn diff_ops_lines_to_empty_all_remove() {
    let ops: Vec<_> = diff_ops(&["a", "b"], &[]).into_iter().map(op).collect();
    assert_eq!(ops, vec![("-", "a"), ("-", "b")]);
}

#[test]
fn diff_ops_identical_all_keep() {
    let ops: Vec<_> = diff_ops(&["x", "y"], &["x", "y"])
        .into_iter()
        .map(op)
        .collect();
    assert_eq!(ops, vec![("=", "x"), ("=", "y")]);
}

#[test]
fn diff_ops_middle_change() {
    let ops: Vec<_> = diff_ops(&["a", "b", "c"], &["a", "B", "c"])
        .into_iter()
        .map(op)
        .collect();
    assert_eq!(ops, vec![("=", "a"), ("-", "b"), ("+", "B"), ("=", "c")]);
}

#[test]
fn diff_ops_append_line() {
    let ops: Vec<_> = diff_ops(&["a", "b"], &["a", "b", "c"])
        .into_iter()
        .map(op)
        .collect();
    assert_eq!(ops, vec![("=", "a"), ("=", "b"), ("+", "c")]);
}

#[test]
fn diff_ops_delete_first_line() {
    let ops: Vec<_> = diff_ops(&["a", "b"], &["b"]).into_iter().map(op).collect();
    assert_eq!(ops, vec![("-", "a"), ("=", "b")]);
}

#[test]
fn unified_diff_identical_content_reports_no_changes() {
    let diff = build_unified_diff(std::path::Path::new("f.rs"), Some("same\n"), "same\n");
    assert!(
        diff.contains("(no textual changes)"),
        "identical content must say no changes, got:\n{diff}"
    );
    assert!(
        !diff
            .lines()
            .any(|l| l.starts_with('+') && !l.starts_with("+++")),
        "no addition lines"
    );
    assert!(
        !diff
            .lines()
            .any(|l| l.starts_with('-') && !l.starts_with("---")),
        "no removal lines"
    );
}

#[test]
fn unified_diff_new_file_marks_all_lines_added() {
    let diff = build_unified_diff(
        std::path::Path::new("f.rs"),
        None,
        "fn foo() {}\nfn bar() {}",
    );
    assert!(diff.contains("+fn foo() {}"), "new lines must be marked +");
    assert!(diff.contains("+fn bar() {}"));
    // No content lines should be removals (--- header is fine)
    assert!(!diff
        .lines()
        .any(|l| l.starts_with('-') && !l.starts_with("---")));
}

#[test]
fn unified_diff_modified_file_marks_old_and_new() {
    let diff = build_unified_diff(
        std::path::Path::new("f.rs"),
        Some("old line\nshared"),
        "new line\nshared",
    );
    assert!(diff.contains("-old line"));
    assert!(diff.contains("+new line"));
    assert!(
        diff.contains(" shared"),
        "unchanged line must have space prefix"
    );
}

#[test]
fn validation_run_summary_uses_exit_code() {
    let a = SessionArtifact::validation_run(
        "cargo test".to_string(),
        PathBuf::from("/ws"),
        1,
        String::new(),
        "FAILED".to_string(),
    );
    assert!(a.summary.contains("failed"));

    let b = SessionArtifact::validation_run(
        "cargo test".to_string(),
        PathBuf::from("/ws"),
        0,
        String::new(),
        String::new(),
    );
    assert!(b.summary.contains("passed"));
}
