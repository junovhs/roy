mod activity_drawer;
mod attention_drawer;
mod chrome;
mod cockpit;
mod diag_drawer;
mod drawer_shell;
mod helpers;
mod panels;
mod resize;
mod review_drawer;

pub(crate) const INK: &str = "#e6e4df";
pub(crate) const INK_DIM: &str = "#9b9892";
pub(crate) const INK_FAINT: &str = "#5f5d58";
pub(crate) const LINE: &str = "rgba(255,255,255,.06)";
pub(crate) const LINE_2: &str = "rgba(255,255,255,.1)";
pub(crate) const MINT: &str = "#a8c5b4";
pub(crate) const CORAL: &str = "#e87858";
pub(crate) const CORAL_SOFT: &str = "#f09a7e";
pub(crate) const PEACH: &str = "#e8b494";
pub(crate) const SURFACE_2: &str = "#1c1d21";

pub use cockpit::Cockpit;
use helpers::{build_cockpit_session, drawer_selected};
pub(crate) use helpers::{is_session_active, now_millis, relative_scope_label, short_path_label};
