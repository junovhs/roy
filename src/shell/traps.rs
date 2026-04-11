// Live via dispatch(); binary wiring pending SHEL-02.
#![allow(dead_code)]

use crate::commands::{schema::Backend, CommandRegistry};

/// Compatibility traps — resolved from the command registry instead of
/// being duplicated in a second static table.
///
/// This keeps denial messaging single-sourced in `commands/registry_data.rs`.
pub fn compat_trap_message(command: &str) -> Option<&'static str> {
    let registry = CommandRegistry::new();
    registry
        .resolve(command)
        .and_then(|schema| match schema.backend {
            Backend::CompatTrap { suggestion } => Some(suggestion),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_trap_resolves_from_registry() {
        let msg = compat_trap_message("bash").expect("bash should be a compat trap");
        assert!(msg.contains("ROY"));
    }

    #[test]
    fn pwd_is_not_a_trap() {
        assert!(compat_trap_message("pwd").is_none());
    }

    #[test]
    fn unknown_command_is_not_a_trap() {
        assert!(compat_trap_message("totally_unknown_roy_cmd").is_none());
    }
}
