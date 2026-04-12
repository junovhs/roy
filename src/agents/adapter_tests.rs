//! Tests for the embedded-agent adapter contract:
//! AgentKind, AgentHandle lifecycle, and AgentError classifications.

use super::{AgentError, AgentErrorKind, AgentHandle, AgentKind, AgentMeta, SupervisionEvent};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn claude_meta() -> AgentMeta {
    AgentMeta {
        kind: AgentKind::ClaudeCode,
        version: "1.0.0".to_string(),
        install_path: PathBuf::from("/usr/local/bin/claude"),
    }
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

// ── AgentKind ─────────────────────────────────────────────────────────────────

#[test]
fn agent_kind_id_claude_code() {
    assert_eq!(AgentKind::ClaudeCode.id(), "claude-code");
}

#[test]
fn agent_kind_id_codex() {
    assert_eq!(AgentKind::Codex.id(), "codex");
}

#[test]
fn agent_kind_id_custom() {
    let k = AgentKind::Custom {
        name: "my-agent".to_string(),
    };
    assert_eq!(k.id(), "my-agent");
}

// ── AgentHandle ───────────────────────────────────────────────────────────────

#[test]
fn new_handle_has_no_events() {
    let h = AgentHandle::new(claude_meta(), 1);
    assert!(h.events().is_empty());
}

#[test]
fn new_handle_has_not_exited() {
    let h = AgentHandle::new(claude_meta(), 1);
    assert!(!h.has_exited());
    assert_eq!(h.exit_code(), None);
}

#[test]
fn push_event_records_output_line() {
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.push_event(SupervisionEvent::OutputLine {
        text: "hello".to_string(),
    });
    assert_eq!(h.events().len(), 1);
    assert_eq!(
        h.events()[0],
        SupervisionEvent::OutputLine {
            text: "hello".to_string()
        }
    );
}

#[test]
fn push_process_exited_zero_sets_exit_code() {
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.push_event(SupervisionEvent::ProcessExited { code: 0 });
    assert!(h.has_exited());
    assert_eq!(h.exit_code(), Some(0));
}

#[test]
fn push_process_exited_nonzero_code() {
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.push_event(SupervisionEvent::ProcessExited { code: 1 });
    assert_eq!(h.exit_code(), Some(1));
}

#[test]
fn events_before_exit_do_not_mark_exited() {
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.push_event(SupervisionEvent::AgentStarted { pid: 42 });
    h.push_event(SupervisionEvent::OutputLine {
        text: "ok".to_string(),
    });
    assert!(!h.has_exited());
}

#[test]
fn multiple_events_are_recorded_in_order() {
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.push_event(SupervisionEvent::AgentStarted { pid: 100 });
    h.push_event(SupervisionEvent::OutputLine {
        text: "line1".to_string(),
    });
    h.push_event(SupervisionEvent::ProcessExited { code: 0 });
    assert_eq!(h.events().len(), 3);
    assert!(matches!(
        h.events()[0],
        SupervisionEvent::AgentStarted { .. }
    ));
    assert!(matches!(
        h.events()[2],
        SupervisionEvent::ProcessExited { .. }
    ));
}

// ── drain_pending ─────────────────────────────────────────────────────────────

#[test]
fn drain_pending_with_no_queue_is_noop() {
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.drain_pending(); // must not panic
    assert!(h.events().is_empty());
}

#[test]
fn drain_pending_moves_events_into_log() {
    use std::sync::{Arc, Mutex};
    let queue = Arc::new(Mutex::new(vec![
        SupervisionEvent::OutputLine {
            text: "line-a".to_string(),
        },
        SupervisionEvent::OutputLine {
            text: "line-b".to_string(),
        },
    ]));
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.set_pending(Arc::clone(&queue));
    h.drain_pending();
    assert_eq!(h.events().len(), 2);
    assert!(
        queue.lock().unwrap().is_empty(),
        "queue must be empty after drain"
    );
}

#[test]
fn drain_pending_captures_exit_code_from_queue() {
    use std::sync::{Arc, Mutex};
    let queue = Arc::new(Mutex::new(vec![SupervisionEvent::ProcessExited {
        code: 42,
    }]));
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.set_pending(queue);
    h.drain_pending();
    assert!(h.has_exited(), "exited state must be set");
    assert_eq!(h.exit_code(), Some(42));
}

#[test]
fn send_input_writes_to_attached_stdin() {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let writer = SharedWriter {
        buf: Arc::clone(&shared),
    };
    let mut h = AgentHandle::new(claude_meta(), 1);
    h.set_stdin(Arc::new(Mutex::new(Box::new(writer))));

    h.send_input("hello\n").expect("stdin write must succeed");

    assert_eq!(&*shared.lock().unwrap(), b"hello\n");
}

#[test]
fn send_input_without_stdin_returns_io_error() {
    let h = AgentHandle::new(claude_meta(), 1);
    let err = h.send_input("hello").expect_err("missing stdin must error");
    assert_eq!(err.kind(), &AgentErrorKind::IoError);
}

// ── AgentError ────────────────────────────────────────────────────────────────

#[test]
fn not_installed_error_has_correct_kind() {
    let e = AgentError::not_installed("claude");
    assert_eq!(e.kind(), &AgentErrorKind::NotInstalled);
    assert!(e.to_string().contains("claude"));
}

#[test]
fn launch_failed_error_has_correct_kind() {
    let e = AgentError::launch_failed("spawn error");
    assert_eq!(e.kind(), &AgentErrorKind::LaunchFailed);
}

#[test]
fn auth_required_error_has_correct_kind() {
    let e = AgentError::auth_required("run claude auth");
    assert_eq!(e.kind(), &AgentErrorKind::AuthRequired);
    assert!(e.to_string().contains("run claude auth"));
}

#[test]
fn io_error_has_correct_kind() {
    let e = AgentError::io_error("pipe broken");
    assert_eq!(e.kind(), &AgentErrorKind::IoError);
}
