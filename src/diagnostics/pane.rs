// Live in tests; UI wiring is in ui/layout/footer.rs (DiagnosticsPane).
#![allow(dead_code)]

//! Trace computation for the developer diagnostics pane.
//!
//! Converts session events into structured [`DiagEntry`] items enriched
//! with registry-level resolution detail (backend type, policy outcome).
//! Pure computation — no Dioxus dependency.

use crate::commands::schema::Backend;
use crate::commands::CommandRegistry;
use crate::session::SessionEvent;

// ── entry ─────────────────────────────────────────────────────────────────────

/// Severity level for a diagnostics trace entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagSeverity {
    Info,
    Warn,
    Error,
}

impl DiagSeverity {
    /// CSS hex color appropriate for this severity.
    pub fn color(self) -> &'static str {
        match self {
            Self::Info => "#6e7681",
            Self::Warn => "#d29922",
            Self::Error => "#f85149",
        }
    }
}

/// One diagnostics trace entry rendered in the developer pane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagEntry {
    /// Short category tag shown before the detail text.
    pub tag: &'static str,
    /// Human-readable detail for this trace entry.
    pub text: String,
    /// Visual severity — controls color in the pane.
    pub severity: DiagSeverity,
}

impl DiagEntry {
    fn info(tag: &'static str, text: String) -> Self {
        Self {
            tag,
            text,
            severity: DiagSeverity::Info,
        }
    }

    fn warn(tag: &'static str, text: String) -> Self {
        Self {
            tag,
            text,
            severity: DiagSeverity::Warn,
        }
    }

    fn error(tag: &'static str, text: String) -> Self {
        Self {
            tag,
            text,
            severity: DiagSeverity::Error,
        }
    }
}

// ── trace builder ─────────────────────────────────────────────────────────────

/// Build a diagnostics trace from session events, enriched with registry info.
///
/// Returns up to `limit` entries in reverse chronological order (newest first).
/// Filters out low-value events (output lines, session lifecycle) that are
/// already visible in the shell transcript.
pub fn build_trace(
    events: &[SessionEvent],
    registry: &CommandRegistry,
    limit: usize,
) -> Vec<DiagEntry> {
    events
        .iter()
        .rev()
        .filter_map(|e| entry_from_event(e, registry))
        .take(limit)
        .collect()
}

fn entry_from_event(event: &SessionEvent, registry: &CommandRegistry) -> Option<DiagEntry> {
    match event {
        SessionEvent::CommandInvoked { command, args, .. } => {
            let backend_tag = registry
                .resolve(command)
                .map(|schema| backend_label(&schema.backend))
                .unwrap_or("?");
            let arg_str = if args.is_empty() {
                String::new()
            } else {
                format!(" {}", args.join(" "))
            };
            Some(DiagEntry::info(
                "resolve",
                format!("{command}{arg_str} → [{backend_tag}]"),
            ))
        }

        SessionEvent::CommandDenied {
            command,
            suggestion,
            ..
        } => {
            let reason = suggestion.as_deref().unwrap_or("policy denied");
            Some(DiagEntry::error("blocked", format!("{command}: {reason}")))
        }

        SessionEvent::CommandNotFound { command, .. } => Some(DiagEntry::warn(
            "notfound",
            format!("{command}: not in ROY command world"),
        )),

        SessionEvent::AgentOutput { text, .. } => {
            let truncated = if text.len() > 80 {
                format!("{}…", &text[..80])
            } else {
                text.clone()
            };
            Some(DiagEntry::info("agent", truncated))
        }

        SessionEvent::ArtifactCreated { artifact, .. } => Some(DiagEntry::info(
            "artifact",
            format!("{}: {}", artifact.kind_str(), artifact.name),
        )),

        // These events are already visible in other panes or are noise.
        SessionEvent::SessionStarted { .. }
        | SessionEvent::SessionEnded { .. }
        | SessionEvent::UserInput { .. }
        | SessionEvent::CommandOutput { .. }
        | SessionEvent::CwdChanged { .. }
        | SessionEvent::HostNotice { .. } => None,
    }
}

fn backend_label(backend: &Backend) -> &'static str {
    match backend {
        Backend::Builtin => "builtin",
        Backend::RoyNative => "native",
        Backend::CompatTrap { .. } => "trap",
        Backend::Blocked { .. } => "blocked",
    }
}

#[cfg(test)]
#[path = "pane_tests.rs"]
mod tests;
