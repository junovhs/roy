//! Tests for agent launch dispatch: already-running guard, not-installed
//! path, and auth-gate regressions.

use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::agents::adapter::{AgentHandle, AgentKind, AgentMeta};
use crate::shell::{DispatchResult, ShellRuntime};

fn rt() -> ShellRuntime {
    ShellRuntime::new(std::env::temp_dir())
}

fn fake_handle() -> AgentHandle {
    let meta = AgentMeta {
        kind: AgentKind::ClaudeCode,
        version: "1.0.0".to_string(),
        install_path: PathBuf::from("/usr/local/bin/claude"),
    };
    AgentHandle::new(meta, 0)
}

struct SharedWriter {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl Write for SharedWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buf.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// ── already-running guard ─────────────────────────────────────────────────────

#[test]
fn dispatch_claude_while_agent_running_returns_error() {
    let mut rt = rt();
    rt.agent_handle = Some(fake_handle());
    match rt.dispatch("claude", &[]) {
        DispatchResult::Executed { exit_code, .. } => {
            assert_eq!(
                exit_code, 1,
                "must fail with exit 1 when agent already running"
            );
        }
        other => panic!("expected Executed error, got {other:?}"),
    }
    // handle must NOT be cleared — the existing session stays
    assert!(rt.agent_handle.is_some(), "existing handle must survive");
}

#[test]
fn already_running_error_mentions_exit() {
    let mut rt = rt();
    rt.agent_handle = Some(fake_handle());
    let lines: Vec<String> = rt.drain_errors(); // clear any previous
    let _ = lines;
    rt.dispatch("claude", &[]);
    let errors = rt.drain_errors();
    assert!(
        errors.iter().any(|l| l.contains("exit")),
        "error must tell user to exit the current session: {errors:?}"
    );
}

// ── not-installed path ────────────────────────────────────────────────────────

/// When `claude` is not on PATH, dispatch must return Executed with exit 127
/// and an informative error line — it must NOT panic or return NotFound.
#[test]
fn dispatch_claude_not_installed_returns_exec_with_127() {
    // This test relies on `claude` NOT being installed in the test environment.
    // If it is installed, this test is skipped via the same env-check pattern
    // used in claude_code_tests.
    let has_claude = std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|d| PathBuf::from(d).join("claude").is_file());

    if has_claude {
        return; // claude is installed; skip the not-installed path
    }

    let mut rt = rt();
    match rt.dispatch("claude", &[]) {
        DispatchResult::Executed { exit_code, .. } => {
            assert_eq!(exit_code, 127, "not-installed must exit 127");
        }
        other => panic!("expected Executed error for missing binary, got {other:?}"),
    }
}

#[test]
fn dispatch_claude_not_installed_writes_error_line() {
    let has_claude = std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|d| PathBuf::from(d).join("claude").is_file());

    if has_claude {
        return;
    }

    let mut rt = rt();
    rt.dispatch("claude", &[]);
    let errors = rt.drain_errors();
    assert!(
        !errors.is_empty(),
        "a not-installed error line must be written"
    );
    assert!(
        errors
            .iter()
            .any(|l| l.to_lowercase().contains("install") || l.contains("PATH")),
        "error must mention installation or PATH: {errors:?}"
    );
}

// ── missing API key must not block launch ─────────────────────────────────────

/// Claude Code can authenticate through persisted CLI login, not only API key.
/// ROY must not reject launch before Claude gets a chance to use that state.
#[test]
fn dispatch_claude_missing_api_key_does_not_fail_preflight_auth_gate() {
    let has_claude = std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|d| PathBuf::from(d).join("claude").is_file());

    if !has_claude {
        return; // binary absent; not the path under test here
    }

    // Temporarily clear the API key for this test.
    let saved = std::env::var("ANTHROPIC_API_KEY").ok();
    std::env::remove_var("ANTHROPIC_API_KEY");

    let mut rt = rt();
    let result = rt.dispatch("claude", &[]);

    // Restore key before any assertions that could panic.
    if let Some(k) = saved {
        std::env::set_var("ANTHROPIC_API_KEY", k);
    }

    if let DispatchResult::Executed {
        exit_code, output, ..
    } = &result
    {
        assert!(
            *exit_code != 1
                || (!output.contains("ANTHROPIC_API_KEY")
                    && !output.to_lowercase().contains("authentication required")),
            "missing API key must not trigger ROY preflight auth failure: {output}"
        );
    }

    let errors = rt.drain_errors();
    assert!(
        !errors.iter().any(|l| {
            l.contains("ANTHROPIC_API_KEY") || l.to_lowercase().contains("authentication required")
        }),
        "ROY must not emit API-key auth gate error: {errors:?}"
    );
}

// ── poll_agent_lines with no agent ────────────────────────────────────────────

#[test]
fn poll_agent_lines_with_no_agent_returns_empty() {
    let mut rt = rt();
    let (lines, exited) = rt.poll_agent_lines();
    assert!(lines.is_empty(), "no agent means no lines");
    assert!(!exited, "no agent means not exited");
}

#[test]
fn send_agent_input_forwards_line_with_newline() {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let writer = SharedWriter {
        buf: Arc::clone(&shared),
    };
    let mut handle = fake_handle();
    handle.set_stdin(Arc::new(Mutex::new(Box::new(writer))));

    let mut rt = rt();
    rt.agent_handle = Some(handle);

    rt.send_agent_input("status")
        .expect("agent input forward must succeed");

    // PTY expects \r (carriage return) for Enter, not \n.
    assert_eq!(&*shared.lock().unwrap(), b"status\r");
}

#[test]
fn send_agent_input_without_agent_returns_error() {
    let mut rt = rt();
    let err = rt
        .send_agent_input("status")
        .expect_err("missing agent must error");
    assert!(err.contains("no embedded agent"));
}
