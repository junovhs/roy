use serde::{Deserialize, Serialize};

/// Structured denial event — printed inline to the terminal, written to session log.
///
/// A denial is not just an error. It is a teaching event (concept.md):
/// WHAT was blocked, WHY, and WHAT the agent should use instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenialResponse {
    /// The blocked input (redacted if sensitive).
    pub blocked: String,
    /// Human-readable reason (policy rule name or description).
    pub reason: String,
    /// The owned alternative the agent should use instead.
    pub alternative: Option<String>,
    /// Policy rule ID that matched, for log correlation.
    pub rule_id: Option<String>,
}

impl DenialResponse {
    pub fn new(blocked: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            blocked: blocked.into(),
            reason: reason.into(),
            alternative: None,
            rule_id: None,
        }
    }

    pub fn with_alternative(mut self, alt: impl Into<String>) -> Self {
        self.alternative = Some(alt.into());
        self
    }

    pub fn with_rule_id(mut self, id: impl Into<String>) -> Self {
        self.rule_id = Some(id.into());
        self
    }

    /// Render as a terminal message string. Printed inline in the shell.
    ///
    /// Format:
    ///   \r\n[ROY] BLOCKED: <blocked>\r\n       Reason:      <reason>\r\n       Alternative: <alt>\r\n
    pub fn render(&self) -> String {
        let mut out = format!("\r\n[ROY] BLOCKED: {}\r\n      Reason:      {}\r\n", self.blocked, self.reason);
        if let Some(alt) = &self.alternative {
            out.push_str(&format!("      Alternative: {}\r\n", alt));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_includes_blocked_and_reason() {
        let d = DenialResponse::new("rm -rf /", "destructive command blocked");
        let rendered = d.render();
        assert!(rendered.contains("rm -rf /"), "rendered must include blocked command");
        assert!(rendered.contains("destructive command blocked"), "rendered must include reason");
        assert!(!rendered.contains("Alternative:"), "no alternative section when None");
    }

    #[test]
    fn render_includes_alternative_when_set() {
        let d = DenialResponse::new("grep -r foo .", "use semmap cat instead")
            .with_alternative("semmap cat <file>");
        let rendered = d.render();
        assert!(rendered.contains("semmap cat <file>"), "rendered must include alternative");
    }

    #[test]
    fn serialization_round_trip() {
        let d = DenialResponse::new("bad_cmd", "reason")
            .with_alternative("good_cmd")
            .with_rule_id("DENY-001");
        let json = serde_json::to_string(&d).expect("serialize");
        let back: DenialResponse = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.blocked, "bad_cmd");
        assert_eq!(back.rule_id.as_deref(), Some("DENY-001"));
    }
}
