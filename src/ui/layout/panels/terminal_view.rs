use dioxus::prelude::*;

use crate::agents::adapter::SupervisionEvent;
use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::super::{is_session_active, short_path_label};
use super::super::terminal_model::{initial_shell_lines, ShellLine};
use super::terminal_composer::TerminalComposer;
use super::terminal_emulator::{AgentTerminalEmulator, TerminalSnapshot};
use super::terminal_line::render_line;
use super::terminal_surface::render_terminal_surface;
use super::{handle_submit, SubmitContext};

#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let input_text = use_signal(String::new);
    let lines: Signal<Vec<ShellLine>> = use_signal(initial_shell_lines);
    let agent_terminal: Signal<AgentTerminalEmulator> = use_signal(AgentTerminalEmulator::default);

    use_future(move || poll_agent_output(runtime, lines, agent_terminal));

    use_effect(|| {
        let _ = document::eval(
            "(function(){\
                var e=document.getElementById('shell-output');\
                if(e)e.scrollTop=e.scrollHeight;\
            })();",
        );
    });

    let prompt = runtime.read().prompt();
    let workspace = short_path_label(runtime.read().workspace_root());
    let session_closed = !is_session_active(&session.read());
    let agent_active = runtime.read().agent_active();
    let agent_snapshot = agent_terminal.read().snapshot();
    let rendered_lines = lines.read().clone();

    let submit = move || {
        let raw = input_text.read().trim().to_string();
        if raw.is_empty() {
            return;
        }
        let pre_prompt = runtime.read().prompt();
        handle_submit(
            raw,
            pre_prompt,
            SubmitContext {
                runtime,
                session,
                lines,
                input_text,
            },
        );
    };

    rsx! {
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                min-height: 0;
            ",

            div {
                style: "
                    flex: 1;
                    background: {super::SURFACE};
                    border-radius: 10px;
                    border: 1px solid {super::LINE};
                    box-shadow: 0 20px 50px rgba(0,0,0,.3), inset 0 1px 0 rgba(255,255,255,.03);
                    display: flex;
                    flex-direction: column;
                    min-height: 0;
                    overflow: hidden;
                    position: relative;
                ",

                ShellPaneHeader { workspace, agent_active }
                ShellPaneOutput { lines: rendered_lines, agent_snapshot }
            }

            TerminalComposer {
                prompt,
                session_closed,
                agent_active,
                input_text,
                on_submit: submit,
            }
        }
    }
}

async fn poll_agent_output(
    mut runtime: Signal<ShellRuntime>,
    mut lines: Signal<Vec<ShellLine>>,
    mut agent_terminal: Signal<AgentTerminalEmulator>,
) {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        if !runtime.peek().agent_active() {
            continue;
        }
        let (events, _exited) = runtime.write().poll_agent_events();
        if events.is_empty() {
            continue;
        }
        apply_agent_events(events, &mut lines, &mut agent_terminal);
    }
}

fn apply_agent_events(
    events: Vec<SupervisionEvent>,
    lines: &mut Signal<Vec<ShellLine>>,
    agent_terminal: &mut Signal<AgentTerminalEmulator>,
) {
    for event in events {
        match event {
            SupervisionEvent::OutputChunk { bytes, .. } => {
                agent_terminal.write().apply_bytes(&bytes);
            }
            SupervisionEvent::OutputLine { text } | SupervisionEvent::ErrorLine { text } => {
                agent_terminal.write().apply_bytes(text.as_bytes());
                agent_terminal.write().apply_bytes(b"\n");
            }
            SupervisionEvent::ProcessExited { code } => {
                let preserved = agent_terminal.write().finish_for_transcript();
                if !preserved.is_empty() {
                    lines.write().extend(preserved);
                }
                lines.write().push(ShellLine::output(format!(
                    "[claude-code exited · code {code}]"
                )));
            }
            SupervisionEvent::CommandAttempt { command, args } => {
                let arg_str = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args.join(" "))
                };
                lines
                    .write()
                    .push(ShellLine::output(format!("[attempt] {command}{arg_str}")));
            }
            SupervisionEvent::AgentStarted { .. } => {}
        }
    }
}

#[component]
fn ShellPaneHeader(workspace: String, agent_active: bool) -> Element {
    rsx! {
        div {
            style: "
                padding: 14px 22px 12px;
                display: flex;
                align-items: center;
                gap: 10px;
                border-bottom: 1px solid {super::LINE};
                position: relative;
                z-index: 2;
                flex-shrink: 0;
            ",
            div { style: "width:6px;height:6px;border-radius:50%;background:{super::INK_DIM};opacity:.5;" }
            div { style: "width:6px;height:6px;border-radius:50%;background:{super::INK_DIM};opacity:.5;" }
            div { style: "width:6px;height:6px;border-radius:50%;background:{super::INK_DIM};opacity:.5;" }
            span {
                style: "
                    font-family: 'JetBrains Mono', monospace;
                    font-size: 12px;
                    color: {super::INK_FAINT};
                    margin-left: 6px;
                    letter-spacing: .02em;
                ",
                "session · "
                em { style: "color: {super::INK_DIM}; font-style: normal;", "{workspace}" }
                if agent_active {
                    em { style: "color: {super::CORAL_SOFT}; font-style: normal; margin-left: 10px;", "· claude-code running" }
                }
            }
        }
    }
}

#[component]
fn ShellPaneOutput(lines: Vec<ShellLine>, agent_snapshot: Option<TerminalSnapshot>) -> Element {
    rsx! {
        div {
            id: "shell-output",
            style: "
                flex: 1;
                padding: 22px 28px;
                font-family: 'JetBrains Mono', monospace;
                font-size: 14px;
                line-height: 1.75;
                overflow-y: auto;
                color: {super::INK_DIM};
                position: relative;
                z-index: 2;
            ",

            for line in lines.iter() {
                { render_line(line) }
            }

            if let Some(snapshot) = agent_snapshot.as_ref() {
                { render_terminal_surface(snapshot) }
            }
        }
    }
}
