use dioxus::prelude::*;

#[component]
pub(super) fn TerminalComposer(
    prompt: String,
    session_closed: bool,
    agent_active: bool,
    input_text: Signal<String>,
    on_submit: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "padding: 12px 0 0; flex-shrink: 0;",

            div {
                style: "
                    background: {super::SURFACE_2};
                    border: 1px solid {super::LINE};
                    border-radius: 10px;
                    padding: 12px 14px;
                    display: flex;
                    flex-direction: column;
                    gap: 8px;
                ",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span {
                        style: "color: {super::CORAL}; font-family: 'JetBrains Mono', monospace; font-size: 14px; flex-shrink: 0;",
                        "{prompt}"
                    }
                    if session_closed {
                        span {
                            style: "color: {super::INK_FAINT}; font-size: 14px; font-style: italic;",
                            "session ended"
                        }
                    } else {
                        if agent_active {
                            span {
                                style: "color: {super::INK_FAINT}; font-size: 14px; font-style: italic;",
                                "agent running\u{2026}"
                            }
                        }
                        input {
                            r#type: "text",
                            value: "{input_text}",
                            autofocus: true,
                            placeholder: if agent_active { "Send input to agent\u{2026}" } else { "Enter a command\u{2026}" },
                            style: "
                                flex: 1;
                                background: transparent;
                                border: none;
                                outline: none;
                                color: {super::INK};
                                font-family: 'Geist', sans-serif;
                                font-size: 15px;
                                caret-color: {super::CORAL};
                                padding: 0;
                                font-weight: 400;
                            ",
                            oninput: move |evt| input_text.set(evt.value()),
                            onkeydown: move |evt| {
                                if evt.key() == Key::Enter {
                                    on_submit.call(());
                                }
                            },
                        }
                    }
                }

                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    div {
                        style: "display: flex; gap: 2px;",
                        for label in ["+", "\u{25eb}", "@"] {
                            button {
                                style: "
                                    background: none;
                                    border: none;
                                    color: {super::INK_FAINT};
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
                                background: {super::CORAL};
                                color: #1a1b1e;
                                border: none;
                                cursor: pointer;
                                font-family: 'Geist', sans-serif;
                                font-size: 13px;
                                font-weight: 500;
                                transition: all .15s;
                            ",
                            onclick: move |_| on_submit.call(()),
                            "Send \u{23ce}"
                        }
                    }
                }
            }
        }
    }
}
