use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::shell::ShellRuntime;

use super::{short_path_label, BORDER, TEXT_ACCENT, TEXT_DIM, TEXT_PRIMARY};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_ACTIVE: &str = "#3fb950";
const STATUS_IDLE: &str = "#d29922";
const STATUS_DONE: &str = "#6e7681";

// ── header ───────────────────────────────────────────────────────────────────

#[component]
pub(super) fn Header(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    use super::BG_HEADER;

    let runtime = runtime.read();
    let session = session.read();
    let workspace = short_path_label(runtime.workspace_root());
    let policy = runtime.policy_name().to_string();
    let active = !matches!(
        session.events().last(),
        Some(SessionEvent::SessionEnded { .. })
    );
    let status_color = if active {
        if session.len() > 1 {
            STATUS_ACTIVE
        } else {
            STATUS_IDLE
        }
    } else {
        STATUS_DONE
    };
    let status_text = if active {
        format!("session #{} · {} events", session.id, session.len())
    } else {
        format!("session #{} ended", session.id)
    };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 6px 16px;
                background: {BG_HEADER};
                border-bottom: 1px solid {BORDER};
                flex-shrink: 0;
            ",

            div {
                style: "display: flex; align-items: center; gap: 16px;",
                span {
                    style: "color: {TEXT_ACCENT}; font-weight: bold; letter-spacing: 0.08em;",
                    "ROY"
                }
                span {
                    style: "color: {TEXT_DIM}; font-size: 11px;",
                    "v{APP_VERSION} - controlled shell host"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 12px;",
                Badge { label: "workspace", value: workspace, color: TEXT_PRIMARY }
                Badge { label: "policy", value: policy, color: TEXT_PRIMARY }
                Badge { label: "agent", value: "local shell".to_string(), color: TEXT_DIM }
                Badge { label: "release", value: format!("v{APP_VERSION}"), color: TEXT_DIM }
            }

            div {
                style: "display: flex; align-items: center; gap: 8px;",
                StatusDot { color: status_color }
                span {
                    style: "color: {TEXT_DIM}; font-size: 11px;",
                    "{status_text}"
                }
            }
        }
    }
}

// ── badge ─────────────────────────────────────────────────────────────────────

#[component]
fn Badge(label: &'static str, value: String, color: &'static str) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 4px;
                background: #1c2128;
                border: 1px solid {BORDER};
                border-radius: 4px;
                padding: 2px 8px;
            ",
            span { style: "color: {TEXT_DIM}; font-size: 10px;", "{label}" }
            span { style: "color: {color}; font-size: 10px;",    "{value}" }
        }
    }
}

// ── status dot ────────────────────────────────────────────────────────────────

#[component]
fn StatusDot(color: &'static str) -> Element {
    rsx! {
        div {
            style: "width: 7px; height: 7px; border-radius: 50%; background: {color};"
        }
    }
}
