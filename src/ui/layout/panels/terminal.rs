use dioxus::prelude::*;

use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::{is_session_active, short_path_label};
use super::terminal_model::{
    handle_submit, initial_shell_lines, ShellLine, SubmitContext, TEXT_ERROR,
};

// ── palette (prototype) ───────────────────────────────────────────────────────
const SURFACE: &str = "#16171a";
const SURFACE_2: &str = "#1c1d21";
const LINE: &str = "rgba(255,255,255,.06)";
const INK: &str = "#e6e4df";
const INK_DIM: &str = "#9b9892";
const INK_FAINT: &str = "#5f5d58";
const CORAL: &str = "#e87858";
const CORAL_SOFT: &str = "#f09a7e";

// ── shell pane ────────────────────────────────────────────────────────────────

/// Full-height terminal pod: rounded frame + scrollable output + composer.
#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let mut input_text = use_signal(String::new);
    let lines: Signal<Vec<ShellLine>> = use_signal(initial_shell_lines);

    // Auto-scroll to bottom whenever lines change.
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

    rsx! {
        // Outer: fills the pod-wrapper in Cockpit
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                min-height: 0;
            ",

            // ── pod-frame (the rounded terminal window) ───────────────────
            div {
                style: "
                    flex: 1;
                    background: {SURFACE};
                    border-radius: 10px;
                    border: 1px solid {LINE};
                    box-shadow: 0 20px 50px rgba(0,0,0,.3), inset 0 1px 0 rgba(255,255,255,.03);
                    display: flex;
                    flex-direction: column;
                    min-height: 0;
                    overflow: hidden;
                    position: relative;
                ",

                // ── terminal head (3 dots + workspace label) ──────────────
                div {
                    style: "
                        padding: 14px 22px 12px;
                        display: flex;
                        align-items: center;
                        gap: 10px;
                        border-bottom: 1px solid {LINE};
                        position: relative;
                        z-index: 2;
                        flex-shrink: 0;
                    ",
                    div { style: "width:6px;height:6px;border-radius:50%;background:{INK_DIM};opacity:.5;" }
                    div { style: "width:6px;height:6px;border-radius:50%;background:{INK_DIM};opacity:.5;" }
                    div { style: "width:6px;height:6px;border-radius:50%;background:{INK_DIM};opacity:.5;" }
                    span {
                        style: "
                            font-family: 'JetBrains Mono', monospace;
                            font-size: 12px;
                            color: {INK_FAINT};
                            margin-left: 6px;
                            letter-spacing: .02em;
                        ",
                        "session · "
                        em { style: "color: {INK_DIM}; font-style: normal;", "{workspace}" }
                    }
                }

                // ── terminal body (.tb) ────────────────────────────────────
                div {
                    id: "shell-output",
                    style: "
                        flex: 1;
                        padding: 22px 28px;
                        font-family: 'JetBrains Mono', monospace;
                        font-size: 14px;
                        line-height: 1.75;
                        overflow-y: auto;
                        color: {INK_DIM};
                        position: relative;
                        z-index: 2;
                    ",

                    for line in lines.read().iter() {
                        div {
                            style: format!(
                                "display:flex;gap:8px;white-space:pre-wrap;word-break:break-all;color:{};",
                                if line.is_error { TEXT_ERROR } else { INK }
                            ),
                            if !line.prefix.is_empty() {
                                span {
                                    style: "color:{CORAL_SOFT};flex-shrink:0;font-weight:bold;",
                                    "{line.prefix}"
                                }
                            }
                            span { "{line.text}" }
                        }
                    }
                }
            }

            // ── composer (below the pod-frame) ────────────────────────────
            div {
                style: "padding: 12px 0 0; flex-shrink: 0;",

                div {
                    style: "
                        background: {SURFACE_2};
                        border: 1px solid {LINE};
                        border-radius: 10px;
                        padding: 12px 14px;
                        display: flex;
                        flex-direction: column;
                        gap: 8px;
                    ",

                    // Input row
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span {
                            style: "color: {CORAL}; font-family: 'JetBrains Mono', monospace; font-size: 14px; flex-shrink: 0;",
                            "{prompt}"
                        }
                        if session_closed {
                            span {
                                style: "color: {INK_FAINT}; font-size: 14px; font-style: italic;",
                                "session ended"
                            }
                        } else {
                            input {
                                r#type: "text",
                                value: "{input_text}",
                                autofocus: true,
                                placeholder: "Enter a command…",
                                style: "
                                    flex: 1;
                                    background: transparent;
                                    border: none;
                                    outline: none;
                                    color: {INK};
                                    font-family: 'Geist', sans-serif;
                                    font-size: 15px;
                                    caret-color: {CORAL};
                                    padding: 0;
                                    font-weight: 400;
                                ",
                                oninput: move |evt| input_text.set(evt.value()),
                                onkeydown: move |evt| {
                                    if evt.key() != Key::Enter {
                                        return;
                                    }
                                    let raw = input_text.read().trim().to_string();
                                    if raw.is_empty() {
                                        return;
                                    }
                                    let pre_prompt = runtime.read().prompt();
                                    handle_submit(
                                        raw,
                                        pre_prompt,
                                        SubmitContext { runtime, session, lines, input_text },
                                    );
                                },
                            }
                        }
                    }

                    // Bottom row: tool buttons + send
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        div {
                            style: "display: flex; gap: 2px;",
                            for label in ["+", "◫", "@"] {
                                button {
                                    style: "
                                        background: none;
                                        border: none;
                                        color: {INK_FAINT};
                                        width: 26px;
                                        height: 26px;
                                        border-radius: 5px;
                                        cursor: pointer;
                                        font-size: 15px;
                                    ",
                                    "{label}"
                                }
                            }
                        }
                        if !session_closed {
                            button {
                                style: "
                                    padding: 6px 14px;
                                    border-radius: 6px;
                                    background: {CORAL};
                                    color: #1a1b1e;
                                    border: none;
                                    cursor: pointer;
                                    font-family: 'Geist', sans-serif;
                                    font-size: 13px;
                                    font-weight: 500;
                                    transition: all .15s;
                                ",
                                onclick: move |_| {
                                    let raw = input_text.read().trim().to_string();
                                    if raw.is_empty() {
                                        return;
                                    }
                                    let pre_prompt = runtime.read().prompt();
                                    handle_submit(
                                        raw,
                                        pre_prompt,
                                        SubmitContext { runtime, session, lines, input_text },
                                    );
                                },
                                "Send \u{23ce}"
                            }
                        }
                    }
                }
            }
        }
    }
}
