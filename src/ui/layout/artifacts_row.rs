use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::ui::artifacts::{artifact_kind_color, artifact_kind_label, ArtifactViewer};

use super::atoms::StatCard;
use super::{short_path_label, BG_PANEL, BORDER, TEXT_DIM};

#[component]
pub(super) fn ArtifactsRow(session: Signal<Session>) -> Element {
    let mut selected_index = use_signal(|| 0usize);
    let session = session.read();
    let artifacts: Vec<_> = session.artifacts().into_iter().rev().cloned().collect();
    let active_index = if artifacts.is_empty() {
        0
    } else {
        (*selected_index.read()).min(artifacts.len() - 1)
    };
    let latest_cwd = session
        .events()
        .iter()
        .rev()
        .find_map(|event| match event {
            SessionEvent::CwdChanged { to, .. } => Some(short_path_label(to)),
            _ => None,
        })
        .unwrap_or_else(|| "workspace root".to_string());

    rsx! {
        div {
            style: "
                min-height: 88px;
                flex-shrink: 0;
                background: {BG_PANEL};
                border-top: 1px solid {BORDER};
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ",

            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 4px 12px;
                    border-bottom: 1px solid {BORDER};
                ",
                span {
                    style: "color: {TEXT_DIM}; font-size: 10px; letter-spacing: 0.1em;",
                    "ARTIFACTS"
                }
                span {
                    style: "color: {TEXT_DIM}; font-size: 10px;",
                    "{artifacts.len()} promoted artifacts · scope {latest_cwd}"
                }
            }

            div {
                style: "
                    min-height: 160px;
                    display: flex;
                    align-items: stretch;
                    gap: 10px;
                    padding: 10px 16px;
                    overflow: hidden;
                ",
                if artifacts.is_empty() {
                    StatCard {
                        label: "NO ARTIFACTS",
                        value: "0".to_string(),
                        detail: "run write/check or trigger a denied command".to_string(),
                    }
                } else {
                    div {
                        style: "width: 260px; flex-shrink: 0; display:flex; flex-direction:column; gap:8px; overflow:auto;",
                        for (index, artifact) in artifacts.iter().enumerate() {
                            button {
                                style: format!(
                                    "text-align:left;padding:10px 12px;border:1px solid {};background:{};cursor:pointer;display:flex;flex-direction:column;gap:4px;",
                                    if index == active_index { artifact_kind_color(&artifact.kind) } else { BORDER },
                                    if index == active_index { "#121a24" } else { BG_PANEL },
                                ),
                                onclick: move |_| selected_index.set(index),
                                span {
                                    style: format!("color:{};font-size:10px;letter-spacing:0.08em;", artifact_kind_color(&artifact.kind)),
                                    "{artifact_kind_label(&artifact.kind)}"
                                }
                                span {
                                    style: "color:#c9d1d9;font-size:12px;",
                                    "{artifact.name}"
                                }
                                span {
                                    style: "color:{TEXT_DIM};font-size:11px;line-height:1.4;",
                                    "{artifact.summary}"
                                }
                            }
                        }
                    }
                    div {
                        style: "flex:1;min-width:0;display:flex;flex-direction:column;gap:6px;overflow:hidden;",
                        ArtifactViewer { artifact: artifacts[active_index].clone() }
                    }
                }
            }
        }
    }
}
