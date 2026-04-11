//! Embedded-agent adapter layer.
//!
//! Installs, launches, and supervises terminal-native agents (Claude Code,
//! Codex) inside the ROY shell. Manages stdin/stdout attachment, auth
//! handoffs, session continuity, and typed host-event capture. Agents are
//! privileged tenants, not arbitrary binaries.
//!
//! Populated by AGEN-01.
