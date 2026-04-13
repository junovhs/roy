//! Tests for ShellRuntime dispatch policy:
//! compatibility traps, NotFound, prompt indicator, transcript errors,
//! and policy engine integration.

use crate::policy::{PolicyEngine, PolicyProfile};
use crate::shell::{DispatchResult, ShellRuntime};

fn rt() -> ShellRuntime {
    ShellRuntime::new(std::env::temp_dir())
}

// ── compatibility traps ───────────────────────────────────────────────────────

#[test]
fn bash_denied_with_suggestion() {
    let mut rt = rt();
    match rt.dispatch("bash", &[]) {
        DispatchResult::Denied {
            command,
            suggestion,
            artifacts,
        } => {
            assert_eq!(command, "bash");
            let msg = suggestion.expect("bash denial must include a suggestion");
            assert!(msg.contains("ROY"), "suggestion must mention ROY");
            assert_eq!(
                artifacts.len(),
                1,
                "denials should promote a trace artifact"
            );
        }
        other => panic!("expected Denied, got {other:?}"),
    }
}

#[test]
fn bash_denial_writes_to_error_transcript() {
    let mut rt = rt();
    rt.dispatch("bash", &[]);
    assert!(
        !rt.drain_errors().is_empty(),
        "denial must write to error transcript"
    );
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
    assert!(
        rt.prompt().contains('\u{276f}'),
        "should show ❯ after success"
    );
    assert!(!rt.prompt().contains('\u{2717}'));
}

#[test]
fn prompt_shows_error_indicator_after_nonzero_exit() {
    let mut rt = rt();
    rt.set_exit_status(1);
    assert!(
        rt.prompt().contains('\u{2717}'),
        "should show ✗ after failure"
    );
    assert!(!rt.prompt().contains('\u{276f}'));
}

#[test]
fn prompt_contains_cwd() {
    let root = std::env::temp_dir();
    let rt = ShellRuntime::new(root.clone());
    let cwd = rt.env().cwd().display().to_string();
    assert!(rt.prompt().contains(&cwd));
}

// ── workspace boundary enforcement ───────────────────────────────────────────

#[test]
fn cd_outside_workspace_is_denied() {
    // Runtime workspace = /tmp; parent of /tmp is outside the boundary.
    let root = std::env::temp_dir().canonicalize().unwrap();
    let mut rt = ShellRuntime::new(root.clone());
    let outside = root.parent().unwrap().to_path_buf();
    // dispatch cd to outside path — should be denied, not CwdChanged
    let result = rt.dispatch("cd", &[outside.to_str().unwrap()]);
    assert!(
        !matches!(result, DispatchResult::CwdChanged { .. }),
        "cd outside workspace must not produce CwdChanged"
    );
}

#[test]
fn cd_outside_workspace_writes_error_to_transcript() {
    let root = std::env::temp_dir().canonicalize().unwrap();
    let mut rt = ShellRuntime::new(root.clone());
    let outside = root.parent().unwrap().to_path_buf();
    rt.dispatch("cd", &[outside.to_str().unwrap()]);
    assert!(
        !rt.drain_errors().is_empty(),
        "boundary violation must write to error transcript"
    );
}

#[test]
fn workspace_root_accessible_via_runtime() {
    let root = std::env::temp_dir().canonicalize().unwrap();
    let rt = ShellRuntime::new(root.clone());
    assert_eq!(rt.workspace_root(), root.as_path());
}

#[test]
fn runtime_reports_default_policy_name() {
    let rt = rt();
    assert_eq!(rt.policy_name(), "permissive");
}

#[test]
fn set_policy_updates_reported_policy_name() {
    let mut rt = rt();
    rt.set_policy(PolicyEngine::new(PolicyProfile::dev()));
    assert_eq!(rt.policy_name(), "dev");
}

// ── policy gate integration ───────────────────────────────────────────────────

#[test]
fn restrictive_policy_blocks_critical_command_via_dispatch() {
    let mut rt = rt();
    // dev profile: max_risk = High → sudo (Critical) is denied by policy gate
    rt.set_policy(PolicyEngine::new(PolicyProfile::dev()));
    assert!(matches!(
        rt.dispatch("sudo", &["rm", "-rf", "/"]),
        DispatchResult::Denied { .. }
    ));
}

#[test]
fn restrictive_policy_denial_writes_to_error_transcript() {
    let mut rt = rt();
    rt.set_policy(PolicyEngine::new(PolicyProfile::dev()));
    rt.dispatch("sudo", &[]);
    assert!(
        !rt.drain_errors().is_empty(),
        "policy denial must write to error transcript"
    );
}

#[test]
fn permissive_policy_still_allows_compat_trap_to_produce_registry_denial() {
    // Permissive policy lets sudo through the gate; registry compat trap denies it.
    // The suggestion text comes from the registry, not from policy.
    let mut rt = rt();
    match rt.dispatch("sudo", &[]) {
        DispatchResult::Denied { suggestion, .. } => {
            let msg = suggestion.expect("registry denial must include suggestion");
            assert!(
                msg.contains("policy"),
                "registry denial should mention policy"
            );
        }
        other => panic!("expected Denied, got {other:?}"),
    }
}
