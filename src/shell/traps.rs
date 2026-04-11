// Live via dispatch(); binary wiring pending SHEL-02.
#![allow(dead_code)]

/// Compatibility traps — well-known commands ROY explicitly blocks.
///
/// Returns an informative denial redirecting the agent toward the
/// ROY-native alternative. This is the mechanism that retrains habits
/// without a prompt lecture every turn.
pub static COMPAT_TRAPS: &[(&str, &str)] = &[
    ("bash",    "ROY does not provide a bash surface. Use ROY-native commands or `help`."),
    ("sh",      "ROY does not provide a sh surface. Use ROY-native commands or `help`."),
    ("zsh",     "ROY does not provide a zsh surface. Use ROY-native commands or `help`."),
    ("fish",    "ROY does not provide a fish surface. Use ROY-native commands or `help`."),
    ("csh",     "ROY does not provide a csh surface. Use ROY-native commands or `help`."),
    ("grep",    "grep is not available here. ROY search commands are pending (TOOL-02)."),
    ("rg",      "rg is not available here. ROY search commands are pending (TOOL-02)."),
    ("find",    "find is not available here. ROY workspace inspection is pending (TOOL-02)."),
    ("ls",      "ls is not available here. Use `roy ls` (pending TOOL-02)."),
    ("cat",     "cat is not available here. Use `roy read <path>` (pending TOOL-02)."),
    ("head",    "head is not available here. Use `roy read <path>` (pending TOOL-02)."),
    ("tail",    "tail is not available here. Use `roy read <path>` (pending TOOL-02)."),
    ("rm",      "rm is not available here. Use ROY-native mutation commands (pending TOOL-02)."),
    ("mv",      "mv is not available here. Use ROY-native mutation commands (pending TOOL-02)."),
    ("cp",      "cp is not available here. Use ROY-native mutation commands (pending TOOL-02)."),
    ("curl",    "curl is not available here. Network commands are controlled by ROY policy."),
    ("wget",    "wget is not available here. Network commands are controlled by ROY policy."),
    ("sudo",    "sudo is not available here. ROY controls permissions through its policy engine."),
    ("apt",     "apt is not available here. ROY manages its own environment."),
    ("apt-get", "apt-get is not available here. ROY manages its own environment."),
    ("pip",     "pip is not available here. ROY manages its own environment."),
    ("npm",     "npm is not available here. ROY manages its own environment."),
    ("python",  "python is not available here. Use ROY-native scripting surfaces (pending)."),
    ("python3", "python3 is not available here. Use ROY-native scripting surfaces (pending)."),
    ("node",    "node is not available here. Use ROY-native scripting surfaces (pending)."),
];
