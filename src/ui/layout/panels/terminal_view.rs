use dioxus::prelude::*;

use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::super::{is_session_active, short_path_label};
use super::super::terminal_model::{initial_shell_lines, ShellLine};
use super::{handle_submit, SubmitContext};
use super::terminal_composer::TerminalComposer;
use super::terminal_line::render_line;

#[component]
pub(crate) fn ShellPane(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let input_text = use_signal(String::new);
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
    let workspace = short_path_label(runtime.read().workspace_root());
    let session_closed = !is_session_active(&session.read());

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
                    }
                }

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

                    for line in lines.read().iter() {
                        { render_line(line) }
                    }
                }
            }

            TerminalComposer {
                prompt,
                session_closed,
                input_text,
                on_submit: submit,
            }
        }
    }
}
