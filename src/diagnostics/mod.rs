//! Developer diagnostics — trace computation and display data for the
//! developer pane. Not part of the embedded agent's world.
//!
//! [`pane::build_trace`] converts session events into [`pane::DiagEntry`]
//! items enriched with registry-level resolution detail. The Dioxus rendering
//! lives in `ui/layout/footer.rs` (`DiagnosticsPane`).

pub mod pane;

pub use pane::build_trace;
