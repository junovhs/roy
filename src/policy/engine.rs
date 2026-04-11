// Live via ShellRuntime dispatch; dead_code suppressed at module level.
#![allow(dead_code)]

use crate::commands::schema::RiskLevel;

use super::profile::{PolicyPermission, PolicyProfile};

/// Typed outcome of a policy evaluation.
///
/// Callers map outcomes to session events and `DispatchResult` variants.
/// `TransformedResult` is reserved for future output-redaction features.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyOutcome {
    /// Command is permitted to execute.
    Allow,
    /// Command is denied by policy — include reason in denial message.
    Deny { reason: String },
    /// Command requires explicit user approval before execution.
    ApprovalPending { command: String, reason: String },
}

impl PolicyOutcome {
    /// True if this outcome allows execution.
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }

    /// True if this outcome blocks execution.
    pub fn is_blocked(&self) -> bool {
        !self.is_allowed()
    }
}

/// Policy engine — evaluates commands against a `PolicyProfile`.
///
/// Sits in the dispatch path between command resolution and execution.
/// A single engine is attached to each `ShellRuntime` instance and
/// evaluates every command before the runtime acts on it.
pub struct PolicyEngine {
    profile: PolicyProfile,
}

impl PolicyEngine {
    /// Create an engine with the given profile.
    pub fn new(profile: PolicyProfile) -> Self {
        Self { profile }
    }

    /// Evaluate `command` (with known `risk`) against the active profile.
    ///
    /// Evaluation order:
    /// 1. Hard-deny list → `Deny`
    /// 2. Override-allow list → `Allow` (bypasses risk threshold)
    /// 3. Risk threshold exceeded → `Deny` or `RequireApproval`
    /// 4. Default permission
    pub fn evaluate(&self, command: &str, risk: RiskLevel) -> PolicyOutcome {
        // 1. Hard-deny list
        if self.profile.is_blocked(command) {
            return PolicyOutcome::Deny {
                reason: format!("'{command}' is blocked by the '{name}' policy profile",
                    name = self.profile.name),
            };
        }

        // 2. Override-allow list
        if self.profile.is_explicitly_allowed(command) {
            return PolicyOutcome::Allow;
        }

        // 3. Risk threshold
        if risk > self.profile.max_risk {
            return match self.profile.default_permission {
                PolicyPermission::RequireApproval => PolicyOutcome::ApprovalPending {
                    command: command.to_string(),
                    reason: format!(
                        "risk level {risk:?} exceeds profile '{name}' threshold ({max:?}); requires approval",
                        name = self.profile.name,
                        max = self.profile.max_risk,
                    ),
                },
                _ => PolicyOutcome::Deny {
                    reason: format!(
                        "risk level {risk:?} exceeds profile '{name}' threshold ({max:?})",
                        name = self.profile.name,
                        max = self.profile.max_risk,
                    ),
                },
            };
        }

        // 4. Default
        match self.profile.default_permission {
            PolicyPermission::Allow => PolicyOutcome::Allow,
            PolicyPermission::Deny => PolicyOutcome::Deny {
                reason: format!("denied by default rule in profile '{}'", self.profile.name),
            },
            PolicyPermission::RequireApproval => PolicyOutcome::ApprovalPending {
                command: command.to_string(),
                reason: format!(
                    "profile '{}' requires approval for all commands",
                    self.profile.name,
                ),
            },
        }
    }

    /// Convenience: true if `evaluate(command, risk)` would allow execution.
    pub fn is_allowed(&self, command: &str, risk: RiskLevel) -> bool {
        self.evaluate(command, risk).is_allowed()
    }

    /// Replace the active profile.
    pub fn set_profile(&mut self, profile: PolicyProfile) {
        self.profile = profile;
    }

    /// Active profile name.
    pub fn profile_name(&self) -> &str {
        &self.profile.name
    }
}

