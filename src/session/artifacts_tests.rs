use std::path::PathBuf;

use super::{ArtifactBody, ArtifactKind, SessionArtifact};

#[test]
fn diff_artifact_for_new_file_uses_dev_null_origin() {
    let artifact = SessionArtifact::diff(
        PathBuf::from("src/lib.rs"),
        None,
        "pub fn answer() -> u32 { 42 }".to_string(),
    )
    .expect("new file should create a diff artifact");

    assert_eq!(artifact.kind, ArtifactKind::Diff);
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
