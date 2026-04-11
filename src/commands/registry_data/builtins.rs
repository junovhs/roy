use crate::commands::schema::{CommandSchema, RiskLevel, Visibility};

use super::builtin_schema;

pub(super) static BUILTINS: &[CommandSchema] = &[
    builtin_schema(
        "cd",
        "change working directory",
        "cd [path]    change to path, or stay in current dir if no arg",
        RiskLevel::Low,
        Visibility::Public,
    ),
    builtin_schema(
        "pwd",
        "print working directory",
        "pwd          print current directory path",
        RiskLevel::Safe,
        Visibility::Public,
    ),
    builtin_schema(
        "env",
        "print controlled environment",
        "env [key]    print all env vars, or filter by key substring",
        RiskLevel::Safe,
        Visibility::Public,
    ),
    builtin_schema(
        "printenv",
        "alias for env",
        "printenv [key]    same as env",
        RiskLevel::Safe,
        Visibility::Hidden,
    ),
    builtin_schema(
        "exit",
        "exit the session",
        "exit [n]     exit with code n (default 0)",
        RiskLevel::Low,
        Visibility::Public,
    ),
    builtin_schema(
        "quit",
        "alias for exit",
        "quit [n]     same as exit",
        RiskLevel::Low,
        Visibility::Hidden,
    ),
    builtin_schema(
        "help",
        "show ROY help",
        "help         show this help",
        RiskLevel::Safe,
        Visibility::Public,
    ),
    builtin_schema(
        "roy",
        "alias for help",
        "roy          same as help",
        RiskLevel::Safe,
        Visibility::Hidden,
    ),
    builtin_schema(
        "?",
        "alias for help",
        "?            same as help",
        RiskLevel::Safe,
        Visibility::Hidden,
    ),
    builtin_schema(
        "commands",
        "list available ROY commands",
        "commands     list all available public commands (one per line)",
        RiskLevel::Safe,
        Visibility::Public,
    ),
];
