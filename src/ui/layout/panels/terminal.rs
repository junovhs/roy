use dioxus::prelude::*;

use crate::session::Session;
use crate::shell::ShellRuntime;

#[path = "terminal_composer.rs"]
mod terminal_composer;
#[path = "terminal_emulator.rs"]
mod terminal_emulator;
#[path = "terminal_line.rs"]
mod terminal_line;
#[path = "terminal_submit.rs"]
pub(super) mod terminal_submit;
#[path = "terminal_surface.rs"]
mod terminal_surface;
#[path = "terminal_view.rs"]
mod terminal_view;

pub(super) const INK: &str = "#e6e4df";
pub(super) const INK_DIM: &str = "#9b9892";
pub(super) const INK_FAINT: &str = "#5f5d58";
pub(super) const CORAL: &str = "#e87858";
pub(super) const CORAL_SOFT: &str = "#f09a7e";

pub(crate) struct SubmitContext {
    pub(crate) runtime: Signal<ShellRuntime>,
    pub(crate) session: Signal<Session>,
    pub(crate) lines: Signal<Vec<super::terminal_model::ShellLine>>,
    pub(crate) input_text: Signal<String>,
}

pub(crate) use terminal_submit::handle_submit;
pub(crate) use terminal_view::ShellPane;
