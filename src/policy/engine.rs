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
#[path = "engine_tests.rs"]
mod tests;
