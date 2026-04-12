use super::*;
use crate::policy::profile::{PolicyPermission, PolicyProfile};

fn permissive() -> PolicyEngine {
    PolicyEngine::new(PolicyProfile::permissive())
}

fn read_only() -> PolicyEngine {
    PolicyEngine::new(PolicyProfile::read_only())
}

fn dev() -> PolicyEngine {
    PolicyEngine::new(PolicyProfile::dev())
}

#[test]
fn allow_is_allowed() {
    assert!(PolicyOutcome::Allow.is_allowed());
    assert!(!PolicyOutcome::Allow.is_blocked());
}

#[test]
fn deny_is_blocked() {
    let d = PolicyOutcome::Deny {
        reason: "test".to_string(),
    };
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

#[test]
fn permissive_allows_safe() {
    assert!(permissive().is_allowed("pwd", RiskLevel::Safe));
}

#[test]
fn permissive_allows_critical() {
    assert!(permissive().is_allowed("rm", RiskLevel::Critical));
}

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

#[test]
fn dev_allows_high_risk() {
    assert!(dev().is_allowed("curl", RiskLevel::High));
}

#[test]
fn dev_denies_critical_risk() {
    assert!(!dev().is_allowed("sudo", RiskLevel::Critical));
}

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

#[test]
fn allowed_command_bypasses_risk_threshold() {
    let mut profile = PolicyProfile::read_only();
    profile.allowed.push("special".to_string());
    let engine = PolicyEngine::new(profile);
    assert!(matches!(
        engine.evaluate("special", RiskLevel::Critical),
        PolicyOutcome::Allow
    ));
}

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

#[test]
fn require_approval_profile_returns_approval_pending_above_threshold() {
    let profile = PolicyProfile {
        name: "strict".to_string(),
        max_risk: RiskLevel::Safe,
        default_permission: PolicyPermission::RequireApproval,
        blocked: Vec::new(),
        allowed: Vec::new(),
    };
    let engine = PolicyEngine::new(profile);
    assert!(matches!(
        engine.evaluate("curl", RiskLevel::High),
        PolicyOutcome::ApprovalPending { .. }
    ));
}

#[test]
fn require_approval_profile_allows_at_or_below_threshold() {
    let profile = PolicyProfile {
        name: "strict".to_string(),
        max_risk: RiskLevel::Safe,
        default_permission: PolicyPermission::RequireApproval,
        blocked: Vec::new(),
        allowed: Vec::new(),
    };
    let engine = PolicyEngine::new(profile);
    // At threshold: default_permission kicks in → RequireApproval
    assert!(matches!(
        engine.evaluate("pwd", RiskLevel::Safe),
        PolicyOutcome::ApprovalPending { .. }
    ));
}
