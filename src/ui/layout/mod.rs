mod atoms;
mod chrome;
mod footer;
mod panels;

use dioxus::prelude::*;

use chrome::Header;
use footer::{ArtifactsRow, DiagnosticsPane};
use panels::{ActivityPanel, ShellPane, WorkspacePanel};

// ── palette ──────────────────────────────────────────────────────────────────
// pub(crate) so sub-modules can import via `use super::*` or `use super::X`

pub(crate) const BG_BASE: &str = "#0d0f12";
pub(crate) const BG_PANEL: &str = "#161b22";
pub(crate) const BG_SHELL: &str = "#010409";
pub(crate) const BG_HEADER: &str = "#0d1117";
pub(crate) const BORDER: &str = "#30363d";
pub(crate) const TEXT_PRIMARY: &str = "#c9d1d9";
pub(crate) const TEXT_DIM: &str = "#6e7681";
pub(crate) const TEXT_ACCENT: &str = "#e8944a";
pub(crate) const TEXT_YELLOW: &str = "#d29922";

// ── root cockpit ─────────────────────────────────────────────────────────────

/// Root shell cockpit. Wraps the workspace header, three-column body
/// (workspace context | shell pane | activity/approvals), artifacts row,
/// and collapsible diagnostics pane.
#[component]
pub fn Cockpit() -> Element {
    let diag_open = use_signal(|| false);

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                width: 100vw;
                background: {BG_BASE};
                color: {TEXT_PRIMARY};
                font-family: 'JetBrains Mono', 'Cascadia Code', Consolas, monospace;
                font-size: 13px;
                line-height: 1.5;
                overflow: hidden;
                box-sizing: border-box;
            ",

            Header {}

            div {
                style: "display: flex; flex: 1; overflow: hidden; border-top: 1px solid {BORDER};",

                WorkspacePanel {}
                ShellPane {}
                ActivityPanel {}
            }

            ArtifactsRow {}
            DiagnosticsPane { open: diag_open }
        }
    }
}