impl Default for PolicyEngine {
    /// Default engine uses the permissive profile — transparent pass-through.
    fn default() -> Self {
        Self::new(PolicyProfile::permissive())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::profile::PolicyProfile;

    fn permissive() -> PolicyEngine {
        PolicyEngine::new(PolicyProfile::permissive())
    }

    fn read_only() -> PolicyEngine {
        PolicyEngine::new(PolicyProfile::read_only())
    }

    fn dev() -> PolicyEngine {
        PolicyEngine::new(PolicyProfile::dev())
    }

    // ── PolicyOutcome helpers ─────────────────────────────────────────────────

    #[test]
    fn allow_is_allowed() {
        assert!(PolicyOutcome::Allow.is_allowed());
        assert!(!PolicyOutcome::Allow.is_blocked());
    }

    #[test]
    fn deny_is_blocked() {
        let d = PolicyOutcome::Deny { reason: "test".to_string() };
        assert!(!d.is_allowed());
        assert!(d.is_blocked());
    }

    #[test]
    fn approval_pending_is_blocked() {
        let a = PolicyOutcome::ApprovalPending {
            command: "cmd".to_string(),
            reason: "test".to_string(),
        };
        assert!(a.is_blocked());
    }

    // ── permissive profile ────────────────────────────────────────────────────

    #[test]
    fn permissive_allows_safe() {
        assert!(permissive().is_allowed("pwd", RiskLevel::Safe));
    }

    #[test]
    fn permissive_allows_critical() {
        assert!(permissive().is_allowed("rm", RiskLevel::Critical));
    }

    // ── read_only profile ─────────────────────────────────────────────────────

    #[test]
    fn read_only_allows_safe_commands_on_allow_list() {
        assert!(read_only().is_allowed("pwd", RiskLevel::Safe));
    }

    #[test]
    fn read_only_denies_medium_risk() {
        assert!(!read_only().is_allowed("grep", RiskLevel::Medium));
    }

    #[test]
    fn read_only_denies_critical_risk() {
        assert!(!read_only().is_allowed("bash", RiskLevel::Critical));
    }

    // ── dev profile ───────────────────────────────────────────────────────────

    #[test]
    fn dev_allows_high_risk() {
        assert!(dev().is_allowed("curl", RiskLevel::High));
    }

    #[test]
    fn dev_denies_critical_risk() {
        assert!(!dev().is_allowed("sudo", RiskLevel::Critical));
    }

    // ── hard-deny list ────────────────────────────────────────────────────────

    #[test]
    fn blocked_command_is_denied_regardless_of_risk() {
        let mut profile = PolicyProfile::permissive();
        profile.blocked.push("forbidden".to_string());
        let engine = PolicyEngine::new(profile);
        assert!(matches!(
            engine.evaluate("forbidden", RiskLevel::Safe),
            PolicyOutcome::Deny { .. }
        ));
    }

    // ── override-allow list ───────────────────────────────────────────────────

    #[test]
    fn allowed_command_bypasses_risk_threshold() {
        let mut profile = PolicyProfile::read_only();
        profile.allowed.push("special".to_string());
        let engine = PolicyEngine::new(profile);
        // read_only max_risk = Safe, but "special" is on allow list
        assert!(matches!(
            engine.evaluate("special", RiskLevel::Critical),
            PolicyOutcome::Allow
        ));
    }

    // ── profile management ────────────────────────────────────────────────────

    #[test]
    fn set_profile_changes_behavior() {
        let mut engine = PolicyEngine::new(PolicyProfile::permissive());
        assert!(engine.is_allowed("sudo", RiskLevel::Critical));
        engine.set_profile(PolicyProfile::dev());
        assert!(!engine.is_allowed("sudo", RiskLevel::Critical));
    }

    #[test]
    fn profile_name_matches() {
        let engine = PolicyEngine::new(PolicyProfile::dev());
        assert_eq!(engine.profile_name(), "dev");
    }

    #[test]
    fn default_engine_is_permissive() {
        let engine = PolicyEngine::default();
        assert_eq!(engine.profile_name(), "permissive");
    }
}
