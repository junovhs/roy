//! Tests for ShellRuntime built-in command handlers:
//! pwd, cd, env, exit, help, and the transcript drain.
//! Discovery-surface tests (commands builtin, help sections, not_found hint)
//! are in runtime_tests_discoverability.rs.

use crate::shell::{DispatchResult, ShellRuntime};

fn rt() -> ShellRuntime {
    ShellRuntime::new(std::env::temp_dir())
}

// ── pwd ───────────────────────────────────────────────────────────────────────

#[test]
fn pwd_returns_cwd_with_exit_zero() {
    let mut rt = rt();
    let expected = std::env::temp_dir().display().to_string();
    match rt.dispatch("pwd", &[]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            assert_eq!(output, expected);
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn pwd_writes_to_transcript() {
    let mut rt = rt();
    rt.dispatch("pwd", &[]);
    let lines = rt.drain_output();
    assert!(!lines.is_empty(), "pwd must write to transcript");
    assert_eq!(lines[0], std::env::temp_dir().display().to_string());
}

// ── cd ────────────────────────────────────────────────────────────────────────

#[test]
fn cd_absolute_path_changes_cwd() {
    // cd to the workspace root itself — always a valid within-workspace move.
    let root = std::env::temp_dir().canonicalize().unwrap();
    let mut rt = ShellRuntime::new(root.clone());
    match rt.dispatch("cd", &[root.to_str().unwrap()]) {
        DispatchResult::CwdChanged { to } => {
            assert_eq!(to, root);
            assert_eq!(rt.env().cwd(), root.as_path());
            assert_eq!(rt.last_exit_status(), Some(0));
        }
        other => panic!("expected CwdChanged, got {other:?}"),
    }
}

#[test]
fn cd_nonexistent_returns_error_and_exit_one() {
    let mut rt = rt();
    match rt.dispatch("cd", &["/nonexistent_roy_test_path_xyz_99992"]) {
        DispatchResult::Executed { exit_code, .. } => {
            assert_eq!(exit_code, 1);
            assert_eq!(rt.last_exit_status(), Some(1));
        }
        other => panic!("expected Executed with error, got {other:?}"),
    }
}

#[test]
fn cd_nonexistent_writes_error_to_transcript() {
    let mut rt = rt();
    rt.dispatch("cd", &["/nonexistent_roy_test_path_xyz_99993"]);
    assert!(
        !rt.drain_errors().is_empty(),
        "cd error must write to error transcript"
    );
}

#[test]
fn cd_no_args_returns_cwd_unchanged() {
    let root = std::env::temp_dir();
    let mut rt = ShellRuntime::new(root.clone());
    match rt.dispatch("cd", &[]) {
        DispatchResult::CwdChanged { to } => assert_eq!(to, root),
        other => panic!("expected CwdChanged, got {other:?}"),
    }
}

// ── env ───────────────────────────────────────────────────────────────────────

#[test]
fn env_output_contains_path_and_shell() {
    let mut rt = rt();
    match rt.dispatch("env", &[]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            assert!(output.contains("PATH="), "env must include PATH");
            assert!(output.contains("SHELL=roy"), "env must include SHELL=roy");
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn env_filter_narrows_output() {
    let mut rt = rt();
    match rt.dispatch("env", &["SHELL"]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            assert!(output.contains("SHELL=roy"));
            assert!(!output.contains("PATH="), "filter should exclude PATH");
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

// ── exit ──────────────────────────────────────────────────────────────────────

#[test]
fn exit_default_code_is_zero() {
    let mut rt = rt();
    assert_eq!(rt.dispatch("exit", &[]), DispatchResult::Exit { code: 0 });
    assert_eq!(rt.last_exit_status(), Some(0));
}

#[test]
fn exit_with_explicit_code() {
    let mut rt = rt();
    assert_eq!(
        rt.dispatch("exit", &["42"]),
        DispatchResult::Exit { code: 42 }
    );
    assert_eq!(rt.last_exit_status(), Some(42));
}

#[test]
fn quit_alias_exits() {
    let mut rt = rt();
    assert_eq!(rt.dispatch("quit", &[]), DispatchResult::Exit { code: 0 });
}

// ── help ──────────────────────────────────────────────────────────────────────

#[test]
fn help_lists_builtins_and_exits_zero() {
    let mut rt = rt();
    match rt.dispatch("help", &[]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            assert!(output.contains("ROY"));
            assert!(output.contains("cd"));
            assert!(output.contains("pwd"));
            assert!(output.contains("env"));
            assert!(output.contains("exit"));
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

// ── drain ─────────────────────────────────────────────────────────────────────

#[test]
fn drain_output_clears_buffer() {
    let mut rt = rt();
    rt.dispatch("pwd", &[]);
    let first = rt.drain_output();
    assert!(!first.is_empty());
    let second = rt.drain_output();
    assert!(second.is_empty(), "drain must clear the buffer");
}
