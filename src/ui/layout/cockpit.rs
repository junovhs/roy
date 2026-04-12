use dioxus::prelude::*;

use crate::shell::ShellRuntime;

use super::activity_drawer::ActivityDrawer;
use super::attention_drawer::AttentionDrawer;
use super::chrome::Header;
use super::diag_drawer::DiagDrawer;
use super::panels::ShellPane;
use super::resize::WindowResizeZones;
use super::review_drawer::ReviewDrawer;
use super::{
    build_cockpit_session, drawer_selected, now_millis, CORAL, INK, INK_FAINT, LINE, SURFACE_2,
};

#[component]
pub fn Cockpit() -> Element {
    let open_drawer: Signal<Option<&'static str>> = use_signal(|| None);

    let workspace_root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let runtime_root = workspace_root.clone();
    let runtime = use_signal(move || ShellRuntime::new(runtime_root.clone()));
    let session_root = workspace_root.clone();
    let session = use_signal(move || build_cockpit_session(session_root.clone(), now_millis()));

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                width: 100vw;
                position: relative;
                background: linear-gradient(180deg,#1e1f23 0%,#16171a 100%);
                overflow: hidden;
                -webkit-font-smoothing: antialiased;
                font-family: 'Geist', 'Inter', -apple-system, sans-serif;
                font-size: 15px;
                color: {INK};
                box-sizing: border-box;
            ",

            WindowResizeZones {}

            div {
                style: "position: relative; flex-shrink: 0;",
                onmousedown: move |_| { dioxus::desktop::window().drag(); },
                div {
                    style: "
                        position: absolute;
                        top: 50%;
                        left: 16px;
                        transform: translateY(-50%);
                        display: flex;
                        gap: 7px;
                        z-index: 6;
                    ",
                    button {
                        style: "width:11px;height:11px;border-radius:50%;background:#ff5f57;border:none;padding:0;cursor:pointer;",
                        title: "Close",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| { dioxus::desktop::window().close(); },
                    }
                    button {
                        style: "width:11px;height:11px;border-radius:50%;background:#febc2e;border:none;padding:0;cursor:pointer;",
                        title: "Minimize",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| { dioxus::desktop::window().window.set_minimized(true); },
                    }
                    button {
                        style: "width:11px;height:11px;border-radius:50%;background:#28c840;border:none;padding:0;cursor:pointer;",
                        title: "Maximize",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| { dioxus::desktop::window().toggle_maximized(); },
                    }
                }

                Header { runtime, session }
            }

            div {
                style: "
                    flex: 1;
                    display: flex;
                    position: relative;
                    min-height: 0;
                    padding: 20px 28px 20px;
                ",

                div {
                    style: "flex: 1; display: flex; min-height: 0; position: relative;",
                    div {
                        style: "
                            flex: 1;
                            position: relative;
                            display: flex;
                            flex-direction: column;
                            min-width: 0;
                        ",
                        ShellPane { runtime, session }
                        div {
                            style: "
                                position: absolute;
                                right: 14px;
                                top: 50%;
                                transform: translateY(-50%);
                                display: flex;
                                flex-direction: column;
                                gap: 6px;
                                z-index: 3;
                            ",
                            EdgeBtn { label: "!", drawer: "attention", open_drawer }
                            EdgeBtn { label: "R", drawer: "review",    open_drawer }
                            EdgeBtn { label: "A", drawer: "activity",  open_drawer }
                            EdgeBtn { label: "D", drawer: "diag",      open_drawer }
                        }
                    }
                }

                ActivityDrawer  { open_drawer, session }
                DiagDrawer      { open_drawer, runtime, session }
                AttentionDrawer { open_drawer, session }
                ReviewDrawer    { open_drawer, session }
            }
        }
    }
}

#[component]
fn EdgeBtn(
    label: &'static str,
    drawer: &'static str,
    open_drawer: Signal<Option<&'static str>>,
) -> Element {
    let is_active = drawer_selected(open_drawer.read().as_deref(), drawer);
    let color = if is_active { CORAL } else { INK_FAINT };
    let bg = if is_active {
        "rgba(232,120,88,.06)"
    } else {
        SURFACE_2
    };
    let border = if is_active {
        "rgba(232,120,88,.35)"
    } else {
        LINE
    };

    rsx! {
        button {
            style: "
                width: 30px;
                height: 30px;
                border-radius: 8px;
                background: {bg};
                border: 1px solid {border};
                color: {color};
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;
                font-family: 'Geist', sans-serif;
                font-size: 14px;
                font-weight: 500;
                transition: all .2s;
            ",
            onclick: move |_| {
                let cur = open_drawer.read().as_deref() == Some(drawer);
                open_drawer.set(if cur { None } else { Some(drawer) });
            },
            "{label}"
        }
    }
}
