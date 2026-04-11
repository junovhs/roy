use dioxus::prelude::*;

use super::{BORDER, TEXT_DIM, TEXT_PRIMARY, TEXT_YELLOW};

// ── panel header ─────────────────────────────────────────────────────────────

#[component]
pub(super) fn PanelHeader(title: &'static str) -> Element {
    rsx! {
        div {
            style: "
                padding: 5px 12px;
                border-bottom: 1px solid {BORDER};
                color: {TEXT_DIM};
                font-size: 10px;
                letter-spacing: 0.1em;
                flex-shrink: 0;
            ",
            "{title}"
        }
    }
}

// ── field ─────────────────────────────────────────────────────────────────────

#[component]
pub(super) fn Field(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 1px;",
            span {
                style: "color: {TEXT_DIM}; font-size: 10px; letter-spacing: 0.05em;",
                "{label}"
            }
            span {
                style: "color: {TEXT_PRIMARY}; font-size: 12px;",
                "{value}"
            }
        }
    }
}

// ── section label ─────────────────────────────────────────────────────────────

#[component]
pub(super) fn SectionLabel(text: &'static str) -> Element {
    rsx! {
        span {
            style: "color: {TEXT_DIM}; font-size: 10px; letter-spacing: 0.1em;",
            "{text}"
        }
    }
}

// ── placeholder line ──────────────────────────────────────────────────────────

#[component]
pub(super) fn PlaceholderLine(
    prefix: &'static str,
    prefix_color: &'static str,
    text: &'static str,
) -> Element {
    rsx! {
        div {
            style: "display: flex; gap: 8px; align-items: baseline;",
            span {
                style: "color: {prefix_color}; font-weight: bold; font-size: 12px;",
                "{prefix}"
            }
            span {
                style: "color: {TEXT_DIM}; font-size: 12px;",
                "{text}"
            }
        }
    }
}

// ── diag line ─────────────────────────────────────────────────────────────────

#[component]
pub(super) fn DiagLine(tag: &'static str, text: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; gap: 8px; align-items: baseline;",
            span {
                style: "color: {TEXT_YELLOW}; font-size: 11px; min-width: 60px;",
                "[{tag}]"
            }
            span {
                style: "color: {TEXT_DIM}; font-size: 11px;",
                "{text}"
            }
        }
    }
}
