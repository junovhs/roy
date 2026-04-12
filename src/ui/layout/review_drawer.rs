use dioxus::prelude::*;

use crate::session::Session;

use super::drawer_shell::DrawerShell;
use super::{INK, INK_DIM, INK_FAINT, LINE, MINT};

#[component]
pub(super) fn ReviewDrawer(
    open_drawer: Signal<Option<&'static str>>,
    session: Signal<Session>,
) -> Element {
    let sess = session.read();
    let artifacts: Vec<_> = sess.artifacts().into_iter().rev().cloned().collect();

    rsx! {
        DrawerShell { name: "review", title: "Review", subtitle: "Outcome", open_drawer,
            if artifacts.is_empty() {
                div {
                    style: "padding:20px 16px;border:1px solid {LINE};border-radius:9px;background:rgba(255,255,255,.015);",
                    div { style: "font-size:12px;color:{INK_FAINT};margin-bottom:6px;", "Pending" }
                    div { style: "font-family:'Fraunces',serif;font-style:italic;font-weight:300;font-size:19px;color:{INK};margin-bottom:7px;", "No artifacts yet" }
                    div { style: "font-size:13.5px;color:{INK_DIM};line-height:1.55;", "Run write, check, or trigger a denied command to promote artifacts here." }
                }
            } else {
                for artifact in &artifacts {
                    div {
                        style: "padding:14px 16px;margin-bottom:10px;border:1px solid {LINE};border-radius:9px;background:rgba(255,255,255,.015);",
                        div { style: "font-size:12px;color:{MINT};margin-bottom:6px;", "✓ {artifact.kind.as_str()}" }
                        div { style: "font-family:'Fraunces',serif;font-style:italic;font-weight:300;font-size:19px;color:{INK};margin-bottom:7px;", "{artifact.name}" }
                        div { style: "font-size:13.5px;color:{INK_DIM};line-height:1.55;", "{artifact.summary}" }
                    }
                }
            }
        }
    }
}
