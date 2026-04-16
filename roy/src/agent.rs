use std::sync::Mutex;
use crate::policy::{Rule, RuleAction, RulePattern};

/// Interface for an agent adapter.
///
/// Each adapter describes one kind of agent (Claude Code, Codex, etc.).
/// It tells the AgentHost how to detect when its agent is running in the
/// terminal, and provides extra policy rules to enforce while it is active.
///
/// Detection is heuristic: the adapter inspects lines of terminal OUTPUT
/// (the agent printing to stdout) to decide whether it is running. A false
/// positive activates stricter rules; a false negative means the base policy
/// applies. Neither is catastrophic. Refine `detect_active` as patterns
/// become clearer.
pub trait AgentAdapter: Send + Sync {
    /// Stable identifier used in logs and config.
    fn name(&self) -> &str;

    /// Return true if `line` looks like output from this agent.
    ///
    /// Called with trimmed lines from the terminal output stream.
    /// Implementations should be cheap (no I/O, no allocation).
    fn detect_active(&self, line: &str) -> bool;

    /// Additional policy rules to enforce while this agent is running.
    ///
    /// These are merged with (and take priority over) the base session rules.
    /// An empty vec means "use base policy as-is."
    fn extra_rules(&self) -> Vec<Rule>;
}

/// Adapter for Claude Code (`claude` CLI).
///
/// Claude Code writes recognizable markers to stdout: its spinner characters,
/// tool-call headers, and the `claude>` / `>` prompt prefix. We look for a
/// small set of high-signal patterns to detect when it is active.
pub struct ClaudeCodeAdapter;

impl AgentAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &str {
        "claude-code"
    }

    fn detect_active(&self, line: &str) -> bool {
        // Claude Code writes these to stdout during tool execution.
        // The patterns are conservative to avoid false positives.
        line.starts_with("claude>")
            || line.starts_with("> Claude")
            || line.contains("Tool: ")
            || line.contains("ToolUse:")
    }

    fn extra_rules(&self) -> Vec<Rule> {
        // Agent-mode policy: block operations that are risky under automation.
        // These supplement (and shadow) whatever is in roy.toml.
        vec![
            Rule {
                id: "A001".to_string(),
                description: "agent: no force-push".to_string(),
                pattern: RulePattern::Contains("--force".to_string()),
                action: RuleAction::Deny,
                alternative: Some("submit a PR for human review".to_string()),
            },
            Rule {
                id: "A002".to_string(),
                description: "agent: no recursive delete".to_string(),
                pattern: RulePattern::Contains("rm -rf".to_string()),
                action: RuleAction::Deny,
                alternative: Some("move files to a temp dir, or ask the user".to_string()),
            },
            Rule {
                id: "A003".to_string(),
                description: "agent: no production deploys".to_string(),
                pattern: RulePattern::Contains("deploy --prod".to_string()),
                action: RuleAction::Deny,
                alternative: Some("deploy to staging first, then ask the user to promote".to_string()),
            },
        ]
    }
}

/// Tracks which agent (if any) is currently active in this terminal session.
///
/// `AgentHost` inspects lines of terminal output (not input) to detect
/// when a known agent starts producing output. Once an agent is detected
/// it stays active until the session is reset — agents do not cleanly signal
/// when they exit, so staying active is the safe default.
///
/// In practice the host is consulted by `RoySession` to determine whether
/// to merge the active adapter's extra rules into the evaluated rule set.
pub struct AgentHost {
    adapters: Vec<Box<dyn AgentAdapter>>,
    /// Name of the currently active adapter, or None if no agent detected.
    active: Mutex<Option<String>>,
}

impl AgentHost {
    /// Create a host with the given set of adapters.
    pub fn new(adapters: Vec<Box<dyn AgentAdapter>>) -> Self {
        Self {
            adapters,
            active: Mutex::new(None),
        }
    }

    /// Build the default host with all bundled adapters.
    pub fn default_adapters() -> Self {
        Self::new(vec![Box::new(ClaudeCodeAdapter)])
    }

    /// Observe a line of terminal output and update the active adapter.
    ///
    /// Call this from the terminal output path (shell → display) whenever
    /// a line is available. Returns the adapter name that became active,
    /// or `None` if no change.
    pub fn observe(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        for adapter in &self.adapters {
            if adapter.detect_active(trimmed) {
                let mut active = self.active.lock().unwrap_or_else(|e| e.into_inner());
                if active.as_deref() != Some(adapter.name()) {
                    let name = adapter.name().to_string();
                    *active = Some(name.clone());
                    log::info!("[ROY] agent detected: {name}");
                    return Some(name);
                }
                break;
            }
        }
        None
    }

