use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::shell::ShellRuntime;

use super::{short_path_label, CORAL, INK, INK_FAINT, LINE, MINT, PEACH};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// ── header / marquee ─────────────────────────────────────────────────────────

/// Top bar: Roy brand mark, session strip, and health chip.
#[component]
pub(super) fn Header(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let runtime = runtime.read();
    let session = session.read();
    let workspace = short_path_label(runtime.workspace_root());
    let policy = runtime.policy_name().to_string();
    let event_count = session.len();
    let active = !matches!(
        session.events().last(),
        Some(SessionEvent::SessionEnded { .. })
    );

    let (dot_bg, dot_shadow, chip_color, status_text) = if active {
        if event_count > 2 {
            (MINT, "rgba(168,197,180,.4)", MINT, "Running normally")
        } else {
            (PEACH, "rgba(232,180,148,.4)", PEACH, "Idle")
        }
    } else {
        ("#5f5d58", "none", "#5f5d58", "Session ended")
    };

    rsx! {
        div {
            style: "
                padding: 18px 28px 14px 70px;
                display: flex;
                align-items: center;
                gap: 20px;
                border-bottom: 1px solid {LINE};
                position: relative;
                z-index: 4;
                flex-shrink: 0;
            ",

            // ── brand mark ─────────────────────────────────────────────────
            span {
                style: "
                    font-family: 'Fraunces', Georgia, serif;
                    font-size: 26px;
                    font-weight: 300;
                    font-style: italic;
                    color: {CORAL};
                    line-height: 1;
                    letter-spacing: -0.01em;
                    flex-shrink: 0;
                ",
                "ROY"
            }

            // ── session strip ───────────────────────────────────────────────
            div {
                style: "
                    flex: 1;
                    display: flex;
                    align-items: center;
                    gap: 22px;
                    padding-left: 22px;
                    border-left: 1px solid {LINE};
                ",
                StripItem { label: "Agent", value: "local-shell".to_string() }
                StripItem { label: "Workspace", value: workspace }
                StripItem { label: "Policy", value: policy }
                StripItem { label: "v", value: format!("v{APP_VERSION}") }
            }

            // ── health chip ─────────────────────────────────────────────────
            div {
                style: "
                    margin-left: auto;
                    display: flex;
                    align-items: center;
                    gap: 9px;
                    padding: 6px 13px;
                    border: 1px solid {LINE};
                    border-radius: 100px;
                    background: rgba(255,255,255,.02);
                    flex-shrink: 0;
                ",
                div {
                    style: "
                        width: 6px;
                        height: 6px;
                        border-radius: 50%;
                        background: {dot_bg};
                        box-shadow: 0 0 8px {dot_shadow};
                        animation: pulse 2.4s ease-in-out infinite;
                        flex-shrink: 0;
                    "
                }
                span {
                    style: "
                        font-family: 'Geist', sans-serif;
                        font-size: 13.5px;
                        color: {chip_color};
                        font-weight: 400;
                        white-space: nowrap;
                    ",
                    "{status_text}"
                }
            }
        }
    }
}

// ── strip item ────────────────────────────────────────────────────────────────

#[component]
fn StripItem(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: baseline; gap: 7px;",
            span {
                style: "font-size: 12px; color: {INK_FAINT}; font-weight: 400;",
                "{label}"
            }
            span {
                style: "font-size: 13px; color: {INK}; font-weight: 400;",
                "{value}"
            }
        }
    }
}
