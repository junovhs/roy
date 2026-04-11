// Live in tests; UI wiring pending SES-02.
#![allow(dead_code)]

use std::path::PathBuf;

use super::artifacts::SessionArtifact;
use super::events::{SessionEvent, Timestamp};

/// A ROY shell session — owns the ordered event ledger.
///
/// A `Session` is created when a shell pane opens and accumulates every
/// typed event that occurs: user inputs, command invocations, policy
/// outcomes, agent outputs, and host notices. It is the durable
/// structure underneath the terminal transcript.
pub struct Session {
    /// Unique identifier for this session (ms since UNIX epoch at creation).
    pub id: u64,
    /// Workspace root at session start.
    pub workspace_root: PathBuf,
    /// Ordered event ledger.
    events: Vec<SessionEvent>,
}

impl Session {
    /// Open a new session rooted at `workspace_root`.
    ///
    /// Automatically pushes a [`SessionEvent::SessionStarted`] event so
    /// the ledger always begins with a known anchor point.
    pub fn new(id: u64, workspace_root: PathBuf, now: Timestamp) -> Self {
        let mut s = Self {
            id,
            workspace_root,
            events: Vec::new(),
        };
        s.events.push(SessionEvent::SessionStarted { ts: now });
        s
    }

    /// Append an event to the ledger.
    pub fn push(&mut self, event: SessionEvent) {
        self.events.push(event);
    }

    /// Ordered slice of all events since the session started.
    pub fn events(&self) -> &[SessionEvent] {
        &self.events
    }

    /// All events whose `kind_str()` matches `kind`.
    ///
    /// Useful for filtering: `session.events_of_kind("command_denied")`.
    pub fn events_of_kind<'a>(&'a self, kind: &str) -> Vec<&'a SessionEvent> {
        self.events
            .iter()
            .filter(|e| e.kind_str() == kind)
            .collect()
    }

    /// All promoted artifacts in chronological order.
    pub fn artifacts(&self) -> Vec<&SessionArtifact> {
        self.events
            .iter()
            .filter_map(|event| match event {
                SessionEvent::ArtifactCreated { artifact, .. } => Some(artifact),
                _ => None,
            })
            .collect()
    }

    /// Iterator over all events in chronological order — for replay.
    pub fn replay(&self) -> impl Iterator<Item = &SessionEvent> {
        self.events.iter()
    }

    /// Total number of events including the initial `SessionStarted`.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// True only when no events exist (should not happen after `new`).
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Close the session, appending a [`SessionEvent::SessionEnded`].
    pub fn end(&mut self, exit_code: i32, now: Timestamp) {
        self.events
            .push(SessionEvent::SessionEnded { exit_code, ts: now });
    }
}

#[cfg(test)]
#[path = "engine_tests.rs"]
mod tests;
