use super::test_support::*;
use super::*;
use dioxus::prelude::ReadableExt;

#[test]
fn initial_shell_lines_show_banner_and_help_hint() {
    let lines = initial_shell_lines();

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].text, "ROY - shell runtime ready");
    assert_eq!(lines[0].kind, LineKind::Output);
    assert_eq!(lines[1].text, "type 'help' for available commands");
    assert_eq!(lines[1].kind, LineKind::Output);
}

#[test]
fn flatten_chunks_splits_multiline_output_and_preserves_blanks() {
    let flattened = flatten_chunks(vec!["alpha\nbeta".to_string(), String::new()]);

    assert_eq!(
        flattened,
        vec!["alpha".to_string(), "beta".to_string(), String::new()]
    );
}

#[test]
fn handle_submit_parse_error_records_echo_and_error() {
    with_runtime(|| {
        let (_runtime, session, lines, input_text, ctx) = make_ctx();

        handle_submit("\"unterminated".to_string(), "roy> ".to_string(), ctx);

        assert_eq!(&*input_text.read(), "");
        let rendered = lines.read();
        assert_eq!(rendered.len(), 2);
        assert_eq!(rendered[0].prefix, "roy> ");
        assert_eq!(rendered[0].text, "\"unterminated");
        assert_eq!(rendered[0].kind, LineKind::Echo);
        assert_eq!(rendered[1].kind, LineKind::Error);
        assert!(rendered[1].text.contains("parse error"));

        let events = session.read();
        assert_eq!(events.events_of_kind("command_invoked").len(), 0);
        assert!(matches!(
            events.events().last(),
            Some(SessionEvent::CommandOutput { is_error: true, .. })
        ));
    });
}

#[test]
fn handle_submit_not_found_renders_registry_hint() {
    with_runtime(|| {
        let (_runtime, session, lines, _input_text, ctx) = make_ctx();

        handle_submit(
            "not_a_real_roy_command".to_string(),
            "roy> ".to_string(),
            ctx,
        );

        let rendered = lines.read();
        assert_eq!(rendered.len(), 2);
        assert_eq!(rendered[1].kind, LineKind::NotFound);
        assert!(rendered[1].text.contains("help"));

        let events = session.read();
        assert_eq!(events.events_of_kind("command_not_found").len(), 1);
        assert_eq!(events.events_of_kind("command_output").len(), 1);
    });
}

#[test]
fn handle_submit_success_renders_runtime_output() {
    with_runtime(|| {
        let (_runtime, session, lines, _input_text, ctx) = make_ctx();

        handle_submit("pwd".to_string(), "roy> ".to_string(), ctx);

        let rendered = lines.read();
        assert_eq!(rendered.len(), 2);
        assert_eq!(rendered[1].kind, LineKind::Output);
        assert!(!rendered[1].text.trim().is_empty());

        let events = session.read();
        assert_eq!(events.events_of_kind("command_invoked").len(), 1);
        assert_eq!(events.events_of_kind("command_output").len(), 1);
    });
}
