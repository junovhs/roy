use super::test_support::*;
use super::*;
use crate::session::SessionArtifact;
use dioxus::prelude::ReadableExt;

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
