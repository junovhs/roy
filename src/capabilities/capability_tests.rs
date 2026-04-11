use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

fn temp_workspace(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("roy-{prefix}-{unique}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn runtime(root: &Path) -> CapabilityRuntime {
    CapabilityRuntime::new(
        WorkspaceBoundary::new(root.to_path_buf()),
        root.to_path_buf(),
    )
}

#[test]
fn list_dir_returns_sorted_entries() {
    let root = temp_workspace("cap-list");
    std::fs::write(root.join("b.txt"), "b").unwrap();
    std::fs::create_dir(root.join("alpha")).unwrap();

    let output = runtime(&root)
        .execute(&CapabilityRequest::Fs(FsCapability::ListDir { path: None }))
        .unwrap();

    match output {
        CapabilityOutput::DirectoryListing { entries, .. } => {
            assert_eq!(entries[0].name, "alpha");
            assert_eq!(entries[0].kind, "dir");
            assert_eq!(entries[1].name, "b.txt");
        }
        other => panic!("expected DirectoryListing, got {other:?}"),
    }
}

#[test]
fn write_then_read_round_trips_file_contents() {
    let root = temp_workspace("cap-write");
    let runtime = runtime(&root);

    runtime
        .execute(&CapabilityRequest::Fs(FsCapability::WriteFile {
            path: "note.txt".to_string(),
            contents: "hello".to_string(),
        }))
        .unwrap();

    let output = runtime
        .execute(&CapabilityRequest::Fs(FsCapability::ReadFile {
            path: "note.txt".to_string(),
        }))
        .unwrap();

    match output {
        CapabilityOutput::FileContents { contents, .. } => assert_eq!(contents, "hello"),
        other => panic!("expected FileContents, got {other:?}"),
    }
}

#[test]
fn read_rejects_escape_outside_workspace() {
    let root = temp_workspace("cap-escape-root");
    let runtime = runtime(&root);
    let outside = root
        .parent()
        .unwrap()
        .join(format!("escape-{}.txt", root.file_name().unwrap().to_string_lossy()));
    std::fs::write(&outside, "escape").unwrap();

    let err = runtime
        .execute(&CapabilityRequest::Fs(FsCapability::ReadFile {
            path: outside.display().to_string(),
        }))
        .unwrap_err();

    assert!(err.to_string().contains("workspace boundary"));
}

#[test]
fn cargo_check_reports_missing_manifest() {
    let root = temp_workspace("cap-check-missing");
    let err = runtime(&root)
        .execute(&CapabilityRequest::Validation(
            ValidationCapability::CargoCheck,
        ))
        .unwrap_err();

    assert!(err.to_string().contains("no Cargo.toml"));
}
