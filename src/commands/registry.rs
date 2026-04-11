// Registry is live via resolve(); ShellRuntime wiring below keeps it reachable.
#![allow(dead_code)]

use super::schema::{Backend, CommandSchema, RiskLevel, Visibility};

/// All commands known to ROY — built-ins, ROY-native (pending), and compat traps.
///
/// This is the explicit, data-driven substitution table. Every name that ROY
/// should handle — including names it intentionally denies — has an entry here.
/// Unknown names are `NotFound`; listed-but-denied names are `Denied`.
static COMMANDS: &[CommandSchema] = &[
    // ── built-ins ─────────────────────────────────────────────────────────────
    CommandSchema {
        name: "cd",
        purpose: "change working directory",
        help_text: "cd [path]  — change to path, or stay in current dir if no arg",
        risk_level: RiskLevel::Low,
        visibility: Visibility::Public,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "pwd",
        purpose: "print working directory",
        help_text: "pwd  — print current directory path",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Public,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "env",
        purpose: "print controlled environment",
        help_text: "env [key]  — print all env vars, or filter by key substring",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Public,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "printenv",
        purpose: "alias for env",
        help_text: "printenv [key]  — same as env",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Hidden,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "exit",
        purpose: "exit the session",
        help_text: "exit [n]  — exit with code n (default 0)",
        risk_level: RiskLevel::Low,
        visibility: Visibility::Public,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "quit",
        purpose: "alias for exit",
        help_text: "quit [n]  — same as exit",
        risk_level: RiskLevel::Low,
        visibility: Visibility::Hidden,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "help",
        purpose: "show ROY help",
        help_text: "help  — list built-in commands and available surfaces",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Public,
        backend: Backend::Builtin,
    },
    CommandSchema {
        name: "roy",
        purpose: "alias for help",
        help_text: "roy  — same as help",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Hidden,
        backend: Backend::Builtin,
    },
    // ── compatibility traps — shells ──────────────────────────────────────────
    CommandSchema {
        name: "bash",
        purpose: "Unix shell (blocked)",
        help_text: "Not available in ROY.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "ROY does not provide a bash surface. Use ROY-native commands or `help`.",
        },
    },
    CommandSchema {
        name: "sh",
        purpose: "POSIX shell (blocked)",
        help_text: "Not available in ROY.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "ROY does not provide a sh surface. Use ROY-native commands or `help`.",
        },
    },
    CommandSchema {
        name: "zsh",
        purpose: "Z shell (blocked)",
        help_text: "Not available in ROY.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "ROY does not provide a zsh surface. Use ROY-native commands or `help`.",
        },
    },
    CommandSchema {
        name: "fish",
        purpose: "fish shell (blocked)",
        help_text: "Not available in ROY.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "ROY does not provide a fish surface. Use ROY-native commands or `help`.",
        },
    },
    CommandSchema {
        name: "csh",
        purpose: "C shell (blocked)",
        help_text: "Not available in ROY.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "ROY does not provide a csh surface. Use ROY-native commands or `help`.",
        },
    },
    // ── compatibility traps — filesystem ──────────────────────────────────────
    CommandSchema {
        name: "grep",
        purpose: "text search (blocked)",
        help_text: "Not available. Use ROY search commands (pending TOOL-02).",
        risk_level: RiskLevel::Medium,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "grep is not available here. ROY search commands are pending (TOOL-02).",
        },
    },
    CommandSchema {
        name: "rg",
        purpose: "ripgrep (blocked)",
        help_text: "Not available. Use ROY search commands (pending TOOL-02).",
        risk_level: RiskLevel::Medium,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "rg is not available here. ROY search commands are pending (TOOL-02).",
        },
    },
    CommandSchema {
        name: "find",
        purpose: "file search (blocked)",
        help_text: "Not available. Use ROY workspace inspection (pending TOOL-02).",
        risk_level: RiskLevel::Medium,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "find is not available here. ROY workspace inspection is pending (TOOL-02).",
        },
    },
    CommandSchema {
        name: "ls",
        purpose: "directory listing (blocked)",
        help_text: "Not available. Use `roy ls` (pending TOOL-02).",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "ls is not available here. Use `roy ls` (pending TOOL-02).",
        },
    },
    CommandSchema {
        name: "cat",
        purpose: "file read (blocked)",
        help_text: "Not available. Use `roy read <path>` (pending TOOL-02).",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "cat is not available here. Use `roy read <path>` (pending TOOL-02).",
        },
    },
    CommandSchema {
        name: "head",
        purpose: "file head (blocked)",
        help_text: "Not available. Use `roy read <path>` (pending TOOL-02).",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "head is not available here. Use `roy read <path>` (pending TOOL-02).",
        },
    },
    CommandSchema {
        name: "tail",
        purpose: "file tail (blocked)",
        help_text: "Not available. Use `roy read <path>` (pending TOOL-02).",
        risk_level: RiskLevel::Safe,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "tail is not available here. Use `roy read <path>` (pending TOOL-02).",
        },
    },
    CommandSchema {
        name: "rm",
        purpose: "file remove (blocked)",
        help_text: "Not available. Use ROY-native mutation commands (pending TOOL-02).",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "rm is not available here. Use ROY-native mutation commands (pending TOOL-02).",
        },
    },
    CommandSchema {
        name: "mv",
        purpose: "file move (blocked)",
        help_text: "Not available. Use ROY-native mutation commands (pending TOOL-02).",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "mv is not available here. Use ROY-native mutation commands (pending TOOL-02).",
        },
    },
    CommandSchema {
        name: "cp",
        purpose: "file copy (blocked)",
        help_text: "Not available. Use ROY-native mutation commands (pending TOOL-02).",
        risk_level: RiskLevel::Medium,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "cp is not available here. Use ROY-native mutation commands (pending TOOL-02).",
        },
    },
    // ── compatibility traps — network ─────────────────────────────────────────
    CommandSchema {
        name: "curl",
        purpose: "HTTP client (blocked)",
        help_text: "Not available. Network commands are controlled by ROY policy.",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "curl is not available here. Network commands are controlled by ROY policy.",
        },
    },
    CommandSchema {
        name: "wget",
        purpose: "HTTP download (blocked)",
        help_text: "Not available. Network commands are controlled by ROY policy.",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "wget is not available here. Network commands are controlled by ROY policy.",
        },
    },
    // ── compatibility traps — privilege / package management ──────────────────
    CommandSchema {
        name: "sudo",
        purpose: "privilege escalation (blocked)",
        help_text: "Not available. ROY controls permissions through its policy engine.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "sudo is not available here. ROY controls permissions through its policy engine.",
        },
    },
    CommandSchema {
        name: "apt",
        purpose: "package manager (blocked)",
        help_text: "Not available. ROY manages its own environment.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "apt is not available here. ROY manages its own environment.",
        },
    },
    CommandSchema {
        name: "apt-get",
        purpose: "package manager (blocked)",
        help_text: "Not available. ROY manages its own environment.",
        risk_level: RiskLevel::Critical,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "apt-get is not available here. ROY manages its own environment.",
        },
    },
    CommandSchema {
        name: "pip",
        purpose: "Python package manager (blocked)",
        help_text: "Not available. ROY manages its own environment.",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "pip is not available here. ROY manages its own environment.",
        },
    },
    CommandSchema {
        name: "npm",
        purpose: "Node package manager (blocked)",
        help_text: "Not available. ROY manages its own environment.",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "npm is not available here. ROY manages its own environment.",
        },
    },
    // ── compatibility traps — runtimes ────────────────────────────────────────
    CommandSchema {
        name: "python",
        purpose: "Python interpreter (blocked)",
        help_text: "Not available. Use ROY-native scripting surfaces (pending).",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "python is not available here. Use ROY-native scripting surfaces (pending).",
        },
    },
    CommandSchema {
        name: "python3",
        purpose: "Python 3 interpreter (blocked)",
        help_text: "Not available. Use ROY-native scripting surfaces (pending).",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "python3 is not available here. Use ROY-native scripting surfaces (pending).",
        },
    },
    CommandSchema {
        name: "node",
        purpose: "Node.js interpreter (blocked)",
        help_text: "Not available. Use ROY-native scripting surfaces (pending).",
        risk_level: RiskLevel::High,
        visibility: Visibility::Hidden,
        backend: Backend::CompatTrap {
            suggestion: "node is not available here. Use ROY-native scripting surfaces (pending).",
        },
    },
];

