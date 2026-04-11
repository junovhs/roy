//! Tests for ShellRuntime ROY-native command dispatch.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::shell::{DispatchResult, ShellRuntime};

fn temp_workspace(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("roy-native-{prefix}-{unique}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn ls_lists_workspace_entries() {
    let root = temp_workspace("ls");
    std::fs::write(root.join("a.txt"), "hello").unwrap();
    std::fs::create_dir(root.join("src")).unwrap();

    let mut rt = ShellRuntime::new(root);
    match rt.dispatch("ls", &[]) {
        DispatchResult::Executed { output, exit_code, artifacts } => {
            assert_eq!(exit_code, 0);
            assert!(output.contains("a.txt"));
            assert!(output.contains("src"));
            assert!(artifacts.is_empty(), "ls should not promote an artifact");
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn read_prints_file_contents() {
    let root = temp_workspace("read");
    let file = root.join("note.txt");
    std::fs::write(&file, "hello world").unwrap();

    let mut rt = ShellRuntime::new(root);
    match rt.dispatch("read", &["note.txt"]) {
        DispatchResult::Executed { output, exit_code, artifacts } => {
            assert_eq!(exit_code, 0);
            assert_eq!(output, "hello world");
            assert!(artifacts.is_empty(), "read should not promote an artifact");
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn write_creates_file() {
    let root = temp_workspace("write");
    let mut rt = ShellRuntime::new(root.clone());

    match rt.dispatch("write", &["note.txt", "hello world"]) {
        DispatchResult::Executed { output, exit_code, artifacts } => {
            assert_eq!(exit_code, 0);
            assert!(output.contains("wrote"));
            assert_eq!(artifacts.len(), 1, "write should promote a diff artifact");
        }
        other => panic!("expected Executed, got {other:?}"),
    }

    assert_eq!(std::fs::read_to_string(root.join("note.txt")).unwrap(), "hello world");
}

#[test]
fn write_requires_contents_arg() {
    let root = temp_workspace("write-usage");
    let mut rt = ShellRuntime::new(root);

    match rt.dispatch("write", &["note.txt"]) {
        DispatchResult::Executed { exit_code, .. } => assert_eq!(exit_code, 2),
        other => panic!("expected Executed, got {other:?}"),
    }

    assert!(!rt.drain_errors().is_empty(), "usage error must hit stderr");
}

#[test]
fn read_rejects_escape_outside_workspace() {
    let root = temp_workspace("escape-root");
    let outside = root
        .parent()
        .unwrap()
        .join(format!("outside-{}.txt", root.file_name().unwrap().to_string_lossy()));
    std::fs::write(&outside, "escape").unwrap();

    let mut rt = ShellRuntime::new(root);
    match rt.dispatch("read", &[outside.to_str().unwrap()]) {
        DispatchResult::Executed { exit_code, output, .. } => {
            assert_eq!(exit_code, 1);
            assert!(output.contains("workspace boundary"));
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn check_runs_cargo_check_for_current_workspace() {
    let root = temp_workspace("check");
    std::fs::create_dir(root.join("src")).unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"tool02_check_fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn fixture() -> u32 { 7 }\n").unwrap();

    let mut rt = ShellRuntime::new(root.clone());
    match rt.dispatch("check", &[]) {
        DispatchResult::Executed { output, exit_code, artifacts } => {
            assert_eq!(exit_code, 0);
            assert!(output.contains("cargo check"));
            assert!(output.contains(root.to_str().unwrap()));
            assert_eq!(artifacts.len(), 1, "check should promote a validation artifact");
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}
