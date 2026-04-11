use dioxus::prelude::*;

use super::{BORDER, TEXT_ACCENT, TEXT_DIM, TEXT_YELLOW};

// ── header ───────────────────────────────────────────────────────────────────

#[component]
pub(super) fn Header() -> Element {
    use super::BG_HEADER;
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
                    "controlled shell host"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 12px;",
                Badge { label: "workspace", value: "\u{2014}", color: TEXT_DIM }
                Badge { label: "policy",    value: "none",     color: TEXT_DIM }
                Badge { label: "agent",     value: "none",     color: TEXT_DIM }
            }

            div {
                style: "display: flex; align-items: center; gap: 8px;",
                StatusDot { color: TEXT_YELLOW }
                span {
                    style: "color: {TEXT_DIM}; font-size: 11px;",
                    "no session"
                }
            }
        }
    }
}

// ── badge ─────────────────────────────────────────────────────────────────────

#[component]
fn Badge(label: &'static str, value: &'static str, color: &'static str) -> Element {
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
