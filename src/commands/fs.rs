use crate::capabilities::{CapabilityRequest, FsCapability};

use super::schema::{Backend, CommandSchema, RiskLevel, Visibility};

static FS_COMMANDS: &[CommandSchema] = &[
    CommandSchema {
        name: "ls",
        purpose: "list workspace files and directories",
        help_text: "ls [path]     list entries within the workspace",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Public,
        backend: Backend::RoyNative,
    },
    CommandSchema {
        name: "read",
        purpose: "read a workspace file or internal schema contract",
        help_text: "read <path> | read schema <name>   print a workspace file or schema contract",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Public,
        backend: Backend::RoyNative,
    },
    CommandSchema {
        name: "write",
        purpose: "overwrite or create a workspace file",
        help_text: "write <path> <text>   overwrite or create a file",
        risk_level: RiskLevel::Medium,
        visibility: Visibility::Public,
        backend: Backend::RoyNative,
    },
];

pub(crate) fn native_commands() -> &'static [CommandSchema] {
    FS_COMMANDS
}

pub(crate) fn parse_request(
    name: &str,
    args: &[&str],
) -> Option<Result<CapabilityRequest, String>> {
    match name {
        "ls" => Some(Ok(CapabilityRequest::Fs(FsCapability::ListDir {
            path: args.first().map(|value| (*value).to_string()),
        }))),
        "read" => Some(match args {
            [path] => Ok(CapabilityRequest::Fs(FsCapability::ReadFile {
                path: (*path).to_string(),
            })),
            _ => Err("usage: read <path>".to_string()),
        }),
        "write" => Some(match args {
            [path, contents] => Ok(CapabilityRequest::Fs(FsCapability::WriteFile {
                path: (*path).to_string(),
                contents: (*contents).to_string(),
            })),
            _ => Err("usage: write <path> <text>".to_string()),
        }),
        _ => None,
    }
}
