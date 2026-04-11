// Schemas are consulted at runtime; ROY-native wiring pending TOOL-02+.
#![allow(dead_code)]

/// How a command is executed once resolved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Backend {
    /// Built-in handled directly by `ShellRuntime` (cd, pwd, env, exit, help).
    Builtin,
    /// ROY-native capability — implementation pending TOOL-02+.
    RoyNative,
    /// Compatibility trap: blocked with an informative ROY-native suggestion.
    CompatTrap { suggestion: &'static str },
    /// Explicitly blocked, no ROY-native alternative yet.
    Blocked { reason: &'static str },
}

impl Backend {
    /// True if this backend produces a `Denied` dispatch result.
    pub fn is_denied(&self) -> bool {
        matches!(self, Self::CompatTrap { .. } | Self::Blocked { .. })
    }

    /// Suggestion text for denied backends, if any.
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            Self::CompatTrap { suggestion } => Some(suggestion),
            Self::Blocked { .. } | Self::Builtin | Self::RoyNative => None,
        }
    }
}

/// Risk classification — used by the policy layer (POL-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// No side-effects (reads, prints).
    Safe,
    /// Local mutation with bounded impact.
    Low,
    /// Broader local mutation or network reads.
    Medium,
    /// Destructive or network-write operations.
    High,
    /// System-wide or irreversible operations.
    Critical,
}

/// Whether a command appears in public help listings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    /// Listed in `help` output and discoverable by the agent.
    Public,
    /// Not listed; only appears when explicitly invoked (compat traps).
    Hidden,
}

/// Full metadata for one command in the ROY registry.
///
/// Every command — built-in, ROY-native, compat trap, or blocked — has a
/// schema. Schemas power help text, policy decisions, and UI display.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSchema {
    /// Primary command name (unique key in the registry).
    pub name: &'static str,
    /// One-line description of what this command does or why it is blocked.
    pub purpose: &'static str,
    /// Help text shown to the agent on request.
    pub help_text: &'static str,
    /// Risk classification for policy decisions.
    pub risk_level: RiskLevel,
    /// Whether the command is listed in public help.
    pub visibility: Visibility,
    /// How the command is executed.
    pub backend: Backend,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compat_bash() -> CommandSchema {
        CommandSchema {
            name: "bash",
            purpose: "Unix Bourne-Again Shell",
            help_text: "Not available in ROY.",
            risk_level: RiskLevel::Critical,
            visibility: Visibility::Hidden,
            backend: Backend::CompatTrap { suggestion: "Use ROY-native commands." },
        }
    }

    fn builtin_pwd() -> CommandSchema {
        CommandSchema {
            name: "pwd",
            purpose: "print working directory",
            help_text: "pwd  — print current directory path",
            risk_level: RiskLevel::Safe,
            visibility: Visibility::Public,
            backend: Backend::Builtin,
        }
    }

    #[test]
    fn compat_trap_is_denied() {
        assert!(compat_bash().backend.is_denied());
    }

    #[test]
    fn builtin_is_not_denied() {
        assert!(!builtin_pwd().backend.is_denied());
    }

    #[test]
    fn compat_trap_suggestion_is_some() {
        assert!(compat_bash().backend.suggestion().is_some());
    }

    #[test]
    fn builtin_suggestion_is_none() {
        assert!(builtin_pwd().backend.suggestion().is_none());
    }

    #[test]
    fn risk_level_ord() {
        assert!(RiskLevel::Safe < RiskLevel::Critical);
        assert!(RiskLevel::Low < RiskLevel::High);
    }

    #[test]
    fn schema_is_clone_and_partial_eq() {
        let s = builtin_pwd();
        assert_eq!(s.clone(), s);
    }
}
