use crate::denial::DenialResponse;
use crate::interceptor::Disposition;

/// A single policy rule.
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub description: String,
    /// Substring or prefix that triggers this rule.
    pub pattern: RulePattern,
    pub action: RuleAction,
    pub alternative: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RulePattern {
    /// Match if the command line contains this substring.
    Contains(String),
    /// Match if the command line starts with this prefix (after trimming).
    StartsWith(String),
    /// Match if the command line equals this string exactly (after trimming).
    Exact(String),
}

impl RulePattern {
    pub fn matches(&self, line: &str) -> bool {
        let trimmed = line.trim();
        match self {
            RulePattern::Contains(s) => trimmed.contains(s.as_str()),
            RulePattern::StartsWith(s) => trimmed.starts_with(s.as_str()),
            RulePattern::Exact(s) => trimmed == s.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuleAction {
    Deny,
    // Redirect will be added in CORE-01 when the command dispatch table exists.
}

/// Policy engine — evaluates an input line against a set of rules.
pub struct PolicyEngine {
    rules: Vec<Rule>,
}

impl PolicyEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Evaluate `line` against all rules. Returns the first matching disposition,
    /// or `Disposition::Passthrough` if no rule matches.
    pub fn evaluate(&self, line: &str) -> Disposition {
        for rule in &self.rules {
            if rule.pattern.matches(line) {
                match rule.action {
                    RuleAction::Deny => {
                        let mut denial = DenialResponse::new(line.trim(), &rule.description)
                            .with_rule_id(&rule.id);
                        if let Some(alt) = &rule.alternative {
                            denial = denial.with_alternative(alt);
                        }
                        return Disposition::Denied(denial);
                    },
                }
            }
        }
        Disposition::Passthrough
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deny_rule(id: &str, pattern: RulePattern, alt: Option<&str>) -> Rule {
        Rule {
            id: id.to_string(),
            description: format!("test rule {id}"),
            pattern,
            action: RuleAction::Deny,
            alternative: alt.map(str::to_string),
        }
    }

    #[test]
    fn no_rules_passes_through() {
        let engine = PolicyEngine::empty();
        assert!(matches!(engine.evaluate("ls -la"), Disposition::Passthrough));
    }

    #[test]
    fn deny_rule_matches_and_blocks() {
        let engine = PolicyEngine::new(vec![
            deny_rule("D001", RulePattern::StartsWith("rm -rf".to_string()), None),
        ]);
        let result = engine.evaluate("rm -rf /important");
        match result {
            Disposition::Denied(d) => {
                assert_eq!(d.rule_id.as_deref(), Some("D001"));
                assert!(d.blocked.contains("rm -rf"));
            },
            _ => panic!("expected Denied, got something else"),
        }
    }

    #[test]
    fn non_matching_command_passes_through() {
        let engine = PolicyEngine::new(vec![
            deny_rule("D001", RulePattern::StartsWith("rm -rf".to_string()), None),
        ]);
        assert!(matches!(engine.evaluate("ls -la"), Disposition::Passthrough));
    }

    #[test]
    fn alternative_included_in_denial() {
        let engine = PolicyEngine::new(vec![
            deny_rule("D002", RulePattern::Contains("grep -r".to_string()), Some("semmap cat")),
        ]);
        let result = engine.evaluate("grep -r foo .");
        match result {
            Disposition::Denied(d) => {
                assert_eq!(d.alternative.as_deref(), Some("semmap cat"));
            },
            _ => panic!("expected Denied"),
        }
    }

    #[test]
    fn first_matching_rule_wins() {
        let engine = PolicyEngine::new(vec![
            deny_rule("D001", RulePattern::StartsWith("bad".to_string()), Some("alt1")),
            deny_rule("D002", RulePattern::Contains("bad".to_string()), Some("alt2")),
        ]);
        // Both rules match "bad cmd" but D001 should fire first
        let result = engine.evaluate("bad cmd");
        match result {
            Disposition::Denied(d) => assert_eq!(d.rule_id.as_deref(), Some("D001")),
            _ => panic!("expected Denied"),
        }
    }
}
