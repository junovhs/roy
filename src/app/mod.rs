use dioxus::prelude::*;

use crate::ui::layout::Cockpit;

/// Root application component. Bootstraps the ROY shell cockpit.
#[component]
pub fn App() -> Element {
    rsx! {
        Cockpit {}
    }
}
