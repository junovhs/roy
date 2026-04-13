use dioxus::prelude::*;

use crate::agents::adapter::SupervisionEvent;
use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::super::is_session_active;
use super::super::terminal_model::{initial_shell_lines, ShellLine};
use super::terminal_composer::key_to_pty_bytes;
use super::terminal_emulator::AgentTerminalEmulator;
use super::terminal_line::render_line;
use super::terminal_surface::{render_row_range, visible_row_count};
use super::{handle_submit, SubmitContext};

#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let mut input_text = use_signal(String::new);
    let lines: Signal<Vec<ShellLine>> = use_signal(initial_shell_lines);
    let agent_terminal: Signal<AgentTerminalEmulator> = use_signal(AgentTerminalEmulator::default);

    use_future(move || poll_agent_output(runtime, lines, agent_terminal));

    // Scroll to bottom each render; focus the terminal div when agent is active
    // so that onkeydown can forward keystrokes to the PTY without a separate input widget.
    use_effect(move || {
        let _ = document::eval(
            "(function(){\
                var s=document.getElementById('shell-output');\
                if(!s)return;\
                s.scrollTop=s.scrollHeight;\
                if(s.tabIndex>=0)s.focus();\
            })();",
        );
    });

    let prompt = runtime.read().prompt();
    let session_closed = !is_session_active(&session.read());
    let agent_active = runtime.read().agent_active();
    let agent_snapshot = agent_terminal.read().snapshot();
    let rendered_lines = lines.read().clone();

    // Pre-compute agent rows so we can pass slices into rsx!.
    let inline_agent = agent_snapshot.as_ref().map(|s| {
        let vis = visible_row_count(s);
        (s.rows[..vis.min(s.rows.len())].to_vec(), s.cursor)
    });

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
        // Single unified terminal area — no mode switch, no separate panels.
        // Agent output appears inline after the transcript, exactly like any other
        // program's output in a real terminal.
        div {
            id: "shell-output",
            // tabindex=0 makes the div focusable when agent is running so onkeydown
            // can forward raw bytes to the PTY. tabindex=-1 when shell mode because
            // the <input> element handles focus instead.
            tabindex: if agent_active { 0 } else { -1 },
            style: "
                flex:1;
                padding:22px 28px;
                font-family:'JetBrains Mono',monospace;
                font-size:14px;
                line-height:1.75;
                overflow-y:auto;
                color:{super::INK_DIM};
                outline:none;
            ",
            onkeydown: move |evt| {
                if agent_active {
                    evt.prevent_default();
                    if let Some(bytes) = key_to_pty_bytes(&evt) {
                        let _ = runtime.write().send_agent_raw(&bytes);
                    }
                }
            },

            // Transcript lines (always visible)
            for line in rendered_lines.iter() {
                { render_line(line) }
            }

            // Agent TUI rows appear inline after transcript — no takeover, no separate panel.
            if let Some((ref rows, cursor)) = inline_agent {
                { render_row_range(rows, 0, cursor) }
            }

            // Inline prompt + input — only shown in shell mode.
            // When agent is active, the agent draws its own prompt in its TUI rows above.
            if !agent_active {
                div {
                    style: "display:flex; align-items:center; gap:8px; margin-top:4px;",
                    if session_closed {
                        span {
                            style: "color:{super::INK_FAINT}; font-size:14px; font-style:italic;",
                            "session ended"
                        }
                    } else {
                        span {
                            style: "
                                color:{super::CORAL};
                                font-family:'JetBrains Mono',monospace;
                                font-size:14px;
                                flex-shrink:0;
                                white-space:nowrap;
                            ",
                            "{prompt}"
                        }
                        input {
                            r#type: "text",
                            value: "{input_text}",
                            autofocus: true,
                            style: "
                                flex:1; background:transparent; border:none; outline:none;
                                color:{super::INK};
                                font-family:'JetBrains Mono',monospace;
                                font-size:14px;
                                caret-color:{super::CORAL};
                                padding:0;
                            ",
                            oninput: move |evt| *input_text.write() = evt.value(),
                            onkeydown: move |evt| {
                                if evt.key() == Key::Enter { submit(); }
                            },
                        }
                    }
                }
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
