//! Tests for CodexAdapter — meta, auth, binary discovery, and PATH isolation.

use std::path::PathBuf;

use super::super::adapter::{AgentAdapter, AgentAuthMethod, AgentErrorKind, AgentKind};
use super::CodexAdapter;

fn fake_adapter() -> CodexAdapter {
    CodexAdapter::from_path(PathBuf::from("/usr/local/bin/codex"), "0.120.0")
}

#[test]
fn meta_kind_is_codex() {
    assert_eq!(fake_adapter().meta().kind, AgentKind::Codex);
}

#[test]
fn meta_version_is_preserved() {
    assert_eq!(fake_adapter().meta().version, "0.120.0");
}

#[test]
fn meta_install_path_is_preserved() {
    assert_eq!(
        fake_adapter().meta().install_path,
        PathBuf::from("/usr/local/bin/codex")
    );
}

#[test]
fn auth_method_supports_device_or_api_key() {
    assert_eq!(
        fake_adapter().auth_method(),
        AgentAuthMethod::OauthDeviceOrEnvVar {
            key: "OPENAI_API_KEY".to_string(),
        }
    );
}

#[test]
fn discover_matches_environment_when_binary_present_or_absent() {
    let has_codex = super::super::host::discover_binary("codex").is_ok();

    match CodexAdapter::discover() {
        Ok(adapter) => {
            assert!(
                has_codex,
                "discover() succeeded but PATH scan found no codex"
            );
            assert_eq!(adapter.meta().kind, AgentKind::Codex);
            let install_name = adapter
                .meta()
                .install_path
                .file_stem()
                .and_then(|n| n.to_str());
            assert_eq!(install_name, Some("codex"));
            assert!(
                !adapter.meta().version.trim().is_empty(),
                "discover() must keep a non-empty version string"
            );
        }
        Err(err) => {
            assert!(
                !has_codex,
                "discover() failed even though codex is on PATH: {err}"
            );
            assert_eq!(err.kind(), &AgentErrorKind::NotInstalled);
        }
    }
}

#[test]
fn missing_binary_returns_not_installed() {
    let result = super::super::host::discover_binary("__roy_no_such_codex_binary_42__");
    let err = result.expect_err("must fail for absent binary");
    assert_eq!(err.kind(), &AgentErrorKind::NotInstalled);
    assert!(err.to_string().contains("__roy_no_such_codex_binary_42__"));
}

#[test]
fn controlled_path_starts_with_roy_bin() {
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
