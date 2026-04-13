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
    let grid_tick: Signal<u64> = use_signal(|| 0u64);

    use_future(move || poll_agent_output(runtime, lines, line_buf, agent_term, grid_tick));

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
    let _tick = grid_tick.read();

    let grid_rows: Vec<Vec<(char, u32, u32, u16)>> = if agent_active {
        agent_term.read().lock().unwrap().styled_rows()
    } else {
        Vec::new()
    };

    let submit_shell = move || {
        let raw = input_text.read().trim().to_string();
        if raw.is_empty() { return; }
        let pre_prompt = runtime.read().prompt();
        handle_submit(raw, pre_prompt, SubmitContext { runtime, session, lines, input_text });
    };

    rsx! {
        div {
            id: "shell-output",
            style: "flex:1;padding:22px 28px;font-family:'JetBrains Mono',monospace;font-size:14px;line-height:1.75;overflow-y:auto;color:{super::INK_DIM};",
            // Clicking the terminal area re-focuses the capture input.
            onclick: move |_| { let _ = document::eval("document.getElementById('pty-cap')?.focus();"); },

            for line in rendered_lines.iter() { { render_line(line) } }

            if agent_active {
                // Grid: fills the role of the entire terminal surface.
                div {
                    style: "color:{super::INK};line-height:1.2;margin-top:4px;",
                    for (ri, row) in grid_rows.into_iter().enumerate() {
                        { render_grid_row(row, ri) }
                    }
                }
                // Off-screen input that captures every keystroke in character mode.
                input {
                    id: "pty-cap",
                    r#type: "text", value: "", autofocus: true,
                    style: "position:absolute;left:-9999px;width:1px;height:1px;opacity:0;",
                    onkeydown: move |evt| {
                        if evt.modifiers().ctrl() {
                            if let Key::Character(ch) = &evt.key() {
                                if let Some(c) = ch.chars().next() {
                                    let u = c.to_ascii_uppercase();
                                    if u.is_ascii_uppercase() {
                                        let _ = runtime.write().send_agent_raw(&[(u as u8) - b'@']);
                                        evt.prevent_default(); return;
                                    }
                                }
                            }
                        }
                        if let Some(seq) = nav_key_seq(&evt.key()) {
                            let _ = runtime.write().send_agent_raw(seq);
                            evt.prevent_default(); return;
                        }
                        match evt.key() {
                            Key::Enter => { let _ = runtime.write().send_agent_raw(b"\r"); }
                            Key::Backspace => { let _ = runtime.write().send_agent_raw(b"\x7f"); }
                            Key::Character(ch) if !evt.modifiers().ctrl() && !evt.modifiers().alt() => {
                                let _ = runtime.write().send_agent_raw(ch.as_bytes());
                            }
                            _ => return,
                        }
                        evt.prevent_default();
                    },
                }
            } else {
                // Shell mode: visible prompt + line-edit input.
                div {
                    style: "display:flex;align-items:center;gap:8px;margin-top:4px;",
                    if session_closed {
                        span { style: "color:{super::INK_FAINT};font-size:14px;font-style:italic;", "session ended" }
                    } else {
                        span {
                            style: "color:{super::CORAL};font-family:'JetBrains Mono',monospace;font-size:14px;flex-shrink:0;white-space:nowrap;",
                            "{prompt}"
                        }
                        input {
                            r#type: "text", value: "{input_text}", autofocus: true,
                            style: "flex:1;background:transparent;border:none;outline:none;color:{super::INK};font-family:'JetBrains Mono',monospace;font-size:14px;caret-color:{super::CORAL};padding:0;",
                            oninput: move |evt| *input_text.write() = evt.value(),
                            onkeydown: move |evt| { if evt.key() == Key::Enter { submit_shell(); } },
                        }
                    }
                }
            }
        }
    }
}

fn nav_key_seq(key: &Key) -> Option<&'static [u8]> {
    match key {
        Key::ArrowUp    => Some(b"\x1b[A"), Key::ArrowDown  => Some(b"\x1b[B"),
        Key::ArrowRight => Some(b"\x1b[C"), Key::ArrowLeft  => Some(b"\x1b[D"),
        Key::Escape     => Some(b"\x1b"),   Key::Tab        => Some(b"\x09"),
        Key::Home       => Some(b"\x1b[H"), Key::End        => Some(b"\x1b[F"),
        Key::PageUp     => Some(b"\x1b[5~"),Key::PageDown   => Some(b"\x1b[6~"),
        Key::Delete     => Some(b"\x1b[3~"),_ => None,
    }
}

fn render_grid_row(row: Vec<(char, u32, u32, u16)>, ri: usize) -> Element {
    let spans = row_spans(&row);
    rsx! {
        div { key: "{ri}", style: "white-space:pre;min-height:1.2em;",
            if spans.is_empty() { "\u{00a0}" }
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
    mut grid_tick: Signal<u64>,
) {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        if !runtime.peek().agent_active() { continue; }
        let (events, _exited) = runtime.write().poll_agent_events();
        if !events.is_empty() && apply_events(events, &mut lines, &mut line_buf, &mut agent_term) {
            *grid_tick.write() += 1;
        }
    }
}

fn apply_events(
    events: Vec<SupervisionEvent>,
    lines: &mut Signal<Vec<ShellLine>>,
    line_buf: &mut Signal<String>,
    agent_term: &mut Signal<TermHandle>,
) -> bool {
    let mut dirty = false;
    for event in events {
        match event {
            SupervisionEvent::OutputChunk { bytes, .. } => {
                agent_term.read().lock().unwrap().feed(&bytes);
                dirty = true;
            }
            SupervisionEvent::OutputLine { text } | SupervisionEvent::ErrorLine { text } => {
                let g = agent_term.read();
                let mut t = g.lock().unwrap();
                t.feed(text.as_bytes()); t.feed(b"\r\n");
                dirty = true;
            }
            SupervisionEvent::ProcessExited { code } => {
                let rows = agent_term.read().lock().unwrap().text_rows();
                let mut w = lines.write();
                for r in rows { w.push(ShellLine::output(r)); }
                w.push(ShellLine::output(format!("[claude-code exited · code {code}]")));
                drop(w);
                let p = std::mem::take(&mut *line_buf.write());
                if !p.is_empty() { lines.write().push(ShellLine::output(p)); }
                *agent_term.write() = new_term_handle();
            }
            SupervisionEvent::AgentStarted { .. } => { *agent_term.write() = new_term_handle(); }
            SupervisionEvent::CommandAttempt { command, args } => {
                let s = if args.is_empty() { String::new() } else { format!(" {}", args.join(" ")) };
                lines.write().push(ShellLine::output(format!("[attempt] {command}{s}")));
            }
        }
    }
    dirty
}
