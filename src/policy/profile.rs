// Profiles are live via PolicyEngine; ShellRuntime wiring below.
#![allow(dead_code)]

use crate::commands::schema::RiskLevel;

/// What the policy engine decides about a particular command invocation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolicyPermission {
    /// Command may execute.
    Allow,
    /// Command is hard-blocked by this profile.
    Deny,
    /// Command may only execute after explicit user approval.
    RequireApproval,
}

/// A named set of rules governing what commands are permitted in a session.
///
/// Profiles are attached to sessions and workspaces — not to the app globally.
/// The evaluation order is:
/// 1. `blocked` hard-deny list (highest priority)
/// 2. `allowed` override-allow list (bypasses risk check)
/// 3. `max_risk` threshold — commands with risk > max_risk → Deny
/// 4. `default_permission` fallback
#[derive(Debug, Clone)]
pub struct PolicyProfile {
    /// Human-readable profile name (used in diagnostics and UI).
    pub name: String,
    /// Maximum risk level permitted without explicit override.
    pub max_risk: RiskLevel,
    /// What to do for commands not matched by any explicit rule.
    pub default_permission: PolicyPermission,
    /// Hard-deny list — these commands are always blocked regardless of risk.
    pub blocked: Vec<String>,
    /// Override-allow list — these commands bypass the risk threshold.
    pub allowed: Vec<String>,
}

impl PolicyProfile {
    /// Fully permissive profile — allows everything.
    ///
    /// Used when policy is a transparent pass-through and the registry
    /// compat-trap layer is the sole denial mechanism. Appropriate for
    /// v0.1 default where policy is in-path but not yet restrictive.
    pub fn permissive() -> Self {
        Self {
            name: "permissive".to_string(),
            max_risk: RiskLevel::Critical,
            default_permission: PolicyPermission::Allow,
            blocked: Vec::new(),
            allowed: Vec::new(),
        }
    }

    /// Read-only profile — denies anything above `Safe` risk.
    ///
    /// Allows inspection, help, and session-lifecycle commands, but denies
    /// broader mutation or system-level operations.
    pub fn read_only() -> Self {
        Self {
            name: "read_only".to_string(),
            max_risk: RiskLevel::Safe,
            default_permission: PolicyPermission::Deny,
            blocked: Vec::new(),
            allowed: vec![
                "pwd".to_string(),
                "env".to_string(),
                "printenv".to_string(),
                "help".to_string(),
                "roy".to_string(),
                "?".to_string(),
                "exit".to_string(),
                "quit".to_string(),
            ],
        }
    }

    /// Developer profile — allows most operations, denies `Critical` risk.
    ///
    /// Suitable for general development work where destructive system-level
    /// operations should require explicit approval.
    pub fn dev() -> Self {
        Self {
            name: "dev".to_string(),
            max_risk: RiskLevel::High,
            default_permission: PolicyPermission::Allow,
            blocked: Vec::new(),
            allowed: Vec::new(),
        }
    }

    /// True if `command` is on the hard-deny list.
    pub fn is_blocked(&self, command: &str) -> bool {
        self.blocked.iter().any(|b| b == command)
    }

    /// True if `command` is on the override-allow list.
    pub fn is_explicitly_allowed(&self, command: &str) -> bool {
        self.allowed.iter().any(|a| a == command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissive_allows_critical_risk() {
        assert!(RiskLevel::Critical <= PolicyProfile::permissive().max_risk);
    }

    #[test]
    fn read_only_max_risk_is_safe() {
        assert_eq!(PolicyProfile::read_only().max_risk, RiskLevel::Safe);
    }

    #[test]
    fn dev_max_risk_is_high() {
        assert_eq!(PolicyProfile::dev().max_risk, RiskLevel::High);
    }

    #[test]
    fn blocked_list_is_denied() {
        let mut p = PolicyProfile::permissive();
        p.blocked.push("dangerous_cmd".to_string());
        assert!(p.is_blocked("dangerous_cmd"));
        assert!(!p.is_blocked("safe_cmd"));
    }

    #[test]
    fn allowed_list_override() {
        let p = PolicyProfile::read_only();
        assert!(p.is_explicitly_allowed("pwd"));
        assert!(!p.is_explicitly_allowed("bash"));
    }

    #[test]
    fn read_only_allows_exit_aliases() {
        let p = PolicyProfile::read_only();
        assert!(p.is_explicitly_allowed("exit"));
        assert!(p.is_explicitly_allowed("quit"));
        assert!(p.is_explicitly_allowed("roy"));
        assert!(p.is_explicitly_allowed("?"));
    }

    #[test]
    fn permissive_has_no_blocked_by_default() {
        assert!(!PolicyProfile::permissive().is_blocked("anything"));
    }

    #[test]
    fn profile_is_clone() {
        let p = PolicyProfile::dev();
        let _ = p.clone();
    }
}
