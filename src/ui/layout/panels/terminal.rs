use dioxus::prelude::*;

use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::atoms::PanelHeader;
use super::super::{
    is_session_active, BG_PANEL, BG_SHELL, BORDER, TEXT_ACCENT, TEXT_DIM, TEXT_PRIMARY,
};
use super::terminal_model::{handle_submit, initial_shell_lines, ShellLine, SubmitContext, TEXT_ERROR};

#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let mut input_text = use_signal(String::new);
    let lines: Signal<Vec<ShellLine>> = use_signal(initial_shell_lines);

    use_effect(|| {
        let _ = document::eval(
            "(function(){\
                var e=document.getElementById('shell-output');\
                if(e)e.scrollTop=e.scrollHeight;\
            })();",
        );
    });

    let prompt = runtime.read().prompt();
    let session_closed = !is_session_active(&session.read());

    rsx! {
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                overflow: hidden;
                background: {BG_SHELL};
                border-right: 1px solid {BORDER};
            ",

            PanelHeader { title: "SHELL" }

            div {
                id: "shell-output",
                style: "
                    flex: 1;
                    padding: 12px 16px;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column;
                    gap: 1px;
                    font-size: 12px;
                ",

                for line in lines.read().iter() {
                    div {
                        style: format!(
                            "display: flex; gap: 8px; color: {}; white-space: pre-wrap; word-break: break-all; line-height: 1.6;",
                            if line.is_error { TEXT_ERROR } else { TEXT_PRIMARY }
                        ),
                        if !line.prefix.is_empty() {
                            span {
                                style: "color: {TEXT_ACCENT}; flex-shrink: 0; font-weight: bold;",
                                "{line.prefix}"
                            }
                        }
                        span { "{line.text}" }
                    }
                }
            }

            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 8px 16px;
                    border-top: 1px solid {BORDER};
                    background: {BG_PANEL};
                    flex-shrink: 0;
                ",

                span {
                    style: "color: {TEXT_ACCENT}; font-weight: bold; user-select: none; font-size: 12px; flex-shrink: 0;",
                    "{prompt}"
                }

                if session_closed {
                    span {
                        style: "color: {TEXT_DIM}; font-style: italic; font-size: 12px;",
                        "session ended"
                    }
                } else {
                    input {
                        r#type: "text",
                        value: "{input_text}",
                        autofocus: true,
                        style: "
                            flex: 1;
                            background: transparent;
                            border: none;
                            outline: none;
                            color: {TEXT_PRIMARY};
                            font-family: inherit;
                            font-size: 12px;
                            caret-color: {TEXT_ACCENT};
                            padding: 0;
                        ",
                        oninput: move |evt| {
                            input_text.set(evt.value());
                        },
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
                                SubmitContext {
                                    runtime,
                                    session,
                                    lines,
                                    input_text,
                                },
                            );
                        },
                    }
                }
            }
        }
    }
}
