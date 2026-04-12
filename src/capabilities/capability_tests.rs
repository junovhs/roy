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
    let outside = root.parent().unwrap().join(format!(
        "escape-{}.txt",
        root.file_name().unwrap().to_string_lossy()
    ));
    std::fs::write(&outside, "escape").unwrap();

    let err = runtime
        .execute(&CapabilityRequest::Fs(FsCapability::ReadFile {
            path: outside.display().to_string(),
        }))
        .unwrap_err();

    assert!(err.to_string().contains("workspace boundary"));
}

#[test]
fn list_dir_labels_symlinks_as_link() {
    let root = temp_workspace("cap-symlink");
    std::fs::write(root.join("target.txt"), "t").unwrap();
    std::os::unix::fs::symlink(root.join("target.txt"), root.join("link.txt")).unwrap();

    let output = runtime(&root)
        .execute(&CapabilityRequest::Fs(FsCapability::ListDir { path: None }))
        .unwrap();

    let CapabilityOutput::DirectoryListing { entries, .. } = output else {
        panic!("expected listing");
    };
    let link = entries.iter().find(|e| e.name == "link.txt").expect("symlink must appear");
    assert_eq!(link.kind, "link", "symlink must be labelled 'link', not 'file'");
}

#[test]
fn list_dir_labels_dirs_as_dir() {
    let root = temp_workspace("cap-dir-kind");
    std::fs::create_dir(root.join("subdir")).unwrap();
    std::fs::write(root.join("file.txt"), "x").unwrap();

    let output = runtime(&root)
        .execute(&CapabilityRequest::Fs(FsCapability::ListDir { path: None }))
        .unwrap();

    let CapabilityOutput::DirectoryListing { entries, .. } = output else { panic!() };
    let dir_entry = entries.iter().find(|e| e.name == "subdir").unwrap();
    let file_entry = entries.iter().find(|e| e.name == "file.txt").unwrap();
    assert_eq!(dir_entry.kind, "dir");
    assert_eq!(file_entry.kind, "file");
}

#[test]
fn write_file_rejects_path_that_is_a_directory() {
    let root = temp_workspace("cap-write-dir");
    std::fs::create_dir(root.join("adir")).unwrap();

    let err = runtime(&root)
        .execute(&CapabilityRequest::Fs(FsCapability::WriteFile {
            path: "adir".to_string(),
            contents: "oops".to_string(),
        }))
        .unwrap_err();

    assert!(err.to_string().contains("directory"), "got: {err}");
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

// ── CapabilityOutput methods ──────────────────────────────────────────────────

fn validation(exit_code: i32, stdout: &str, stderr: &str) -> CapabilityOutput {
    CapabilityOutput::ValidationRun {
        command: "cargo test".to_string(),
        cwd: PathBuf::from("/ws"),
        exit_code,
        stdout: stdout.to_string(),
        stderr: stderr.to_string(),
    }
}

#[test]
fn exit_code_is_zero_for_non_validation() {
    let listing = CapabilityOutput::DirectoryListing {
        path: PathBuf::from("/ws"),
        entries: vec![],
    };
    assert_eq!(listing.exit_code(), 0);

    let contents = CapabilityOutput::FileContents {
        path: PathBuf::from("/ws/f"),
        contents: "x".to_string(),
    };
    assert_eq!(contents.exit_code(), 0);
}

#[test]
fn exit_code_reflects_validation_run_exit_code() {
    assert_eq!(validation(0, "", "").exit_code(), 0);
    assert_eq!(validation(1, "", "").exit_code(), 1);
    assert_eq!(validation(42, "", "").exit_code(), 42);
}

#[test]
fn primary_text_success_no_stdout_shows_passed_summary() {
    let text = validation(0, "", "").primary_text();
    assert!(text.contains("passed"), "got: {text}");
}

#[test]
fn primary_text_success_with_stdout_shows_stdout() {
    let text = validation(0, "test output", "").primary_text();
    assert_eq!(text, "test output");
}

#[test]
fn primary_text_failure_with_stderr_shows_stderr() {
    let text = validation(1, "", "error: something broke").primary_text();
    assert_eq!(text, "error: something broke");
}

#[test]
fn primary_text_failure_no_stderr_shows_exit_code() {
    let text = validation(2, "", "").primary_text();
    assert!(text.contains("exit code 2"), "got: {text}");
}

#[test]
fn error_text_none_on_success() {
    assert!(validation(0, "", "some stderr").error_text().is_none());
}

#[test]
fn error_text_none_when_stderr_empty() {
    assert!(validation(1, "", "").error_text().is_none());
}

#[test]
fn error_text_none_when_stderr_equals_stdout() {
    assert!(validation(1, "same", "same").error_text().is_none());
}

#[test]
fn error_text_some_when_exit_nonzero_and_stderr_distinct() {
    let out = validation(1, "stdout text", "stderr text");
    assert_eq!(out.error_text(), Some("stderr text"));
}
