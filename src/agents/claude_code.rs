#![allow(dead_code)]

//! Concrete adapter for hosting Claude Code inside the ROY shell.
//!
//! Spawns `claude` with PATH prepended by ROY's bin dir (so blocked commands
//! are absent before any OS fallback), then drives stdout/stderr/exit via
//! supervision threads whose events are collected via [`AgentHandle::drain_pending`].

use std::path::PathBuf;

use super::adapter::{
    AgentAdapter, AgentAuthMethod, AgentError, AgentHandle, AgentKind, AgentMeta, LaunchConfig,
};
use super::host::{discover_binary, launch_supervised_agent, probe_version};

// ── adapter ───────────────────────────────────────────────────────────────────

/// Adapter for hosting Claude Code inside the ROY shell.
///
/// Use [`ClaudeCodeAdapter::discover()`] in production to auto-detect the
/// installed `claude` binary, or [`ClaudeCodeAdapter::from_path()`] in tests.
#[derive(Debug)]
pub struct ClaudeCodeAdapter {
    meta: AgentMeta,
}

impl ClaudeCodeAdapter {
    /// Locate `claude` in the system PATH and read its version string.
    ///
    /// Returns [`AgentError`] with kind `NotInstalled` if the binary is absent.
    pub fn discover() -> Result<Self, AgentError> {
        let install_path = discover_binary("claude")?;
        let version = probe_version(&install_path).unwrap_or_else(|_| "unknown".to_string());
        Ok(Self {
            meta: AgentMeta {
                kind: AgentKind::ClaudeCode,
                version,
                install_path,
            },
        })
    }

    /// Build the adapter from a known binary path, bypassing auto-discovery.
    ///
    /// No filesystem check is performed — intended for tests.
    pub fn from_path(install_path: PathBuf, version: impl Into<String>) -> Self {
        Self {
            meta: AgentMeta {
                kind: AgentKind::ClaudeCode,
                version: version.into(),
                install_path,
            },
        }
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    fn meta(&self) -> &AgentMeta {
        &self.meta
    }

    /// Claude Code uses OAuth device flow for first-time authentication.
    ///
    /// When `ANTHROPIC_API_KEY` is present in `LaunchConfig::env_overrides`
    /// it is forwarded to the subprocess and no interactive login is needed.
    fn auth_method(&self) -> AgentAuthMethod {
        AgentAuthMethod::OauthDeviceOrEnvVar {
            key: "ANTHROPIC_API_KEY".to_string(),
        }
    }

    /// Spawn Claude Code with a controlled PATH and start supervision threads.
    ///
    /// Sets `PATH=<roy_bin>:<sys_PATH>`, `ROY_SESSION_ID`, and any caller
    /// `env_overrides`. Returns a handle with `AgentStarted` recorded; caller
    /// must poll [`AgentHandle::drain_pending`] to collect subsequent events.
    fn launch(&self, config: LaunchConfig) -> Result<AgentHandle, AgentError> {
        launch_supervised_agent(&self.meta, config, "claude")
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[path = "claude_code_tests.rs"]
mod tests;
