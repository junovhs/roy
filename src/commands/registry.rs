// Registry is live via resolve(); ShellRuntime wiring below keeps it reachable.
#![allow(dead_code)]

#[path = "registry_data.rs"]
mod registry_data;

use super::schema::{CommandSchema, Visibility};
use super::{fs, validation};

/// ROY command registry — the explicit, data-driven substitution table.
///
/// Resolves command names to their [`CommandSchema`], covering built-ins,
/// ROY-native commands (pending TOOL-02+), and compatibility traps.
/// Unknown names return `None` → `DispatchResult::NotFound`.
pub struct CommandRegistry;

impl CommandRegistry {
    /// Create the registry with the default ROY command table.
    pub fn new() -> Self {
        Self
    }

    /// Resolve a command name to its schema, or `None` if unknown.
    pub fn resolve(&self, name: &str) -> Option<&CommandSchema> {
        registry_data::builtins()
            .iter()
            .chain(fs::native_commands().iter())
            .chain(validation::native_commands().iter())
            .chain(registry_data::compat_traps().iter())
            .find(|s| s.name == name)
    }

    /// All commands visible in public help listings.
    pub fn public_commands(&self) -> Vec<&CommandSchema> {
        registry_data::builtins()
            .iter()
            .chain(fs::native_commands().iter())
            .chain(validation::native_commands().iter())
            .chain(registry_data::compat_traps().iter())
            .filter(|s| s.visibility == Visibility::Public)
            .collect()
    }

    /// Help lines for public commands, in registry order.
    pub fn public_help_lines(&self) -> Vec<&'static str> {
        self.public_commands()
            .into_iter()
            .map(|s| s.help_text)
            .collect()
    }

    /// Total number of known commands (public + hidden).
    pub fn len(&self) -> usize {
        registry_data::builtins().len()
            + fs::native_commands().len()
            + validation::native_commands().len()
            + registry_data::compat_traps().len()
    }

    /// True if no commands are registered (should never be true after `new`).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::schema::{Backend, RiskLevel};

    fn reg() -> CommandRegistry {
        CommandRegistry::new()
    }

    #[test]
    fn registry_is_not_empty() {
        assert!(!reg().is_empty());
    }

    #[test]
    fn resolve_builtin_cd() {
        let r = reg();
        let s = r.resolve("cd").expect("cd must be in registry");
        assert!(matches!(s.backend, Backend::Builtin));
    }

    #[test]
    fn resolve_builtin_pwd() {
        let r = reg();
        let s = r.resolve("pwd").expect("pwd must be in registry");
        assert!(matches!(s.backend, Backend::Builtin));
    }

    #[test]
    fn resolve_compat_trap_bash() {
        let r = reg();
        let s = r.resolve("bash").expect("bash must be in registry");
        assert!(s.backend.is_denied());
        assert!(s.backend.suggestion().is_some());
    }

    #[test]
    fn resolve_compat_trap_curl() {
        let r = reg();
        let s = r.resolve("curl").expect("curl must be in registry");
        assert!(matches!(s.backend, Backend::CompatTrap { .. }));
    }

    #[test]
    fn resolve_compat_trap_sudo() {
        let r = reg();
        let s = r.resolve("sudo").expect("sudo must be in registry");
        assert_eq!(s.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn resolve_unknown_returns_none() {
        assert!(reg().resolve("completely_unknown_xyz_99999").is_none());
    }

    #[test]
    fn public_commands_excludes_compat_traps() {
        let r = reg();
        let public = r.public_commands();
        assert!(!public.is_empty());
        for s in &public {
            assert!(
                !s.backend.is_denied(),
                "public command {} must not be denied",
                s.name
            );
        }
    }

    #[test]
    fn public_commands_includes_cd_pwd_env_exit_help() {
        let r = reg();
        let names: Vec<&str> = r.public_commands().iter().map(|s| s.name).collect();
        for must_be_public in &[
            "cd", "pwd", "env", "exit", "help", "ls", "read", "write", "check",
        ] {
            assert!(
                names.contains(must_be_public),
                "{must_be_public} must be public"
            );
        }
    }

    #[test]
    fn bash_is_hidden() {
        let r = reg();
        let s = r.resolve("bash").unwrap();
        assert_eq!(s.visibility, Visibility::Hidden);
    }

    #[test]
    fn public_help_lines_include_pwd() {
        let r = reg();
        let lines = r.public_help_lines();
        assert!(lines.iter().any(|line| line.contains("pwd")));
    }

    #[test]
    fn len_matches_all_registry_sources() {
        let r = reg();
        let expected = registry_data::builtins().len()
            + fs::native_commands().len()
            + validation::native_commands().len()
            + registry_data::compat_traps().len();

        assert_eq!(r.len(), expected);
    }

    #[test]
    fn total_commands_include_hidden_entries_beyond_public_listing() {
        let r = reg();
        let hidden = registry_data::builtins()
            .iter()
            .chain(fs::native_commands().iter())
            .chain(validation::native_commands().iter())
            .chain(registry_data::compat_traps().iter())
            .filter(|schema| schema.visibility == Visibility::Hidden)
            .count();

        assert!(r.len() > r.public_commands().len());
        assert_eq!(r.len() - r.public_commands().len(), hidden);
    }
}
