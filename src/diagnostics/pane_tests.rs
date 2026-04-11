//! Tests for diagnostics::pane — build_trace and DiagSeverity.

use crate::commands::CommandRegistry;
use crate::session::{ArtifactBody, ArtifactKind, SessionArtifact, SessionEvent};
use super::{build_trace, DiagSeverity};

fn reg() -> CommandRegistry { CommandRegistry::new() }

fn invoked(command: &str, args: Vec<&str>) -> SessionEvent {
    SessionEvent::CommandInvoked {
        command: command.to_string(),
        args: args.iter().map(|s| s.to_string()).collect(),
        ts: 0,
    }
}

fn denied(command: &str, suggestion: Option<&str>) -> SessionEvent {
    SessionEvent::CommandDenied {
        command: command.to_string(),
        suggestion: suggestion.map(str::to_string),
        ts: 0,
    }
}

fn not_found(command: &str) -> SessionEvent {
    SessionEvent::CommandNotFound { command: command.to_string(), ts: 0 }
}

fn artifact(name: &str) -> SessionEvent {
    SessionEvent::ArtifactCreated {
        artifact: SessionArtifact {
            name: name.to_string(),
            kind: ArtifactKind::ValidationRun,
            summary: "cargo check passed".to_string(),
            body: ArtifactBody::Note {
                text: "ok".to_string(),
            },
        },
        ts: 0,
    }
}

// ── builtin resolution ────────────────────────────────────────────────────────

#[test]
fn builtin_command_shows_builtin_backend() {
    let events = vec![invoked("pwd", vec![])];
    let trace = build_trace(&events, &reg(), 10);
    assert_eq!(trace.len(), 1);
    assert_eq!(trace[0].tag, "resolve");
    assert!(trace[0].text.contains("[builtin]"), "got: {}", trace[0].text);
    assert_eq!(trace[0].severity, DiagSeverity::Info);
}

#[test]
fn native_command_shows_native_backend() {
    let events = vec![invoked("ls", vec![])];
    let trace = build_trace(&events, &reg(), 10);
    assert!(trace[0].text.contains("[native]"), "got: {}", trace[0].text);
}

#[test]
fn command_with_args_includes_args_in_trace() {
    let events = vec![invoked("cd", vec!["/tmp"])];
    let trace = build_trace(&events, &reg(), 10);
    assert!(trace[0].text.contains("/tmp"), "got: {}", trace[0].text);
}

// ── blocked / not found ───────────────────────────────────────────────────────

#[test]
fn denied_command_shows_error_severity() {
    let events = vec![denied("bash", Some("ROY does not provide a bash surface"))];
    let trace = build_trace(&events, &reg(), 10);
    assert_eq!(trace[0].tag, "blocked");
    assert_eq!(trace[0].severity, DiagSeverity::Error);
    assert!(trace[0].text.contains("bash"));
}

#[test]
fn denied_without_suggestion_shows_fallback_reason() {
    let events = vec![denied("bash", None)];
    let trace = build_trace(&events, &reg(), 10);
    assert!(trace[0].text.contains("policy denied"), "got: {}", trace[0].text);
}

#[test]
fn not_found_shows_warn_severity() {
    let events = vec![not_found("totally_unknown")];
    let trace = build_trace(&events, &reg(), 10);
    assert_eq!(trace[0].tag, "notfound");
    assert_eq!(trace[0].severity, DiagSeverity::Warn);
    assert!(trace[0].text.contains("totally_unknown"));
}

// ── filtering ─────────────────────────────────────────────────────────────────

#[test]
fn user_input_events_are_excluded() {
    let events = vec![SessionEvent::UserInput { text: "pwd".to_string(), ts: 0 }];
    assert!(build_trace(&events, &reg(), 10).is_empty());
}

#[test]
fn command_output_events_are_excluded() {
    let events = vec![SessionEvent::CommandOutput {
        text: "output".to_string(),
        is_error: false,
        ts: 0,
    }];
    assert!(build_trace(&events, &reg(), 10).is_empty());
}

#[test]
fn limit_caps_trace_length() {
    let events: Vec<SessionEvent> = (0..20).map(|_| invoked("pwd", vec![])).collect();
    let trace = build_trace(&events, &reg(), 5);
    assert_eq!(trace.len(), 5);
}

#[test]
fn trace_is_newest_first() {
    let events = vec![invoked("pwd", vec![]), invoked("ls", vec![])];
    let trace = build_trace(&events, &reg(), 10);
    assert!(trace[0].text.starts_with("ls"), "got: {}", trace[0].text);
    assert!(trace[1].text.starts_with("pwd"), "got: {}", trace[1].text);
}

#[test]
fn artifacts_are_visible_in_trace() {
    let trace = build_trace(&[artifact("check")], &reg(), 10);
    assert_eq!(trace[0].tag, "artifact");
    assert!(trace[0].text.contains("check"));
}

// ── DiagSeverity ──────────────────────────────────────────────────────────────

#[test]
fn severity_colors_are_distinct() {
    assert_ne!(DiagSeverity::Info.color(), DiagSeverity::Error.color());
    assert_ne!(DiagSeverity::Warn.color(), DiagSeverity::Error.color());
}
