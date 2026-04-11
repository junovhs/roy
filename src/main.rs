mod app;
mod agents;
mod capabilities;
mod commands;
mod diagnostics;
mod policy;
mod session;
mod shell;
mod storage;
mod ui;
mod workspace;

use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("ROY — controlled shell host")
                        .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0, 800.0))
                        .with_min_inner_size(dioxus::desktop::LogicalSize::new(900.0, 600.0)),
                )
                .with_disable_context_menu(true),
        )
        .launch(app::App);
}
