mod agents;
mod app;
mod capabilities;
mod commands;
mod diagnostics;
mod nouns;
mod policy;
mod render;
mod schema_registry;
mod session;
mod shell;
mod storage;
mod ui;
mod workspace;

use tracing_subscriber::EnvFilter;

fn main() {
    let window_title = format!("ROY v{} - controlled shell host", env!("CARGO_PKG_VERSION"));

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title(window_title)
                        .with_decorations(false)
                        .with_inner_size(dioxus::desktop::LogicalSize::new(900.0, 1200.0))
                        .with_min_inner_size(dioxus::desktop::LogicalSize::new(700.0, 600.0)),
                )
                .with_disable_context_menu(true),
        )
        .launch(app::App);
}
