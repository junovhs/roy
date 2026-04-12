use crate::commands::schema::{CommandSchema, RiskLevel};

use super::compat_schema;

pub(super) static COMPAT_TRAPS: &[CommandSchema] = &[
    // ── raw shells ────────────────────────────────────────────────────────────
    compat_schema(
        "bash",
        "Unix shell (blocked)",
        "Not available in ROY — use ROY-native commands.",
        RiskLevel::Critical,
        "ROY is the shell. Use `help` to discover available commands, or `commands` for the full list.",
    ),
    compat_schema(
        "sh",
        "POSIX shell (blocked)",
        "Not available in ROY — use ROY-native commands.",
        RiskLevel::Critical,
        "ROY is the shell. Use `help` to discover available commands, or `commands` for the full list.",
    ),
    compat_schema(
        "zsh",
        "Z shell (blocked)",
        "Not available in ROY — use ROY-native commands.",
        RiskLevel::Critical,
        "ROY is the shell. Use `help` to discover available commands, or `commands` for the full list.",
    ),
    compat_schema(
        "fish",
        "fish shell (blocked)",
        "Not available in ROY — use ROY-native commands.",
        RiskLevel::Critical,
        "ROY is the shell. Use `help` to discover available commands, or `commands` for the full list.",
    ),
    compat_schema(
        "csh",
        "C shell (blocked)",
        "Not available in ROY — use ROY-native commands.",
        RiskLevel::Critical,
        "ROY is the shell. Use `help` to discover available commands, or `commands` for the full list.",
    ),
    // ── file search ───────────────────────────────────────────────────────────
    compat_schema(
        "grep",
        "text search (blocked)",
        "Not available. Use `read <path>` to inspect file contents.",
        RiskLevel::Medium,
        "Use `ls [path]` to explore directories · `read <path>` to inspect a file's contents.",
    ),
    compat_schema(
        "rg",
        "ripgrep (blocked)",
        "Not available. Use `read <path>` to inspect file contents.",
        RiskLevel::Medium,
        "Use `ls [path]` to explore directories · `read <path>` to inspect a file's contents.",
    ),
    compat_schema(
        "find",
        "file search (blocked)",
        "Not available. Use `ls [path]` to explore workspace directories.",
        RiskLevel::Medium,
        "Use `ls [path]` to explore workspace directories · `pwd` to see current location.",
    ),
    // ── file read/write ───────────────────────────────────────────────────────
    compat_schema(
        "cat",
        "file read (blocked)",
        "Not available. Use `read <path>`.",
        RiskLevel::Safe,
        "Use `read <path>` to print a file's contents.",
    ),
    compat_schema(
        "head",
        "file head (blocked)",
        "Not available. Use `read <path>`.",
        RiskLevel::Safe,
        "Use `read <path>` to inspect a file — ROY reads the whole file; scroll or filter as needed.",
    ),
    compat_schema(
        "tail",
        "file tail (blocked)",
        "Not available. Use `read <path>`.",
        RiskLevel::Safe,
        "Use `read <path>` to inspect a file — ROY reads the whole file; scroll or filter as needed.",
    ),
    compat_schema(
        "rm",
        "file remove (blocked)",
        "Not available. ROY does not expose destructive remove.",
        RiskLevel::High,
        "Use `write <path> <text>` to overwrite a file · ROY intentionally has no destructive remove.",
    ),
    compat_schema(
        "mv",
        "file move (blocked)",
        "Not available. Use `write` to create the destination file.",
        RiskLevel::High,
        "Use `read <src>` then `write <dst> <text>` to move a file · ROY has no atomic rename.",
    ),
    compat_schema(
        "cp",
        "file copy (blocked)",
        "Not available. Use `read <src>` then `write <dst> <text>` to copy files.",
        RiskLevel::Medium,
        "Use `read <src>` to get the content, then `write <dst> <text>` to copy it.",
    ),
    // ── network ───────────────────────────────────────────────────────────────
    compat_schema(
        "curl",
        "HTTP client (blocked)",
        "Not available. Network commands are controlled by ROY policy.",
        RiskLevel::High,
        "Network access is controlled by policy — run `help` to see what network capabilities are exposed.",
    ),
    compat_schema(
        "wget",
        "HTTP download (blocked)",
        "Not available. Network commands are controlled by ROY policy.",
        RiskLevel::High,
        "Network access is controlled by policy — run `help` to see what network capabilities are exposed.",
    ),
    // ── privilege / system ────────────────────────────────────────────────────
    compat_schema(
        "sudo",
        "privilege escalation (blocked)",
        "Not available. ROY controls permissions through its policy engine.",
        RiskLevel::Critical,
        "Privilege escalation is not available — ROY's policy engine governs permissions. See `help`.",
    ),
    compat_schema(
        "apt",
        "package manager (blocked)",
        "Not available. ROY manages its own environment.",
        RiskLevel::Critical,
        "Package management is not available in ROY — the environment is managed by the ROY host.",
    ),
    compat_schema(
        "apt-get",
        "package manager (blocked)",
        "Not available. ROY manages its own environment.",
        RiskLevel::Critical,
        "Package management is not available in ROY — the environment is managed by the ROY host.",
    ),
    // ── interpreters ──────────────────────────────────────────────────────────
    compat_schema(
        "pip",
        "Python package manager (blocked)",
        "Not available. ROY manages its own environment.",
        RiskLevel::High,
        "Python package installation is not available — run `help` to see what scripting surfaces exist.",
    ),
    compat_schema(
        "npm",
        "Node package manager (blocked)",
        "Not available. ROY manages its own environment.",
        RiskLevel::High,
        "Node package management is not available — run `help` to see what scripting surfaces exist.",
    ),
    compat_schema(
        "python",
        "Python interpreter (blocked)",
        "Not available. Use ROY-native scripting surfaces.",
        RiskLevel::High,
        "Python is not available — ROY-native scripting surfaces are in development. Run `help` for now.",
    ),
    compat_schema(
        "python3",
        "Python 3 interpreter (blocked)",
        "Not available. Use ROY-native scripting surfaces.",
        RiskLevel::High,
        "Python is not available — ROY-native scripting surfaces are in development. Run `help` for now.",
    ),
    compat_schema(
        "node",
        "Node.js interpreter (blocked)",
        "Not available. Use ROY-native scripting surfaces.",
        RiskLevel::High,
        "Node is not available — ROY-native scripting surfaces are in development. Run `help` for now.",
    ),
];
