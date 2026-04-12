#![allow(dead_code)]

//! Concrete adapter for hosting Codex inside the ROY shell.
//!
//! Spawns `codex` with PATH prepended by ROY's bin dir so the embedded agent
//! encounters ROY's command world first, then streams stdout/stderr/exit
//! through the shared supervision contract.

use std::path::PathBuf;

use super::adapter::{
    AgentAdapter, AgentAuthMethod, AgentError, AgentHandle, AgentKind, AgentMeta, LaunchConfig,
};
use super::host::{discover_binary, launch_supervised_agent, probe_version};

/// Adapter for hosting the Codex CLI inside the ROY shell.
///
/// Use [`CodexAdapter::discover()`] in production to auto-detect the installed
/// `codex` binary, or [`CodexAdapter::from_path()`] in tests.
#[derive(Debug)]
pub struct CodexAdapter {
    meta: AgentMeta,
}

impl CodexAdapter {
    /// Locate `codex` in the system PATH and read its version string.
    ///
    /// Returns [`AgentError`] with kind `NotInstalled` if the binary is absent.
    pub fn discover() -> Result<Self, AgentError> {
        let install_path = discover_binary("codex")?;
        let version = probe_version(&install_path).unwrap_or_else(|_| "unknown".to_string());
        Ok(Self {
            meta: AgentMeta {
                kind: AgentKind::Codex,
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
                kind: AgentKind::Codex,
                version: version.into(),
                install_path,
            },
        }
    }
}

impl AgentAdapter for CodexAdapter {
    fn meta(&self) -> &AgentMeta {
        &self.meta
    }

    /// Codex supports device auth and can also run with an injected API key.
    ///
    /// When `OPENAI_API_KEY` is present in `LaunchConfig::env_overrides` it is
    /// forwarded to the subprocess and no interactive login is needed.
    fn auth_method(&self) -> AgentAuthMethod {
        AgentAuthMethod::OauthDeviceOrEnvVar {
            key: "OPENAI_API_KEY".to_string(),
        }
    }

    /// Spawn Codex with the shared controlled PATH and supervision threads.
    fn launch(&self, config: LaunchConfig) -> Result<AgentHandle, AgentError> {
        launch_supervised_agent(&self.meta, config, "codex")
    }
}

#[cfg(test)]
#[path = "codex_tests.rs"]
mod tests;
