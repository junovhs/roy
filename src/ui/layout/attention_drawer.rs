use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};

use super::drawer_shell::DrawerShell;
use super::{CORAL_SOFT, INK, INK_DIM, INK_FAINT, LINE};

#[component]
pub(super) fn AttentionDrawer(
    open_drawer: Signal<Option<&'static str>>,
    session: Signal<Session>,
) -> Element {
    let sess = session.read();
    let denied: Vec<(String, String)> = sess
        .events()
        .iter()
        .rev()
        .filter_map(|e| {
            if let SessionEvent::CommandDenied {
                command,
                suggestion,
                ..
            } = e
            {
                Some((command.clone(), suggestion.clone().unwrap_or_default()))
            } else {
                None
            }
        })
        .take(5)
        .collect();

    rsx! {
        DrawerShell { name: "attention", title: "Attention", subtitle: "Needs you", open_drawer,
            if denied.is_empty() {
                div {
                    style: "padding:20px 16px;border:1px solid {LINE};border-radius:9px;background:rgba(255,255,255,.015);",
                    div { style: "font-size:12px;color:{INK_FAINT};margin-bottom:6px;", "All clear" }
                    div { style: "font-family:'Fraunces',serif;font-style:italic;font-weight:300;font-size:19px;color:{INK};margin-bottom:7px;", "No blocked commands" }
                    div { style: "font-size:13.5px;color:{INK_DIM};line-height:1.55;", "Denied commands and approval requests will appear here." }
                }
            } else {
                for (cmd, suggestion) in denied {
                    div {
                        style: "padding:14px 16px;margin-bottom:10px;border:1px solid rgba(232,120,88,.25);border-radius:9px;background:rgba(232,120,88,.04);",
                        div { style: "font-size:12px;color:{CORAL_SOFT};margin-bottom:6px;", "Blocked · Policy" }
                        div { style: "font-family:'Fraunces',serif;font-style:italic;font-weight:300;font-size:19px;color:{INK};margin-bottom:7px;", "{cmd}" }
                        if !suggestion.is_empty() {
                            div { style: "font-size:13.5px;color:{INK_DIM};line-height:1.55;", "{suggestion}" }
                        }
                    }
                }
            }
        }
    }
}
