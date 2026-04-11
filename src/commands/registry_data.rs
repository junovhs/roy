use crate::commands::schema::{Backend, CommandSchema, RiskLevel, Visibility};

const fn builtin(
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

const fn compat(
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

/// All commands known to ROY — built-ins, ROY-native (pending), and compat traps.
///
/// This is the explicit, data-driven substitution table. Every name that ROY
/// should handle — including names it intentionally denies — has an entry here.
/// Unknown names are `NotFound`; listed-but-denied names are `Denied`.
pub(super) static COMMANDS: &[CommandSchema] = &[
    builtin("cd", "change working directory", "cd [path]  — change to path, or stay in current dir if no arg", RiskLevel::Low, Visibility::Public),
    builtin("pwd", "print working directory", "pwd  — print current directory path", RiskLevel::Safe, Visibility::Public),
    builtin("env", "print controlled environment", "env [key]  — print all env vars, or filter by key substring", RiskLevel::Safe, Visibility::Public),
    builtin("printenv", "alias for env", "printenv [key]  — same as env", RiskLevel::Safe, Visibility::Hidden),
    builtin("exit", "exit the session", "exit [n]  — exit with code n (default 0)", RiskLevel::Low, Visibility::Public),
    builtin("quit", "alias for exit", "quit [n]  — same as exit", RiskLevel::Low, Visibility::Hidden),
    builtin("help", "show ROY help", "help  — list built-in commands and available surfaces", RiskLevel::Safe, Visibility::Public),
    builtin("roy", "alias for help", "roy  — same as help", RiskLevel::Safe, Visibility::Hidden),
    compat("bash", "Unix shell (blocked)", "Not available in ROY.", RiskLevel::Critical, "ROY does not provide a bash surface. Use ROY-native commands or `help`."),
    compat("sh", "POSIX shell (blocked)", "Not available in ROY.", RiskLevel::Critical, "ROY does not provide a sh surface. Use ROY-native commands or `help`."),
    compat("zsh", "Z shell (blocked)", "Not available in ROY.", RiskLevel::Critical, "ROY does not provide a zsh surface. Use ROY-native commands or `help`."),
    compat("fish", "fish shell (blocked)", "Not available in ROY.", RiskLevel::Critical, "ROY does not provide a fish surface. Use ROY-native commands or `help`."),
    compat("csh", "C shell (blocked)", "Not available in ROY.", RiskLevel::Critical, "ROY does not provide a csh surface. Use ROY-native commands or `help`."),
    compat("grep", "text search (blocked)", "Not available. Use ROY search commands (pending TOOL-02).", RiskLevel::Medium, "grep is not available here. ROY search commands are pending (TOOL-02)."),
    compat("rg", "ripgrep (blocked)", "Not available. Use ROY search commands (pending TOOL-02).", RiskLevel::Medium, "rg is not available here. ROY search commands are pending (TOOL-02)."),
    compat("find", "file search (blocked)", "Not available. Use ROY workspace inspection (pending TOOL-02).", RiskLevel::Medium, "find is not available here. ROY workspace inspection is pending (TOOL-02)."),
    compat("ls", "directory listing (blocked)", "Not available. Use `roy ls` (pending TOOL-02).", RiskLevel::Safe, "ls is not available here. Use `roy ls` (pending TOOL-02)."),
    compat("cat", "file read (blocked)", "Not available. Use `roy read <path>` (pending TOOL-02).", RiskLevel::Safe, "cat is not available here. Use `roy read <path>` (pending TOOL-02)."),
    compat("head", "file head (blocked)", "Not available. Use `roy read <path>` (pending TOOL-02).", RiskLevel::Safe, "head is not available here. Use `roy read <path>` (pending TOOL-02)."),
    compat("tail", "file tail (blocked)", "Not available. Use `roy read <path>` (pending TOOL-02).", RiskLevel::Safe, "tail is not available here. Use `roy read <path>` (pending TOOL-02)."),
    compat("rm", "file remove (blocked)", "Not available. Use ROY-native mutation commands (pending TOOL-02).", RiskLevel::High, "rm is not available here. Use ROY-native mutation commands (pending TOOL-02)."),
    compat("mv", "file move (blocked)", "Not available. Use ROY-native mutation commands (pending TOOL-02).", RiskLevel::High, "mv is not available here. Use ROY-native mutation commands (pending TOOL-02)."),
    compat("cp", "file copy (blocked)", "Not available. Use ROY-native mutation commands (pending TOOL-02).", RiskLevel::Medium, "cp is not available here. Use ROY-native mutation commands (pending TOOL-02)."),
    compat("curl", "HTTP client (blocked)", "Not available. Network commands are controlled by ROY policy.", RiskLevel::High, "curl is not available here. Network commands are controlled by ROY policy."),
    compat("wget", "HTTP download (blocked)", "Not available. Network commands are controlled by ROY policy.", RiskLevel::High, "wget is not available here. Network commands are controlled by ROY policy."),
    compat("sudo", "privilege escalation (blocked)", "Not available. ROY controls permissions through its policy engine.", RiskLevel::Critical, "sudo is not available here. ROY controls permissions through its policy engine."),
    compat("apt", "package manager (blocked)", "Not available. ROY manages its own environment.", RiskLevel::Critical, "apt is not available here. ROY manages its own environment."),
    compat("apt-get", "package manager (blocked)", "Not available. ROY manages its own environment.", RiskLevel::Critical, "apt-get is not available here. ROY manages its own environment."),
    compat("pip", "Python package manager (blocked)", "Not available. ROY manages its own environment.", RiskLevel::High, "pip is not available here. ROY manages its own environment."),
    compat("npm", "Node package manager (blocked)", "Not available. ROY manages its own environment.", RiskLevel::High, "npm is not available here. ROY manages its own environment."),
    compat("python", "Python interpreter (blocked)", "Not available. Use ROY-native scripting surfaces (pending).", RiskLevel::High, "python is not available here. Use ROY-native scripting surfaces (pending)."),
    compat("python3", "Python 3 interpreter (blocked)", "Not available. Use ROY-native scripting surfaces (pending).", RiskLevel::High, "python3 is not available here. Use ROY-native scripting surfaces (pending)."),
    compat("node", "Node.js interpreter (blocked)", "Not available. Use ROY-native scripting surfaces (pending).", RiskLevel::High, "node is not available here. Use ROY-native scripting surfaces (pending)."),
];
