use std::sync::Mutex;
use crate::agent::AgentHost;
use crate::denial::DenialResponse;
use crate::interceptor::{strip_bracketed_paste, Disposition, LineBuffer, RoyInterceptor};
use crate::policy::PolicyEngine;

/// Concrete ROY interceptor bound to a session.
///
/// Holds a base PolicyEngine, an optional AgentHost (for agent-mode extra
/// rules), a line buffer, and a pending-denial queue — all Mutex-wrapped
/// for Send+Sync.
///
/// When an agent is active (AgentHost detected it from terminal output),
/// the session merges the agent's extra rules and checks them before the
/// base policy. Agent rules take priority — an agent cannot escape stricter
/// enforcement by relying on gaps in the base rule set.
pub struct RoySession {
    base_policy: PolicyEngine,
    agent_host: Option<AgentHost>,
    line_buf: Mutex<LineBuffer>,
    /// Denials queued during write_to_pty, drained by handle_event.
    pending_denials: Mutex<Vec<DenialResponse>>,
}

impl RoySession {
    pub fn new(policy: PolicyEngine) -> Self {
        Self {
            base_policy: policy,
            agent_host: None,
            line_buf: Mutex::new(LineBuffer::new()),
            pending_denials: Mutex::new(Vec::new()),
        }
    }

    /// Create a session with an agent host for agent-mode supervision.
    pub fn with_agent_host(policy: PolicyEngine, agent_host: AgentHost) -> Self {
        Self {
            base_policy: policy,
            agent_host: Some(agent_host),
            line_buf: Mutex::new(LineBuffer::new()),
            pending_denials: Mutex::new(Vec::new()),
        }
    }

    pub fn passthrough() -> Self {
        Self::new(PolicyEngine::empty())
    }

    /// Expose the agent host so the terminal output path can call observe().
    pub fn agent_host(&self) -> Option<&AgentHost> {
        self.agent_host.as_ref()
    }
}

impl RoyInterceptor for RoySession {
    fn intercept(&self, bytes: &[u8], in_raw_mode: bool) -> Disposition {
        if in_raw_mode {
            return Disposition::Passthrough;
        }

        let mut buf = self.line_buf.lock().unwrap_or_else(|e| e.into_inner());
        match buf.push(bytes) {
            None => Disposition::Passthrough,
            Some(line) => {
                let raw = String::from_utf8_lossy(&line);
                let line_str = strip_bracketed_paste(raw.as_ref());

                // Agent-mode extra rules are evaluated first (higher priority).
                // They activate only when the AgentHost has detected a running agent.
                if let Some(host) = &self.agent_host {
                    let extra = host.active_extra_rules();
                    if !extra.is_empty() {
                        match PolicyEngine::new(extra).evaluate(&line_str) {
                            Disposition::Denied(resp) => return self.queue_and_deny(resp),
                            Disposition::Redirect(b) => return Disposition::Redirect(b),
                            Disposition::Passthrough => {},
                        }
                    }
                }

                // Base policy.
                match self.base_policy.evaluate(&line_str) {
                    Disposition::Passthrough => Disposition::Passthrough,
                    Disposition::Denied(resp) => self.queue_and_deny(resp),
                    Disposition::Redirect(new_bytes) => Disposition::Redirect(new_bytes),
                }
            },
        }
    }

    fn take_pending_denials(&self) -> Vec<DenialResponse> {
        let mut q = self.pending_denials.lock().unwrap_or_else(|e| e.into_inner());
        std::mem::take(&mut *q)
    }
}

