use dioxus::prelude::*;

use crate::agents::adapter::SupervisionEvent;
use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::super::is_session_active;
use super::super::terminal_model::{initial_shell_lines, ShellLine};
use super::terminal_line::render_line;
use super::terminal_grid::{new_term_handle, row_spans, TermHandle};
use super::{handle_submit, SubmitContext};

#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let mut input_text = use_signal(String::new);
    let lines: Signal<Vec<ShellLine>> = use_signal(initial_shell_lines);
    let line_buf: Signal<String> = use_signal(String::new);
    let agent_term: Signal<TermHandle> = use_signal(new_term_handle);

    use_future(move || poll_agent_output(runtime, lines, line_buf, agent_term));

    use_effect(move || {
        let _ = document::eval(
            "(function(){\
                var s=document.getElementById('shell-output');\
                if(s)s.scrollTop=s.scrollHeight;\
            })();",
        );
    });

    let prompt = runtime.read().prompt();
    let session_closed = !is_session_active(&session.read());
    let agent_active = runtime.read().agent_active();
    let rendered_lines = lines.read().clone();

    // Snapshot the styled grid rows once per render when agent is active.
    let grid_rows: Vec<Vec<(char, u32, u32, u16)>> = if agent_active {
        agent_term.read().lock().unwrap().styled_rows()
    } else {
        Vec::new()
    };

    let mut submit = move || {
        let raw = input_text.read().trim().to_string();
        if raw.is_empty() { return; }
        if agent_active {
            let bytes = format!("{raw}\r");
            let _ = runtime.write().send_agent_raw(bytes.as_bytes());
            *input_text.write() = String::new();
        } else {
            let pre_prompt = runtime.read().prompt();
            handle_submit(raw, pre_prompt, SubmitContext {
                runtime, session, lines, input_text,
            });
        }
    };

    rsx! {
        div {
            id: "shell-output",
            style: "
                flex:1; padding:22px 28px;
                font-family:'JetBrains Mono',monospace; font-size:14px;
                line-height:1.75; overflow-y:auto; color:{super::INK_DIM};
            ",

            for line in rendered_lines.iter() { { render_line(line) } }

            // Live terminal grid — shown only while an agent is running.
            if agent_active {
                div {
                    style: "color:{super::INK}; line-height:1.2; margin-top:4px;",
                    for (ri, row) in grid_rows.into_iter().enumerate() {
                        { render_grid_row(row, ri) }
                    }
                }
            }

            // Input row — always shown (sends to PTY or shell depending on mode).
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
                            font-size:14px; flex-shrink:0; white-space:nowrap;
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
                            font-size:14px; caret-color:{super::CORAL}; padding:0;
                        ",
                        oninput: move |evt| *input_text.write() = evt.value(),
                        onkeydown: move |evt| {
                            if agent_active && evt.modifiers().ctrl() {
                                if let Key::Character(ch) = &evt.key() {
                                    if let Some(c) = ch.chars().next() {
                                        let u = c.to_ascii_uppercase();
                                        if u.is_ascii_uppercase() {
                                            let byte = (u as u8) - b'@';
                                            let _ = runtime.write().send_agent_raw(&[byte]);
                                            evt.prevent_default();
                                            return;
                                        }
                                    }
                                }
                            }
                            if evt.key() == Key::Enter { submit(); }
                        },
                    }
                }
            }
        }
    }
}

fn render_grid_row(row: Vec<(char, u32, u32, u16)>, ri: usize) -> Element {
    let spans = row_spans(&row);
    rsx! {
        div { key: "{ri}", style: "white-space:pre; min-height:1.2em;",
            if spans.is_empty() {
                "\u{00a0}"
            }
            for (si, (text, css)) in spans.into_iter().enumerate() {
                span { key: "{si}", style: "{css}", "{text}" }
            }
        }
    }
}

async fn poll_agent_output(
    mut runtime: Signal<ShellRuntime>,
    mut lines: Signal<Vec<ShellLine>>,
    mut line_buf: Signal<String>,
    mut agent_term: Signal<TermHandle>,
) {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        if !runtime.peek().agent_active() { continue; }
        let (events, _exited) = runtime.write().poll_agent_events();
        if events.is_empty() { continue; }
        apply_events(events, &mut lines, &mut line_buf, &mut agent_term);
    }
}

fn apply_events(
    events: Vec<SupervisionEvent>,
    lines: &mut Signal<Vec<ShellLine>>,
    line_buf: &mut Signal<String>,
    agent_term: &mut Signal<TermHandle>,
) {
    for event in events {
        match event {
            SupervisionEvent::OutputChunk { bytes, .. } => {
                agent_term.read().lock().unwrap().feed(&bytes);
            }
            SupervisionEvent::OutputLine { text } | SupervisionEvent::ErrorLine { text } => {
                agent_term.read().lock().unwrap().feed(text.as_bytes());
                agent_term.read().lock().unwrap().feed(b"\r\n");
            }
            SupervisionEvent::ProcessExited { code } => {
                // Snapshot the final grid, append as transcript lines.
                let text_rows = agent_term.read().lock().unwrap().text_rows();
                {
                    let mut w = lines.write();
                    for row in text_rows { w.push(ShellLine::output(row)); }
                    w.push(ShellLine::output(format!("[claude-code exited · code {code}]")));
                }
                // Flush any partial shell line buffer.
                let partial = std::mem::take(&mut *line_buf.write());
                if !partial.is_empty() { lines.write().push(ShellLine::output(partial)); }
                // Reset the emulator for the next session.
                *agent_term.write() = new_term_handle();
            }
            SupervisionEvent::AgentStarted { .. } => {
                // Fresh emulator for each new session.
                *agent_term.write() = new_term_handle();
            }
            SupervisionEvent::CommandAttempt { command, args } => {
                let arg_str = if args.is_empty() { String::new() }
                    else { format!(" {}", args.join(" ")) };
                lines.write().push(ShellLine::output(format!("[attempt] {command}{arg_str}")));
            }
        }
    }
}
