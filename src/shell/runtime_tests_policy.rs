//! Tests for ShellRuntime dispatch policy:
//! compatibility traps, NotFound, prompt indicator, and transcript errors.

use crate::shell::{DispatchResult, ShellRuntime};

fn rt() -> ShellRuntime {
    ShellRuntime::new(std::env::temp_dir())
}

// ── compatibility traps ───────────────────────────────────────────────────────

#[test]
fn bash_denied_with_suggestion() {
    let mut rt = rt();
    match rt.dispatch("bash", &[]) {
        DispatchResult::Denied { command, suggestion } => {
            assert_eq!(command, "bash");
            let msg = suggestion.expect("bash denial must include a suggestion");
            assert!(msg.contains("ROY"), "suggestion must mention ROY");
        }
        other => panic!("expected Denied, got {other:?}"),
    }
}

#[test]
fn bash_denial_writes_to_error_transcript() {
    let mut rt = rt();
    rt.dispatch("bash", &[]);
    assert!(!rt.drain_errors().is_empty(), "denial must write to error transcript");
}

#[test]
fn grep_denied() {
    let mut rt = rt();
    assert!(matches!(
        rt.dispatch("grep", &["-r", "foo", "."]),
        DispatchResult::Denied { .. }
    ));
}

#[test]
fn curl_denied() {
    let mut rt = rt();
    assert!(matches!(
        rt.dispatch("curl", &["https://example.com"]),
        DispatchResult::Denied { .. }
    ));
}

#[test]
fn sudo_denied() {
    let mut rt = rt();
    assert!(matches!(
        rt.dispatch("sudo", &["rm", "-rf", "/"]),
        DispatchResult::Denied { .. }
    ));
}

#[test]
fn all_shell_fallbacks_denied() {
    let mut rt = rt();
    for shell in &["sh", "zsh", "fish", "csh"] {
        assert!(
            matches!(rt.dispatch(shell, &[]), DispatchResult::Denied { .. }),
            "{shell} must be denied"
        );
    }
}

// ── not found ─────────────────────────────────────────────────────────────────

#[test]
fn unknown_command_returns_not_found() {
    let mut rt = rt();
    match rt.dispatch("completely_unknown_cmd_xyz_12345", &[]) {
        DispatchResult::NotFound { command } => {
            assert_eq!(command, "completely_unknown_cmd_xyz_12345");
        }
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[test]
fn unknown_command_writes_error_to_transcript() {
    let mut rt = rt();
    rt.dispatch("unknown_xyz_99994", &[]);
    let errors = rt.drain_errors();
    assert!(!errors.is_empty());
    assert!(errors[0].contains("command not found"));
}

// ── prompt ────────────────────────────────────────────────────────────────────

#[test]
fn prompt_shows_success_indicator_after_zero_exit() {
    let mut rt = rt();
    rt.set_exit_status(0);
    assert!(rt.prompt().contains('\u{276f}'), "should show ❯ after success");
    assert!(!rt.prompt().contains('\u{2717}'));
}

#[test]
fn prompt_shows_error_indicator_after_nonzero_exit() {
    let mut rt = rt();
    rt.set_exit_status(1);
    assert!(rt.prompt().contains('\u{2717}'), "should show ✗ after failure");
    assert!(!rt.prompt().contains('\u{276f}'));
}

#[test]
fn prompt_contains_cwd() {
    let root = std::env::temp_dir();
    let rt = ShellRuntime::new(root.clone());
    assert!(rt.prompt().contains(root.to_str().unwrap()));
}
