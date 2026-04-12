use dioxus::prelude::*;

use super::{drawer_selected, INK, INK_FAINT, LINE, LINE_2, SURFACE_2};

#[component]
pub(super) fn DrawerShell(
    name: &'static str,
    title: &'static str,
    subtitle: &'static str,
    open_drawer: Signal<Option<&'static str>>,
    children: Element,
) -> Element {
    let open = drawer_selected(open_drawer.read().as_deref(), name);
    let tx = if open {
        "translateX(0)"
    } else {
        "translateX(calc(100% + 60px))"
    };

    rsx! {
        div {
            style: "
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                width: 380px;
                background: {SURFACE_2};
                border: 1px solid {LINE_2};
                border-radius: 10px;
                transform: {tx};
                transition: transform .4s cubic-bezier(.32,.72,0,1);
                display: flex;
                flex-direction: column;
                z-index: 20;
                box-shadow: 0 30px 60px rgba(0,0,0,.4);
            ",

            div {
                style: "
                    padding: 20px 22px 14px;
                    border-bottom: 1px solid {LINE};
                    display: flex;
                    align-items: baseline;
                    justify-content: space-between;
                    flex-shrink: 0;
                ",
                div {
                    div { style: "font-size: 12px; color: {INK_FAINT}; margin-bottom: 2px;", "— {subtitle} —" }
                    div {
                        style: "
                            font-family: 'Fraunces', Georgia, serif;
                            font-style: italic;
                            font-weight: 300;
                            font-size: 24px;
                            color: {INK};
                        ",
                        "{title}"
                    }
                }
                button {
                    style: "background:none;border:none;color:{INK_FAINT};cursor:pointer;font-size:22px;line-height:1;padding:0 2px;",
                    onclick: move |_| open_drawer.set(None),
                    "×"
                }
            }

            div {
                style: "flex: 1; overflow-y: auto; padding: 16px 22px 20px;",
                {children}
            }
        }
    }
}
