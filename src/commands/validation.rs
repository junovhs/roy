use crate::capabilities::{CapabilityRequest, ValidationCapability};

use super::schema::{Backend, CommandSchema, RiskLevel, Visibility};

static VALIDATION_COMMANDS: &[CommandSchema] = &[CommandSchema {
    name: "check",
    purpose: "run trusted workspace validation",
    help_text: "check         run cargo check in the current workspace",
    risk_level: RiskLevel::Low,
    visibility: Visibility::Public,
    backend: Backend::RoyNative,
}];

pub(crate) fn native_commands() -> &'static [CommandSchema] {
    VALIDATION_COMMANDS
}

pub(crate) fn parse_request(
    name: &str,
    args: &[&str],
) -> Option<Result<CapabilityRequest, String>> {
    match name {
        "check" => Some(if args.is_empty() {
            Ok(CapabilityRequest::Validation(
                ValidationCapability::CargoCheck,
            ))
        } else {
            Err("usage: check".to_string())
        }),
        _ => None,
    }
}
