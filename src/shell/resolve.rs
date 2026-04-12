// Live in tests; runtime wiring was simplified, so keep this module allowed
// until resolution is reintroduced as a first-class dispatch step.
#![allow(dead_code)]

use crate::commands::{schema::Backend, CommandRegistry};

/// Outcome of resolving a command name through the ROY command registry.
#[derive(Debug, PartialEq)]
pub enum ResolveOutcome {
    /// Command is a ROY built-in — `ShellRuntime` handles it via direct match.
    Builtin,
    /// Command is known but denied — compat trap or blocked.
    Denied { suggestion: Option<&'static str> },
    /// Command is registered as a ROY-native capability (execution pending TOOL-02+).
    RoyNative,
    /// Command is not in the registry.
    NotFound,
}

/// Pure, side-effect-free resolution of `name` through `registry`.
///
/// Does not write to any transcript buffer — callers do that based on the
/// outcome. This keeps the resolution logic independently testable.
pub fn resolve_command(registry: &CommandRegistry, name: &str) -> ResolveOutcome {
    match registry.resolve(name) {
        None => ResolveOutcome::NotFound,
        Some(schema) => match schema.backend {
            Backend::Builtin | Backend::AgentLaunch => ResolveOutcome::Builtin,
            Backend::RoyNative => ResolveOutcome::RoyNative,
            Backend::CompatTrap { suggestion } => ResolveOutcome::Denied {
                suggestion: Some(suggestion),
            },
            Backend::Blocked { reason } => ResolveOutcome::Denied {
                suggestion: Some(reason),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::CommandRegistry;

    fn reg() -> CommandRegistry {
        CommandRegistry::new()
    }

    #[test]
    fn bash_resolves_to_denied() {
        assert!(matches!(
            resolve_command(&reg(), "bash"),
            ResolveOutcome::Denied { .. }
        ));
    }

    #[test]
    fn bash_denial_has_suggestion() {
        match resolve_command(&reg(), "bash") {
            ResolveOutcome::Denied { suggestion } => {
                assert!(suggestion.is_some());
                assert!(suggestion.unwrap().contains("ROY"));
            }
            other => panic!("expected Denied, got {other:?}"),
        }
    }

    #[test]
    fn curl_resolves_to_denied() {
        assert!(matches!(
            resolve_command(&reg(), "curl"),
            ResolveOutcome::Denied { .. }
        ));
    }

    #[test]
    fn unknown_resolves_to_not_found() {
        assert_eq!(
            resolve_command(&reg(), "completely_unknown_xyz_99999"),
            ResolveOutcome::NotFound
        );
    }

    #[test]
    fn builtin_resolves_to_builtin() {
        assert_eq!(resolve_command(&reg(), "cd"), ResolveOutcome::Builtin);
        assert_eq!(resolve_command(&reg(), "pwd"), ResolveOutcome::Builtin);
        assert_eq!(resolve_command(&reg(), "help"), ResolveOutcome::Builtin);
    }
}
