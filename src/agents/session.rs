// Session contract is live in tests; UI wiring and ledger integration pending AGEN-02.
#![allow(dead_code)]

//! Agent session — durable record of one embedded-agent session inside ROY.
//!
//! Links the agent's meta, the ROY session ID, and the lifecycle state so the
//! session ledger and UI can display meaningful agent context without coupling
//! to the live process handle.

use std::path::PathBuf;

use super::adapter::AgentMeta;

// ── lifecycle state ───────────────────────────────────────────────────────────

/// Lifecycle state of an embedded-agent session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentSessionState {
    /// Agent is being prepared but has not yet started running.
    Initializing,
    /// Agent process is running and accepting input.
    Running,
    /// Agent process is temporarily paused (e.g. awaiting approval).
    Suspended,
    /// Agent process has exited with `code`.
    Exited { code: i32 },
}

impl AgentSessionState {
    /// True when the agent is expected to produce output.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Initializing | Self::Running)
    }
}

// ── session ───────────────────────────────────────────────────────────────────

/// Durable record of one embedded-agent session inside ROY.
///
/// Distinct from the shell [`Session`][crate::session::Session]: the shell
/// session owns the event ledger; the agent session records which agent
/// is hosted and what state it is in. Both share the same `session_id`.
#[derive(Debug, Clone)]
pub struct AgentSession {
    /// ROY session ID this agent is associated with.
    pub session_id: u64,
    /// Metadata of the hosted agent.
    pub meta: AgentMeta,
    /// Current lifecycle state.
    pub state: AgentSessionState,
    /// Workspace root at launch time.
    pub workspace_root: PathBuf,
}

impl AgentSession {
    /// Create a new agent session record in the `Initializing` state.
    pub fn new(session_id: u64, meta: AgentMeta, workspace_root: PathBuf) -> Self {
        Self {
            session_id,
            meta,
            state: AgentSessionState::Initializing,
            workspace_root,
        }
    }

    /// Short identifier of the hosted agent kind (e.g. `"claude-code"`).
    pub fn agent_id(&self) -> &str {
        self.meta.kind.id()
    }

    /// Transition to [`AgentSessionState::Running`].
    pub fn mark_running(&mut self) {
        self.state = AgentSessionState::Running;
    }

    /// Transition to [`AgentSessionState::Suspended`].
    pub fn mark_suspended(&mut self) {
        self.state = AgentSessionState::Suspended;
    }

    /// Transition to [`AgentSessionState::Exited`] with `code`.
    pub fn mark_exited(&mut self, code: i32) {
        self.state = AgentSessionState::Exited { code };
    }

    /// True while the agent is `Initializing` or `Running`.
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::adapter::AgentKind;

    fn meta() -> AgentMeta {
        AgentMeta {
            kind: AgentKind::ClaudeCode,
            version: "1.0.0".to_string(),
            install_path: PathBuf::from("/usr/local/bin/claude"),
        }
    }

    fn session() -> AgentSession {
        AgentSession::new(42, meta(), PathBuf::from("/tmp/ws"))
    }

    // ── initial state ─────────────────────────────────────────────────────────

    #[test]
    fn new_session_is_initializing() {
        assert_eq!(session().state, AgentSessionState::Initializing);
    }

    #[test]
    fn new_session_is_active() {
        assert!(session().is_active());
    }

    #[test]
    fn new_session_agent_id_matches_kind() {
        assert_eq!(session().agent_id(), "claude-code");
    }

    // ── transitions ───────────────────────────────────────────────────────────

    #[test]
    fn mark_running_transitions_to_running() {
        let mut s = session();
        s.mark_running();
        assert_eq!(s.state, AgentSessionState::Running);
        assert!(s.is_active());
    }

    #[test]
    fn mark_suspended_is_not_active() {
        let mut s = session();
        s.mark_running();
        s.mark_suspended();
        assert_eq!(s.state, AgentSessionState::Suspended);
        assert!(!s.is_active());
    }

    #[test]
    fn mark_exited_records_code_and_is_not_active() {
        let mut s = session();
        s.mark_running();
        s.mark_exited(0);
        assert_eq!(s.state, AgentSessionState::Exited { code: 0 });
        assert!(!s.is_active());
    }

    #[test]
    fn mark_exited_nonzero_code() {
        let mut s = session();
        s.mark_exited(1);
        match s.state {
            AgentSessionState::Exited { code } => assert_eq!(code, 1),
            other => panic!("expected Exited, got {other:?}"),
        }
    }

    // ── AgentSessionState helpers ─────────────────────────────────────────────

    #[test]
    fn initializing_is_active() {
        assert!(AgentSessionState::Initializing.is_active());
    }

    #[test]
    fn running_is_active() {
        assert!(AgentSessionState::Running.is_active());
    }

    #[test]
    fn suspended_is_not_active() {
        assert!(!AgentSessionState::Suspended.is_active());
    }

    #[test]
    fn exited_is_not_active() {
        assert!(!AgentSessionState::Exited { code: 0 }.is_active());
    }
}