    /// Return the name of the currently active agent, if any.
    pub fn active_agent(&self) -> Option<String> {
        self.active.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// Return true if any agent is currently detected as active.
    pub fn is_agent_active(&self) -> bool {
        self.active.lock().unwrap_or_else(|e| e.into_inner()).is_some()
    }

    /// Collect the extra rules from the active adapter, if any.
    ///
    /// Called by `RoySession` to build the merged policy.
    pub fn active_extra_rules(&self) -> Vec<Rule> {
        let active = self.active.lock().unwrap_or_else(|e| e.into_inner());
        match active.as_deref() {
            None => vec![],
            Some(name) => {
                self.adapters
                    .iter()
                    .find(|a| a.name() == name)
                    .map(|a| a.extra_rules())
                    .unwrap_or_default()
            },
        }
    }

    /// Reset the active agent (e.g., on session restart).
    pub fn reset(&self) {
        let mut active = self.active.lock().unwrap_or_else(|e| e.into_inner());
        *active = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeAdapter {
        marker: &'static str,
    }

    impl AgentAdapter for FakeAdapter {
        fn name(&self) -> &str {
            self.marker
        }

        fn detect_active(&self, line: &str) -> bool {
            line.contains(self.marker)
        }

        fn extra_rules(&self) -> Vec<Rule> {
            vec![Rule {
                id: "F001".to_string(),
                description: "fake deny".to_string(),
                pattern: RulePattern::Exact("forbidden".to_string()),
                action: RuleAction::Deny,
                alternative: None,
            }]
        }
    }

    fn host_with_fake(marker: &'static str) -> AgentHost {
        AgentHost::new(vec![Box::new(FakeAdapter { marker })])
    }

    #[test]
    fn no_agent_active_initially() {
        let host = host_with_fake("MARKER");
        assert!(!host.is_agent_active());
        assert!(host.active_agent().is_none());
    }

    #[test]
    fn observe_activates_matching_adapter() {
        let host = host_with_fake("MARKER");
        let result = host.observe("some line with MARKER in it");
        assert_eq!(result.as_deref(), Some("MARKER"));
        assert!(host.is_agent_active());
        assert_eq!(host.active_agent().as_deref(), Some("MARKER"));
    }

    #[test]
    fn observe_no_match_returns_none() {
        let host = host_with_fake("MARKER");
        let result = host.observe("unrelated line");
        assert!(result.is_none());
        assert!(!host.is_agent_active());
    }

    #[test]
    fn observe_already_active_returns_none_on_second_detection() {
        let host = host_with_fake("MARKER");
        host.observe("line with MARKER");
        let second = host.observe("another MARKER line");
        assert!(second.is_none(), "already active: no new event");
    }

    #[test]
    fn active_extra_rules_returns_adapter_rules_when_active() {
        let host = host_with_fake("MARKER");
        host.observe("line with MARKER");
        let rules = host.active_extra_rules();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, "F001");
    }

    #[test]
    fn active_extra_rules_empty_when_no_agent() {
        let host = host_with_fake("MARKER");
        assert!(host.active_extra_rules().is_empty());
    }

    #[test]
    fn reset_clears_active_agent() {
        let host = host_with_fake("MARKER");
        host.observe("line with MARKER");
        assert!(host.is_agent_active());
        host.reset();
        assert!(!host.is_agent_active());
    }

    #[test]
    fn claude_code_adapter_detects_tool_line() {
        let adapter = ClaudeCodeAdapter;
        assert!(adapter.detect_active("Tool: bash"));
        assert!(adapter.detect_active("claude> ls"));
    }

    #[test]
    fn claude_code_adapter_no_false_positive_on_plain_text() {
        let adapter = ClaudeCodeAdapter;
        assert!(!adapter.detect_active("just a regular line"));
        assert!(!adapter.detect_active("git commit -m 'fix'"));
    }

    #[test]
    fn claude_code_extra_rules_not_empty() {
        let adapter = ClaudeCodeAdapter;
        let rules = adapter.extra_rules();
        assert!(!rules.is_empty(), "claude-code must ship with agent-mode rules");
        // Verify A001 (force-push) and A002 (rm -rf) are present.
        let ids: Vec<&str> = rules.iter().map(|r| r.id.as_str()).collect();
        assert!(ids.contains(&"A001"), "must deny --force");
        assert!(ids.contains(&"A002"), "must deny rm -rf");
    }
}
