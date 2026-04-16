use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::policy::{Rule, RuleAction, RulePattern};

/// Top-level roy.toml configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyConfig {
    /// Global enable/disable switch. When false, interceptor is not installed.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Path for session artifacts (logs, denial records). Defaults to ~/.roy/sessions.
    #[serde(default)]
    pub session_dir: Option<PathBuf>,

    /// Policy section — defines what is allowed, denied, or redirected.
    #[serde(default)]
    pub policy: PolicyConfig,
}

fn default_true() -> bool {
    true
}

impl Default for RoyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            session_dir: None,
            policy: PolicyConfig::default(),
        }
    }
}

/// Policy rule configuration from roy.toml.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Rules evaluated in order. First match wins.
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

/// A single policy rule in roy.toml format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Rule identifier used in denial logs.
    pub id: String,
    /// Human-readable description / reason shown in denials.
    pub description: String,
    /// Match condition.
    #[serde(flatten)]
    pub pattern: PatternConfig,
    /// What to do when the pattern matches.
    pub action: ActionConfig,
    /// Owned alternative to show in the denial message.
    pub alternative: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternConfig {
    /// Matches if the command line contains this substring.
    Contains(String),
    /// Matches if the command line starts with this prefix (after trim).
    StartsWith(String),
    /// Matches if the command line equals this string exactly (after trim).
    Exact(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionConfig {
    Deny,
}

impl RuleConfig {
    /// Convert to a runtime `Rule`.
    pub fn into_rule(self) -> Rule {
        let pattern = match self.pattern {
            PatternConfig::Contains(s) => RulePattern::Contains(s),
            PatternConfig::StartsWith(s) => RulePattern::StartsWith(s),
            PatternConfig::Exact(s) => RulePattern::Exact(s),
        };
        let action = match self.action {
            ActionConfig::Deny => RuleAction::Deny,
        };
        Rule {
            id: self.id,
            description: self.description,
            pattern,
            action,
            alternative: self.alternative,
        }
    }
}

/// Load `roy.toml` from the given path. Returns `RoyConfig::default()` if the
/// file does not exist (ROY enabled, no rules).
///
/// Returns an error if the file exists but fails to parse.
pub fn load(path: &Path) -> Result<RoyConfig, ConfigError> {
    if !path.exists() {
        return Ok(RoyConfig::default());
    }
    let text = std::fs::read_to_string(path).map_err(ConfigError::Io)?;
    toml::from_str(&text).map_err(ConfigError::Parse)
}

/// Load from a default location: `<project_root>/roy.toml` falling back to
/// `~/.config/roy/roy.toml`.
pub fn load_default() -> Result<RoyConfig, ConfigError> {
    // 1. Workspace-local roy.toml
    if let Ok(cwd) = std::env::current_dir() {
        let local = cwd.join("roy.toml");
        if local.exists() {
            return load(&local);
        }
    }
    // 2. XDG config fallback
    if let Some(home) = std::env::var_os("HOME") {
        let xdg = PathBuf::from(home).join(".config/roy/roy.toml");
        if xdg.exists() {
            return load(&xdg);
        }
    }
    Ok(RoyConfig::default())
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "IO error reading roy.toml: {e}"),
            ConfigError::Parse(e) => write!(f, "Parse error in roy.toml: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TOML: &str = r#"
enabled = true
session_dir = "/tmp/roy-sessions"

[[policy.rules]]
id = "D001"
description = "no rm -rf"
starts_with = "rm -rf"
action = "deny"
alternative = "use trash-cli"

[[policy.rules]]
id = "D002"
description = "use semmap cat not grep"
contains = "grep -r"
action = "deny"
alternative = "semmap cat <file>"
"#;

    #[test]
    fn parses_valid_config() {
        let cfg: RoyConfig = toml::from_str(EXAMPLE_TOML).expect("valid toml");
        assert!(cfg.enabled);
        assert_eq!(cfg.policy.rules.len(), 2);
        assert_eq!(cfg.policy.rules[0].id, "D001");
        assert_eq!(cfg.policy.rules[1].id, "D002");
    }

    #[test]
    fn default_when_no_rules() {
        let cfg: RoyConfig = toml::from_str("enabled = true").expect("valid");
        assert!(cfg.policy.rules.is_empty());
    }

    #[test]
    fn into_rule_converts_correctly() {
        let rule_cfg = RuleConfig {
            id: "T001".to_string(),
            description: "test".to_string(),
            pattern: PatternConfig::StartsWith("bad".to_string()),
            action: ActionConfig::Deny,
            alternative: Some("good".to_string()),
        };
        let rule = rule_cfg.into_rule();
        assert_eq!(rule.id, "T001");
        assert!(rule.pattern.matches("bad command"));
        assert!(!rule.pattern.matches("other"));
        assert_eq!(rule.alternative.as_deref(), Some("good"));
    }

    #[test]
    fn load_returns_default_when_file_missing() {
        let cfg = load(Path::new("/nonexistent/roy.toml")).expect("default on missing");
        assert!(cfg.enabled);
        assert!(cfg.policy.rules.is_empty());
    }

    #[test]
    fn contains_pattern_matches_substring() {
        let rule_cfg = RuleConfig {
            id: "C001".to_string(),
            description: "contains check".to_string(),
            pattern: PatternConfig::Contains("grep".to_string()),
            action: ActionConfig::Deny,
            alternative: None,
        };
        let rule = rule_cfg.into_rule();
        assert!(rule.pattern.matches("grep -r foo ."));
        assert!(!rule.pattern.matches("ls -la"));
    }
}
