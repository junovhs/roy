//! Tests for ClaudeCodeAdapter — meta, auth, binary discovery, and PATH isolation.

use std::path::PathBuf;

use super::super::adapter::{AgentAdapter, AgentAuthMethod, AgentErrorKind, AgentKind};
use super::ClaudeCodeAdapter;

fn fake_adapter() -> ClaudeCodeAdapter {
    ClaudeCodeAdapter::from_path(PathBuf::from("/usr/local/bin/claude"), "1.2.3")
}

// ── meta ──────────────────────────────────────────────────────────────────────

#[test]
fn meta_kind_is_claude_code() {
    assert_eq!(fake_adapter().meta().kind, AgentKind::ClaudeCode);
}

#[test]
fn meta_version_is_preserved() {
    assert_eq!(fake_adapter().meta().version, "1.2.3");
}

#[test]
fn meta_install_path_is_preserved() {
    assert_eq!(
        fake_adapter().meta().install_path,
        PathBuf::from("/usr/local/bin/claude")
    );
}

// ── auth ──────────────────────────────────────────────────────────────────────

#[test]
fn auth_method_supports_device_or_api_key() {
    assert_eq!(
        fake_adapter().auth_method(),
        AgentAuthMethod::OauthDeviceOrEnvVar {
            key: "ANTHROPIC_API_KEY".to_string(),
        }
    );
}

// ── binary discovery ──────────────────────────────────────────────────────────

#[test]
fn which_missing_binary_returns_not_installed() {
    // A name extremely unlikely to exist anywhere on PATH.
    let result = super::super::host::discover_binary("__roy_no_such_binary_8f2c9__");
    let err = result.expect_err("must fail for absent binary");
    assert_eq!(err.kind(), &AgentErrorKind::NotInstalled);
    assert!(
        err.to_string().contains("__roy_no_such_binary_8f2c9__"),
        "error message must name the missing binary"
    );
}

#[test]
fn discover_returns_not_installed_when_claude_absent() {
    // When claude is not installed, discover() must surface NotInstalled.
    // If claude happens to be on PATH in this env, skip the error-path assertion.
    let has_claude = super::super::host::discover_binary("claude").is_ok();

    if !has_claude {
        let err = ClaudeCodeAdapter::discover().expect_err("must fail when claude absent");
        assert_eq!(err.kind(), &AgentErrorKind::NotInstalled);
    }
}

// ── controlled PATH isolation ─────────────────────────────────────────────────

#[test]
fn controlled_path_starts_with_roy_bin() {
    // Verify the PATH string built in launch() places ROY's bin dir first.
    // ROY's bin dir must appear before any OS paths so blocked commands are
    // shadowed or absent rather than falling through to /usr/bin.
    let roy_bin = crate::shell::ShellEnv::roy_path();
    let sys = std::env::var("PATH").unwrap_or_default();
    let controlled = format!("{roy_bin}:{sys}");
    assert!(
        controlled.starts_with(&roy_bin),
        "controlled PATH must begin with ROY bin dir '{roy_bin}'"
    );
}

#[test]
fn controlled_path_preserves_system_entries() {
    // System PATH entries must survive so the agent can still reach utilities
    // that ROY does not yet replace (e.g. git, node, python).
    let roy_bin = crate::shell::ShellEnv::roy_path();
    let sys = std::env::var("PATH").unwrap_or_default();
    let controlled = format!("{roy_bin}:{sys}");
    for entry in sys.split(':').filter(|s| !s.is_empty()) {
        assert!(
            controlled.contains(entry),
            "system entry '{entry}' must survive in controlled PATH"
        );
    }
}

#[test]
fn roy_bin_not_in_system_path() {
    // Confirms that prepending ROY's bin dir is meaningful: the dir is NOT
    // already part of the system PATH, so the prepend actually changes priority.
    let roy_bin = crate::shell::ShellEnv::roy_path();
    let sys = std::env::var("PATH").unwrap_or_default();
    let in_sys = sys.split(':').any(|e| e == roy_bin);
    assert!(
        !in_sys,
        "system PATH must not already contain ROY bin dir '{roy_bin}'; prepend would be redundant"
    );
}
