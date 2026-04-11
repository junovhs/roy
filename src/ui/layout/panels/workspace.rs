use dioxus::prelude::*;

use crate::session::Session;
use crate::shell::ShellRuntime;

use super::super::atoms::{Field, PanelHeader, SectionLabel};
use super::super::{relative_scope_label, short_path_label, BG_PANEL, BORDER, TEXT_DIM};

#[component]
pub(crate) fn WorkspacePanel(runtime: Signal<ShellRuntime>, session: Signal<Session>) -> Element {
    let runtime = runtime.read();
    let session = session.read();
    let root = runtime.workspace_root().display().to_string();
    let cwd = runtime.env().cwd().display().to_string();
    let scope = relative_scope_label(runtime.workspace_root(), runtime.env().cwd());
    let session_value = format!("#{} · {} events", session.id, session.len());

    rsx! {
        div {
            style: "
                width: 260px;
                flex-shrink: 0;
                background: {BG_PANEL};
                border-right: 1px solid {BORDER};
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ",

            PanelHeader { title: "WORKSPACE" }

            div {
                style: "padding: 12px; display: flex; flex-direction: column; gap: 10px; overflow-y: auto; flex: 1;",

                Field { label: "root", value: root }
                Field { label: "cwd", value: cwd }
                Field { label: "scope", value: scope }
                Field { label: "session", value: session_value }

                div {
                    style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid {BORDER};",
                    SectionLabel { text: "POLICY PROFILE" }
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px; margin-top: 4px;",
                        {runtime.policy_name()}
                    }
                }

                div {
                    style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid {BORDER};",
                    SectionLabel { text: "HOST IDENTITY" }
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px; margin-top: 4px;",
                        {format!(
                            "ROY controls {} as a scoped shell world",
                            short_path_label(runtime.workspace_root())
                        )}
                    }
                }

                div {
                    style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid {BORDER};",
                    SectionLabel { text: "INSTALLED AGENTS" }
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px; margin-top: 4px;",
                        "embedded adapters pending AGEN-01"
                    }
                }
            }
        }
    }
}