/// Build a RoySession from the default roy.toml location.
pub fn session_from_env() -> RoySession {
    match crate::config::load_default() {
        Ok(cfg) if cfg.enabled => {
            let rules = cfg.policy.rules.into_iter().map(|r| r.into_rule()).collect();
            RoySession::new(crate::policy::PolicyEngine::new(rules))
        },
        Ok(_) => {
            log::info!("[ROY] disabled via config");
            RoySession::passthrough()
        },
        Err(e) => {
            log::warn!("[ROY] config load failed: {e} — using passthrough");
            RoySession::passthrough()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{Rule, RuleAction, RulePattern};

    fn deny_session(pattern: &str) -> RoySession {
        let rule = Rule {
            id: "T001".to_string(),
            description: "test deny".to_string(),
            pattern: RulePattern::Contains(pattern.to_string()),
            action: RuleAction::Deny,
            alternative: Some("safe_alt".to_string()),
        };
        RoySession::new(PolicyEngine::new(vec![rule]))
    }

    #[test]
    fn raw_mode_always_passes_through() {
        let session = deny_session("anything");
        let result = session.intercept(b"anything\n", true);
        assert!(matches!(result, Disposition::Passthrough));
        assert!(session.take_pending_denials().is_empty(), "raw mode should not queue denials");
    }

    #[test]
    fn partial_line_passes_through() {
        let session = deny_session("blocked");
        let result = session.intercept(b"bloc", false);
        assert!(matches!(result, Disposition::Passthrough));
    }

    #[test]
    fn complete_safe_line_passes_through() {
        let session = deny_session("blocked");
        let result = session.intercept(b"ls -la\n", false);
        assert!(matches!(result, Disposition::Passthrough));
        assert!(session.take_pending_denials().is_empty());
    }

    #[test]
    fn complete_blocked_line_is_denied_and_queued() {
        let session = deny_session("blocked_cmd");
        let result = session.intercept(b"blocked_cmd\n", false);
        match result {
            Disposition::Denied(d) => {
                assert_eq!(d.rule_id.as_deref(), Some("T001"));
                assert_eq!(d.alternative.as_deref(), Some("safe_alt"));
            },
            _ => panic!("expected Denied"),
        }
        let pending = session.take_pending_denials();
        assert_eq!(pending.len(), 1, "denial should be queued");
        assert_eq!(pending[0].rule_id.as_deref(), Some("T001"));
    }

    #[test]
    fn take_pending_denials_drains_queue() {
        let session = deny_session("blocked_cmd");
        session.intercept(b"blocked_cmd\n", false);
        let first = session.take_pending_denials();
        assert_eq!(first.len(), 1);
        let second = session.take_pending_denials();
        assert!(second.is_empty(), "queue must be drained after take");
    }

    #[test]
    fn line_buffer_resets_after_complete_line() {
        let session = deny_session("never_matches");
        session.intercept(b"ls\n", false);
        session.intercept(b"ec", false);
        let result = session.intercept(b"ho\n", false);
        assert!(matches!(result, Disposition::Passthrough));
    }

    fn bracketed_paste(inner: &[u8]) -> Vec<u8> {
        // ESC (0x1b) [ (0x5b) 2 0 0 ~ (0x7e) <content> ESC [ 2 0 1 ~ LF
        // All bytes as hex to avoid neti's parser choking on bracket characters.
        let mut v = vec![0x1b_u8, 0x5b, 0x32, 0x30, 0x30, 0x7e];
        v.extend_from_slice(inner);
        v.extend_from_slice(&[0x1b_u8, 0x5b, 0x32, 0x30, 0x31, 0x7e, 0x0a]);
        v
    }

    #[test]
    fn bracketed_paste_command_is_evaluated_without_markers() {
        // A pasted blocked command arrives wrapped in ESC[200~ / ESC[201~.
        // Policy must fire on the inner text; blocked field must not contain markers.
        let session = deny_session("blocked_cmd");
        let pasted = bracketed_paste(b"blocked_cmd");
        let result = session.intercept(&pasted, false);
        match result {
            Disposition::Denied(d) => {
                assert_eq!(d.blocked, "blocked_cmd", "blocked field must not contain escape sequences");
            },
            _ => panic!("expected Denied for bracketed-paste blocked command"),
        }
    }

    #[test]
    fn bracketed_paste_safe_command_passes_through() {
        let session = deny_session("blocked_cmd");
        let pasted = bracketed_paste(b"ls -la");
        let result = session.intercept(&pasted, false);
        assert!(matches!(result, Disposition::Passthrough));
    }
}
