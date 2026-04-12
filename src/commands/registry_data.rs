use crate::commands::schema::{Backend, CommandSchema, RiskLevel, Visibility};

#[path = "registry_data/builtins.rs"]
mod builtins_data;

#[path = "registry_data/compat.rs"]
mod compat_data;

pub(super) fn builtins() -> &'static [CommandSchema] {
    builtins_data::BUILTINS
}

pub(super) fn compat_traps() -> &'static [CommandSchema] {
    compat_data::COMPAT_TRAPS
}

pub(super) const fn builtin_schema(
    name: &'static str,
    purpose: &'static str,
    help_text: &'static str,
    risk_level: RiskLevel,
    visibility: Visibility,
) -> CommandSchema {
    CommandSchema {
        name,
        purpose,
        help_text,
        risk_level,
        visibility,
        backend: Backend::Builtin,
    }
}

pub(super) const fn agent_launch_schema(
    name: &'static str,
    purpose: &'static str,
    help_text: &'static str,
) -> CommandSchema {
    CommandSchema {
        name,
        purpose,
        help_text,
        risk_level: RiskLevel::High,
        visibility: Visibility::Public,
        backend: Backend::AgentLaunch,
    }
}

pub(super) const fn compat_schema(
    name: &'static str,
    purpose: &'static str,
    help_text: &'static str,
    risk_level: RiskLevel,
    suggestion: &'static str,
) -> CommandSchema {
    CommandSchema {
        name,
        purpose,
        help_text,
        risk_level,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap { suggestion },
    }
}
