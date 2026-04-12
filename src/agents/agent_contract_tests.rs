//! Contract-comparison tests for the generic AgentAdapter layer.
//!
//! Exercises both ClaudeCodeAdapter and CodexAdapter through the shared
//! `AgentAdapter` trait to prove:
//!   1. Both adapters satisfy the generic contract.
//!   2. Agent-specific assumptions (binary name, API key env var) are
//!      handled at the adapter edge and do not leak into the host layer.
//!   3. ROY needs no separate shell philosophy per agent.

use std::path::PathBuf;

use super::adapter::{AgentAdapter, AgentAuthMethod};
use super::claude_code::ClaudeCodeAdapter;
use super::codex::CodexAdapter;

fn fake_claude() -> ClaudeCodeAdapter {
    ClaudeCodeAdapter::from_path(PathBuf::from("/usr/local/bin/claude"), "1.0.0")
}

fn fake_codex() -> CodexAdapter {
    CodexAdapter::from_path(PathBuf::from("/usr/local/bin/codex"), "0.120.0")
}

// ── generic contract ──────────────────────────────────────────────────────────

/// Both adapters must be usable through `&dyn AgentAdapter`.
/// A failing impl would not compile, but this also guards ID values at the
/// generic call site — proving the contract is exercised polymorphically.
#[test]
fn adapters_are_interchangeable_via_trait_object() {
    fn agent_id(a: &dyn AgentAdapter) -> &str {
        a.meta().kind.id()
    }

    assert_eq!(agent_id(&fake_claude()), "claude-code");
    assert_eq!(agent_id(&fake_codex()), "codex");
}

#[test]
fn every_adapter_has_non_empty_version() {
    let adapters: [&dyn AgentAdapter; 2] = [&fake_claude(), &fake_codex()];
    for a in adapters {
        assert!(
            !a.meta().version.trim().is_empty(),
            "version must be non-empty for {:?}",
            a.meta().kind
        );
    }
}

#[test]
fn every_adapter_has_non_empty_install_path() {
    let adapters: [&dyn AgentAdapter; 2] = [&fake_claude(), &fake_codex()];
    for a in adapters {
        assert!(
            !a.meta().install_path.as_os_str().is_empty(),
            "install_path must be non-empty for {:?}",
            a.meta().kind
        );
    }
}

#[test]
fn adapter_ids_are_unique() {
    let adapters: [&dyn AgentAdapter; 2] = [&fake_claude(), &fake_codex()];
    let ids: Vec<&str> = adapters.iter().map(|a| a.meta().kind.id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        ids.len(),
        "every adapter must expose a unique kind id"
    );
}

// ── agent-specific assumptions stay at the edge ───────────────────────────────

/// Each adapter must use OauthDeviceOrEnvVar — the API key strategy is
/// uniform across both agents even though the key names differ.
#[test]
fn both_adapters_use_oauth_device_or_env_var() {
    let adapters: [&dyn AgentAdapter; 2] = [&fake_claude(), &fake_codex()];
    for a in adapters {
        assert!(
            matches!(a.auth_method(), AgentAuthMethod::OauthDeviceOrEnvVar { .. }),
            "{:?} must use OauthDeviceOrEnvVar",
            a.meta().kind
        );
    }
}

/// The API-key env var names are agent-specific and must differ, confirming
/// that Codex assumptions do not bleed into the Claude Code adapter or vice
/// versa.
#[test]
fn api_key_env_vars_are_distinct_per_adapter() {
    let claude_key = match fake_claude().auth_method() {
        AgentAuthMethod::OauthDeviceOrEnvVar { key } => key,
        other => panic!("expected OauthDeviceOrEnvVar, got {other:?}"),
    };
    let codex_key = match fake_codex().auth_method() {
        AgentAuthMethod::OauthDeviceOrEnvVar { key } => key,
        other => panic!("expected OauthDeviceOrEnvVar, got {other:?}"),
    };

    assert_ne!(
        claude_key, codex_key,
        "Claude Code and Codex must use different API key env vars"
    );
    assert_eq!(claude_key, "ANTHROPIC_API_KEY");
    assert_eq!(codex_key, "OPENAI_API_KEY");
}

/// Install paths must differ — each adapter resolves a distinct binary.
/// A shared binary path would indicate the adapters are not independent.
#[test]
fn install_paths_are_distinct_per_adapter() {
    assert_ne!(
        fake_claude().meta().install_path,
        fake_codex().meta().install_path,
        "Claude Code and Codex must resolve different install paths"
    );
}