/// ROY command registry — the explicit, data-driven substitution table.
///
/// Resolves command names to their [`CommandSchema`], covering built-ins,
/// ROY-native commands (pending TOOL-02+), and compatibility traps.
/// Unknown names return `None` → `DispatchResult::NotFound`.
pub struct CommandRegistry {
    commands: &'static [CommandSchema],
}

impl CommandRegistry {
    /// Create the registry with the default ROY command table.
    pub fn new() -> Self {
        Self { commands: COMMANDS }
    }

    /// Resolve a command name to its schema, or `None` if unknown.
    pub fn resolve(&self, name: &str) -> Option<&CommandSchema> {
        self.commands.iter().find(|s| s.name == name)
    }

    /// All commands visible in public help listings.
    pub fn public_commands(&self) -> Vec<&CommandSchema> {
        self.commands
            .iter()
            .filter(|s| s.visibility == Visibility::Public)
            .collect()
    }

    /// Total number of known commands (public + hidden).
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// True if no commands are registered (should never be true after `new`).
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
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
                "public command {} must not be denied", s.name
            );
        }
    }

    #[test]
    fn public_commands_includes_cd_pwd_env_exit_help() {
        let r = reg();
        let names: Vec<&str> = r.public_commands().iter().map(|s| s.name).collect();
        for must_be_public in &["cd", "pwd", "env", "exit", "help"] {
            assert!(names.contains(must_be_public), "{must_be_public} must be public");
        }
    }

    #[test]
    fn bash_is_hidden() {
        let r = reg();
        let s = r.resolve("bash").unwrap();
        assert_eq!(s.visibility, Visibility::Hidden);
    }
}
