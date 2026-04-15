use dioxus::prelude::*;

use crate::agents::adapter::SupervisionEvent;
use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::super::is_session_active;
use super::super::terminal_model::{initial_shell_lines, ShellLine};
use super::terminal_composer::key_to_pty_bytes;
use super::terminal_grid::{new_term_handle, TermHandle};
use super::terminal_grid_render::{render_grid_row, wheel_lines};
use super::terminal_line::render_line;
use super::{handle_submit, SubmitContext};

#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let mut input_text = use_signal(String::new);
    let lines: Signal<Vec<ShellLine>> = use_signal(initial_shell_lines);
    let line_buf: Signal<String> = use_signal(String::new);
    let agent_term: Signal<TermHandle> = use_signal(new_term_handle);
    let mut grid_tick: Signal<u64> = use_signal(|| 0u64);

    use_future(move || poll_agent_output(runtime, lines, line_buf, agent_term, grid_tick));

    use_effect(move || {
        let _ = grid_tick.read(); // re-run when new output arrives
        let _ = document::eval(
            "(function(){\
                var s=document.getElementById('shell-output');\
                if(s&&s.scrollHeight-s.scrollTop-s.clientHeight<120)s.scrollTop=s.scrollHeight;\
            })();",
        );
    });

    let prompt = runtime.read().prompt();
    let session_closed = !is_session_active(&session.read());
    let agent_active = runtime.read().agent_active();
    let rendered_lines = lines.read().clone();
    let _tick = grid_tick.read();

    let grid_snapshot = if agent_active {
        Some(agent_term.read().lock().unwrap().snapshot())
    } else {
        None
    };

    let submit_shell = move || {
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
            id: "shell-output",
            style: "flex:1;padding:22px 28px;font-family:'JetBrains Mono',monospace;font-size:14px;line-height:1.75;overflow-y:auto;overflow-x:clip;color:{super::INK_DIM};",
            onclick: move |_| { let _ = document::eval("document.getElementById('pty-cap')?.focus();"); },
            onwheel: move |evt| {
                if !runtime.read().agent_active() {
                    return;
                }
                let lines = wheel_lines(evt.delta());
                if lines != 0 {
                    let handle = agent_term.read().clone();
                    handle.lock().unwrap().scroll_lines(lines);
                    *grid_tick.write() += 1;
                    evt.prevent_default();
                }
            },

            for line in rendered_lines.iter() { { render_line(line) } }

            if agent_active {
                div {
                    style: "color:{super::INK};margin-top:4px;overflow:hidden;font-family:'Cascadia Mono','JetBrains Mono','Consolas',monospace;font-size:14px;line-height:1;letter-spacing:0;font-kerning:none;font-variant-ligatures:none;font-feature-settings:'liga' 0,'calt' 0;text-rendering:optimizeSpeed;tab-size:8;",
                    {
                        let snap = grid_snapshot.as_ref().unwrap();
                        let cursor = snap.cursor;
                        let palette = snap.colors;
                        let rows = snap.rows.clone();
                        rows.into_iter().enumerate().map(move |(ri, row)| {
                            render_grid_row(row, ri, cursor, palette)
                        })
                    }
                }
                input {
                    id: "pty-cap",
                    r#type: "text", autofocus: true,
                    style: "position:fixed;left:-9999px;width:1px;height:1px;opacity:0;pointer-events:none;",
                    onkeydown: move |evt| {
                        let bytes = key_to_pty_bytes(&evt);
                        if let Some(bytes) = bytes {
                            let _ = runtime.read().send_agent_raw(&bytes);
                            evt.prevent_default();
                        }
                    },
                }
            } else {
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
                            id: "shell-input",
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

async fn poll_agent_output(
    mut runtime: Signal<ShellRuntime>,
    mut lines: Signal<Vec<ShellLine>>,
    mut line_buf: Signal<String>,
    mut agent_term: Signal<TermHandle>,
    mut grid_tick: Signal<u64>,
) {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(16)).await;
        if !runtime.peek().agent_active() {
            continue;
        }
        let (events, _exited) = runtime.write().poll_agent_events();
        if !events.is_empty()
            && apply_events(
                &mut runtime,
                events,
                &mut lines,
                &mut line_buf,
                &mut agent_term,
            )
        {
            *grid_tick.write() += 1;
        }
    }
}

fn apply_events(
    runtime: &mut Signal<ShellRuntime>,
    events: Vec<SupervisionEvent>,
    lines: &mut Signal<Vec<ShellLine>>,
    line_buf: &mut Signal<String>,
    agent_term: &mut Signal<TermHandle>,
) -> bool {
    let mut dirty = false;
    for event in events {
        match event {
            SupervisionEvent::OutputChunk { bytes, .. } => {
                let handle = agent_term.read().clone();
                let mut term = handle.lock().unwrap();
                let replies = term.feed(&bytes);
                term.scroll_to_bottom();
                drop(term);
                send_term_replies(runtime, replies);
                dirty = true;
            }
            SupervisionEvent::OutputLine { text } | SupervisionEvent::ErrorLine { text } => {
                let handle = agent_term.read().clone();
                let mut term = handle.lock().unwrap();
                let mut replies = term.feed(text.as_bytes());
                replies.extend(term.feed(b"\r\n"));
                term.scroll_to_bottom();
                drop(term);
                send_term_replies(runtime, replies);
                dirty = true;
            }
            SupervisionEvent::ProcessExited { code } => {
                let rows = agent_term.read().lock().unwrap().text_rows();
                let mut w = lines.write();
                for r in rows {
                    w.push(ShellLine::output(r));
                }
                w.push(ShellLine::output(format!(
                    "[claude-code exited · code {code}]"
                )));
                drop(w);
                let p = std::mem::take(&mut *line_buf.write());
                if !p.is_empty() {
                    lines.write().push(ShellLine::output(p));
                }
                *agent_term.write() = new_term_handle();
            }
            SupervisionEvent::AgentStarted { .. } => {
                *agent_term.write() = new_term_handle();
            }
            SupervisionEvent::CommandAttempt { command, args } => {
                let s = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args.join(" "))
                };
                lines
                    .write()
                    .push(ShellLine::output(format!("[attempt] {command}{s}")));
            }
        }
    }
    dirty
}

fn send_term_replies(runtime: &mut Signal<ShellRuntime>, replies: Vec<Vec<u8>>) {
    for reply in replies {
        let _ = runtime.write().send_agent_raw(&reply);
    }
}
