//! Tests for ShellRuntime help and discoverability surfaces:
//! help sections, commands builtin, and not_found hint.

use crate::shell::{DispatchResult, ShellRuntime};

fn rt() -> ShellRuntime {
    ShellRuntime::new(std::env::temp_dir())
}

// ── help ──────────────────────────────────────────────────────────────────────

#[test]
fn help_shows_workspace_and_directory() {
    let root = std::env::temp_dir().canonicalize().unwrap();
    let mut rt = ShellRuntime::new(root.clone());
    match rt.dispatch("help", &[]) {
        DispatchResult::Executed { output, .. } => {
            assert!(
                output.contains("Workspace:"),
                "help must show workspace label"
            );
            assert!(
                output.contains("Directory:"),
                "help must show directory label"
            );
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn help_groups_shell_builtins_and_native_commands() {
    let mut rt = rt();
    match rt.dispatch("help", &[]) {
        DispatchResult::Executed { output, .. } => {
            assert!(
                output.contains("Shell built-ins:"),
                "help must have a builtins section"
            );
            assert!(
                output.contains("ROY-native commands:"),
                "help must have a native section"
            );
            assert!(output.contains("ls"), "ls must appear in help");
            assert!(output.contains("read"), "read must appear in help");
            assert!(output.contains("check"), "check must appear in help");
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn help_aliases_roy_and_question_mark_produce_same_output() {
    let out_roy = match ShellRuntime::new(std::env::temp_dir()).dispatch("roy", &[]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            output
        }
        other => panic!("expected Executed, got {other:?}"),
    };
    let out_q = match ShellRuntime::new(std::env::temp_dir()).dispatch("?", &[]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            output
        }
        other => panic!("expected Executed, got {other:?}"),
    };
    assert_eq!(out_roy, out_q, "roy and ? must produce identical output");
}

// ── commands ──────────────────────────────────────────────────────────────────

#[test]
fn commands_lists_public_names_one_per_line() {
    let mut rt = rt();
    match rt.dispatch("commands", &[]) {
        DispatchResult::Executed {
            output, exit_code, ..
        } => {
            assert_eq!(exit_code, 0);
            let names: Vec<&str> = output.lines().collect();
            for required in &[
                "cd", "pwd", "env", "exit", "help", "commands", "ls", "read", "write", "check",
            ] {
                assert!(
                    names.contains(required),
                    "`commands` output must include `{required}`"
                );
            }
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

#[test]
fn commands_output_excludes_hidden_and_denied_names() {
    let mut rt = rt();
    match rt.dispatch("commands", &[]) {
        DispatchResult::Executed { output, .. } => {
            let names: Vec<&str> = output.lines().collect();
            for hidden in &["bash", "sh", "grep", "sudo"] {
                assert!(
                    !names.contains(hidden),
                    "`commands` must not list hidden/denied `{hidden}`"
                );
            }
        }
        other => panic!("expected Executed, got {other:?}"),
    }
}

// ── not_found ─────────────────────────────────────────────────────────────────

#[test]
fn not_found_error_hints_at_help() {
    let mut rt = rt();
    match rt.dispatch("totally_unknown_roy_xyz", &[]) {
        DispatchResult::NotFound { .. } => {
            let errors = rt.drain_errors();
            let combined = errors.join("\n");
            assert!(
                combined.contains("help"),
                "not_found error must mention `help`, got: {combined:?}"
            );
        }
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[test]
fn runtime_command_counts_match_registry_views() {
    let rt = rt();

    assert_eq!(rt.command_count(), rt.registry().len());
    assert_eq!(
        rt.public_command_count(),
        rt.registry().public_commands().len()
    );
    assert!(rt.command_count() > rt.public_command_count());
    assert!(rt.command_count() > 10);
    assert!(rt.public_command_count() > 5);
}
