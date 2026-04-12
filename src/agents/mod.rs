//! Embedded-agent adapter layer.
//!
//! Installs, launches, and supervises terminal-native agents (Claude Code,
//! Codex) inside the ROY shell. Manages stdin/stdout attachment, auth
//! handoffs, session continuity, and typed host-event capture. Agents are
//! privileged tenants, not arbitrary binaries.
//!
//! ## Contract
//!
//! [`adapter::AgentAdapter`] is the core trait: each concrete adapter
//! (implemented in AGEN-02+) describes one agent kind and knows how to
//! launch it. [`adapter::AgentHandle`] owns the per-launch lifecycle state
//! and supervision event buffer. [`session::AgentSession`] is the durable,
//! cloneable session record the UI and event ledger use without holding the
//! live process handle.
//!
//! Concrete adapters: [`claude_code::ClaudeCodeAdapter`] and
//! [`codex::CodexAdapter`].

pub mod adapter;
pub mod claude_code;
pub mod codex;
mod host;
pub mod session;

#[cfg(test)]
#[path = "agent_contract_tests.rs"]
mod contract_tests;
