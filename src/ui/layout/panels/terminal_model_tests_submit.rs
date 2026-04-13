use super::test_support::*;
use super::*;
use crate::session::SessionArtifact;
use dioxus::prelude::ReadableExt;
use dioxus::prelude::WritableExt;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

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

#[test]
fn record_session_outcome_orders_denials_after_output_and_artifacts() {
    let mut session = make_session();
    let artifact = SessionArtifact::denied_command("bash", &["-lc", "pwd"], "blocked".into());
    let result = DispatchResult::Denied {
        command: "bash".to_string(),
        suggestion: Some("Use `help`.".to_string()),
        artifacts: vec![artifact.clone()],
    };

    record_session_outcome(
        &mut session,
        &result,
        &["stdout line".to_string()],
        &["stderr line".to_string()],
    );

    let events = session.events();
    let tail = &events[events.len() - 4..];
    assert!(matches!(
        &tail[2],
        SessionEvent::ArtifactCreated { artifact: a, .. } if a == &artifact
    ));
    assert!(matches!(
        &tail[3],
        SessionEvent::CommandDenied { command, .. } if command == "bash"
    ));
}

#[test]
fn record_session_outcome_closes_session_on_exit() {
    let mut session = make_session();
    record_session_outcome(&mut session, &DispatchResult::Exit { code: 7 }, &[], &[]);
    assert!(matches!(
        session.events().last(),
        Some(SessionEvent::SessionEnded { exit_code: 7, .. })
    ));
}

#[test]
fn record_session_outcome_increments_timestamps_for_multiple_executed_artifacts() {
    let mut session = make_session();
    let first = SessionArtifact::validation_run(
        "cargo check".to_string(),
        std::env::temp_dir(),
        0,
        "ok".to_string(),
        String::new(),
    );
    let second = SessionArtifact::validation_run(
        "cargo test".to_string(),
        std::env::temp_dir(),
        0,
        "ok".to_string(),
        String::new(),
    );

    record_session_outcome(
        &mut session,
        &DispatchResult::Executed {
            output: "ok".to_string(),
            exit_code: 0,
            artifacts: vec![first.clone(), second.clone()],
        },
        &[],
        &[],
    );

    let events = session.events();
    let tail = &events[events.len() - 2..];
    assert!(matches!(
        &tail[0],
        SessionEvent::ArtifactCreated { artifact, .. } if artifact == &first
    ));
    assert!(matches!(
        &tail[1],
        SessionEvent::ArtifactCreated { artifact, .. } if artifact == &second
    ));
}

#[test]
fn handle_submit_denied_command_renders_structured_denial() {
    with_runtime(|| {
        let (_runtime, session, lines, input_text, ctx) = make_ctx();

        handle_submit("bash".to_string(), "roy> ".to_string(), ctx);

        assert_eq!(&*input_text.read(), "");
        let rendered = lines.read();
        assert_eq!(rendered[1].kind, LineKind::DenialHeader);
        assert_eq!(rendered[2].kind, LineKind::DenialHint);

        let events = session.read();
        assert_eq!(events.events_of_kind("artifact_created").len(), 1);
        assert_eq!(events.events_of_kind("command_denied").len(), 1);
    });
}

#[test]
fn handle_submit_exit_renders_session_end_notice() {
    with_runtime(|| {
        let (_runtime, session, lines, _input_text, ctx) = make_ctx();

        handle_submit("exit 7".to_string(), "roy> ".to_string(), ctx);

        let rendered = lines.read();
        assert_eq!(rendered[1].text, "[session ended · exit 7]");

        let events = session.read();
        assert!(matches!(
            events.events().last(),
            Some(SessionEvent::SessionEnded { exit_code: 7, .. })
        ));
    });
}

#[test]
fn handle_submit_agent_active_forwards_raw_input_without_dispatch() {
    with_runtime(|| {
        let shared = Arc::new(Mutex::new(Vec::new()));
        let writer = SharedWriter {
            buf: Arc::clone(&shared),
        };
        let (mut runtime, session, lines, input_text, ctx) = make_ctx();

        runtime.write().agent_handle = Some({
            let mut handle = crate::agents::adapter::AgentHandle::new(
                crate::agents::adapter::AgentMeta {
                    kind: crate::agents::adapter::AgentKind::ClaudeCode,
                    version: "1.0.0".to_string(),
                    install_path: std::path::PathBuf::from("/usr/local/bin/claude"),
                },
                0,
            );
            handle.set_stdin(Arc::new(Mutex::new(Box::new(writer))));
            handle
        });

        handle_submit("continue".to_string(), "roy> ".to_string(), ctx);

        assert_eq!(&*input_text.read(), "");
        // PTY expects \r (carriage return) for Enter, not \n.
        assert_eq!(&*shared.lock().unwrap(), b"continue\r");

        let rendered = lines.read();
        assert_eq!(rendered.len(), 1);
        assert_eq!(rendered[0].kind, LineKind::Echo);
        assert_eq!(rendered[0].text, "continue");

        let events = session.read();
        assert_eq!(events.events_of_kind("user_input").len(), 1);
        assert_eq!(events.events_of_kind("command_invoked").len(), 0);
    });
}
