mod chrome;
mod panels;

use dioxus::prelude::*;

use crate::session::{Session, SessionEvent, Timestamp};
use crate::shell::ShellRuntime;
use chrome::Header;
use panels::ShellPane;

// ── palette ───────────────────────────────────────────────────────────────────

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

// ── helpers ───────────────────────────────────────────────────────────────────

pub(crate) fn now_millis() -> Timestamp {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as Timestamp
}

pub(crate) fn short_path_label(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

pub(crate) fn relative_scope_label(root: &std::path::Path, cwd: &std::path::Path) -> String {
    cwd.strip_prefix(root)
        .ok()
        .and_then(|path| {
            if path.as_os_str().is_empty() {
                None
            } else {
                Some(format!("/{}", path.display()))
            }
        })
        .unwrap_or_else(|| "/".to_string())
}

pub(crate) fn is_session_active(session: &Session) -> bool {
    !matches!(
        session.events().last(),
        Some(SessionEvent::SessionEnded { .. })
    )
}

fn build_cockpit_session(session_root: std::path::PathBuf, ts: Timestamp) -> Session {
    let mut s = Session::new(ts, session_root, ts);
    s.push(SessionEvent::HostNotice {
        message: "ROY shell cockpit ready".to_string(),
        ts: ts + 1,
    });
    s
}

fn drawer_selected(open_drawer: Option<&str>, drawer: &str) -> bool {
    open_drawer == Some(drawer)
}

// ── root cockpit ──────────────────────────────────────────────────────────────

/// Root shell cockpit.
///
/// Fills the OS window directly — no inner card, no outer padding.
/// Layout: traffic-light dots (absolute) + marquee header + main area
/// (pod-wrapper with ShellPane + edge buttons + slide-in drawers).
#[component]
pub fn Cockpit() -> Element {
    let open_drawer: Signal<Option<&'static str>> = use_signal(|| None);

    let workspace_root =
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    let runtime_root = workspace_root.clone();
    let runtime = use_signal(move || ShellRuntime::new(runtime_root.clone()));

    let session_root = workspace_root.clone();
    let session = use_signal(move || build_cockpit_session(session_root.clone(), now_millis()));

    rsx! {
        // Root: fills the OS window
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

            // ── titlebar: window controls + header (drag region) ─────────
            div {
                style: "position: relative; flex-shrink: 0;",
                onmousedown: move |_| { dioxus::desktop::window().drag(); },

                // traffic-light window controls
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
                    // Close — red
                    button {
                        style: "width:11px;height:11px;border-radius:50%;background:#ff5f57;border:none;padding:0;cursor:pointer;",
                        title: "Close",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| { dioxus::desktop::window().close(); },
                    }
                    // Minimize — yellow
                    button {
                        style: "width:11px;height:11px;border-radius:50%;background:#febc2e;border:none;padding:0;cursor:pointer;",
                        title: "Minimize",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| { dioxus::desktop::window().window.set_minimized(true); },
                    }
                    // Maximize / restore — green
                    button {
                        style: "width:11px;height:11px;border-radius:50%;background:#28c840;border:none;padding:0;cursor:pointer;",
                        title: "Maximize",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| { dioxus::desktop::window().toggle_maximized(); },
                    }
                }

                // marquee header (draggable via parent onmousedown)
                Header { runtime, session }
            }

            // ── main area ─────────────────────────────────────────────────
            div {
                style: "
                    flex: 1;
                    display: flex;
                    position: relative;
                    min-height: 0;
                    padding: 20px 28px 20px;
                ",

                // ── canvas ────────────────────────────────────────────────
                div {
                    style: "flex: 1; display: flex; min-height: 0; position: relative;",

                    // ── pod-wrapper ───────────────────────────────────────
                    div {
                        style: "
                            flex: 1;
                            position: relative;
                            display: flex;
                            flex-direction: column;
                            min-width: 0;
                        ",

                        ShellPane { runtime, session }

                        // ── edge buttons ──────────────────────────────────
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

                // ── slide-in drawers (absolute within main) ───────────────
                ActivityDrawer  { open_drawer, session }
                DiagDrawer      { open_drawer, runtime, session }
                AttentionDrawer { open_drawer, session }
                ReviewDrawer    { open_drawer, session }
            }
        }
    }
}

// ── edge button ───────────────────────────────────────────────────────────────

#[component]
fn EdgeBtn(
    label: &'static str,
    drawer: &'static str,
    open_drawer: Signal<Option<&'static str>>,
) -> Element {
    let is_active = drawer_selected(open_drawer.read().as_deref(), drawer);
    let color = if is_active { CORAL } else { INK_FAINT };
    let bg = if is_active { "rgba(232,120,88,.06)" } else { SURFACE_2 };
    let border = if is_active { "rgba(232,120,88,.35)" } else { LINE };

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

// ── drawer shell ──────────────────────────────────────────────────────────────

/// Sliding overlay panel. Always rendered; CSS transform hides/shows it.
/// The root window's overflow:hidden clips the offscreen state.
#[component]
fn DrawerShell(
    name: &'static str,
    title: &'static str,
    subtitle: &'static str,
    open_drawer: Signal<Option<&'static str>>,
    children: Element,
) -> Element {
    let open = drawer_selected(open_drawer.read().as_deref(), name);
    let tx = if open { "translateX(0)" } else { "translateX(calc(100% + 60px))" };

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

            // drawer header
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

            // drawer body
            div {
                style: "flex: 1; overflow-y: auto; padding: 16px 22px 20px;",
                {children}
            }
        }
    }
}

// ── activity drawer ───────────────────────────────────────────────────────────

#[component]
fn ActivityDrawer(
    open_drawer: Signal<Option<&'static str>>,
    session: Signal<Session>,
) -> Element {
    let session = session.read();
    let events: Vec<(String, String, &'static str)> = session
        .events()
        .iter()
        .rev()
        .filter_map(event_row)
        .take(20)
        .collect();

    rsx! {
        DrawerShell { name: "activity", title: "Activity", subtitle: "Session", open_drawer,
            if events.is_empty() {
                div { style: "color: {INK_FAINT}; font-size: 13px;", "No events yet." }
            } else {
                for (tag, msg, color) in events {
                    div {
                        style: "display:flex;gap:14px;padding:10px 0;border-bottom:1px solid {LINE};",
                        span {
                            style: "font-family:'JetBrains Mono',monospace;color:{INK_FAINT};font-size:12px;min-width:48px;padding-top:1px;",
                            "{tag}"
                        }
                        span {
                            style: "font-size:14px;color:{color};flex:1;line-height:1.45;",
                            "{msg}"
                        }
                    }
                }
            }
        }
    }
}

fn event_row(event: &SessionEvent) -> Option<(String, String, &'static str)> {
    match event {
        SessionEvent::SessionStarted { .. } => {
            Some(("SESSION".to_string(), "shell session opened".to_string(), MINT))
        }
        SessionEvent::SessionEnded { exit_code, .. } => Some((
            "SESSION".to_string(),
            format!("ended · exit {exit_code}"),
            INK_FAINT,
        )),
        SessionEvent::UserInput { text, .. } => {
            Some(("INPUT".to_string(), text.clone(), CORAL_SOFT))
        }
        SessionEvent::CommandInvoked { command, args, .. } => Some((
            "CMD".to_string(),
            if args.is_empty() {
                command.clone()
            } else {
                format!("{command} {}", args.join(" "))
            },
            INK_DIM,
        )),
        SessionEvent::CommandOutput { text, is_error, .. } => {
            if text.trim().is_empty() {
                return None;
            }
            Some((
                if *is_error { "STDERR" } else { "STDOUT" }.to_string(),
                text.clone(),
                if *is_error { "#f85149" } else { INK },
            ))
        }
        SessionEvent::CommandDenied { command, .. } => {
            Some(("DENIED".to_string(), format!("{command} blocked"), "#f85149"))
        }
        SessionEvent::CommandNotFound { command, .. } => Some((
            "MISSING".to_string(),
            format!("{command} not in ROY world"),
            "#f85149",
        )),
        SessionEvent::CwdChanged { to, .. } => {
            Some(("CWD".to_string(), to.display().to_string(), INK_DIM))
        }
        SessionEvent::HostNotice { message, .. } => {
            Some(("HOST".to_string(), message.clone(), INK_DIM))
        }
        SessionEvent::ArtifactCreated { artifact, .. } => Some((
            "ARTIFACT".to_string(),
            format!("{} · {}", artifact.name, artifact.summary),
            MINT,
        )),
        SessionEvent::AgentOutput { text, .. } => {
            Some(("AGENT".to_string(), text.clone(), INK))
        }
    }
}

// ── diagnostics drawer ────────────────────────────────────────────────────────

#[component]
fn DiagDrawer(
    open_drawer: Signal<Option<&'static str>>,
    runtime: Signal<ShellRuntime>,
    session: Signal<Session>,
) -> Element {
    let rt = runtime.read();
    let sess = session.read();

    let root = rt.workspace_root().display().to_string();
    let cwd = rt.env().cwd().display().to_string();
    let scope = relative_scope_label(rt.workspace_root(), rt.env().cwd());
    let policy = rt.policy_name().to_string();
    let sess_line = format!("#{} · {} events", sess.id, sess.len());
    let artifact_count = sess.artifacts().len();
    let cmd_count = rt.public_command_count();

    rsx! {
        DrawerShell { name: "diag", title: "Diagnostics", subtitle: "Internals", open_drawer,
            DiagRow { label: "root",      value: root }
            DiagRow { label: "cwd",       value: cwd }
            DiagRow { label: "scope",     value: scope }
            DiagRow { label: "policy",    value: policy }
            DiagRow { label: "session",   value: sess_line }
            DiagRow { label: "artifacts", value: artifact_count.to_string() }
            DiagRow { label: "commands",  value: format!("{cmd_count} public") }
            DiagRow { label: "runtime",   value: format!("ROY v{}", env!("CARGO_PKG_VERSION")) }
        }
    }
}

#[component]
fn DiagRow(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                padding: 7px 0;
                font-family: 'JetBrains Mono', monospace;
                font-size: 12.5px;
                border-bottom: 1px solid {LINE};
            ",
            span { style: "color: {INK_FAINT};", "{label}" }
            span {
                style: "color:{INK};text-align:right;max-width:220px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;",
                "{value}"
            }
        }
    }
}

// ── attention drawer ──────────────────────────────────────────────────────────

#[component]
fn AttentionDrawer(
    open_drawer: Signal<Option<&'static str>>,
    session: Signal<Session>,
) -> Element {
    let sess = session.read();
    let denied: Vec<(String, String)> = sess
        .events()
        .iter()
        .rev()
        .filter_map(|e| {
            if let SessionEvent::CommandDenied { command, suggestion, .. } = e {
                Some((
                    command.clone(),
                    suggestion.clone().unwrap_or_default(),
                ))
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

// ── window resize zones ───────────────────────────────────────────────────────

/// Invisible fixed-position hit zones that enable frameless window resizing.
/// Each zone sets an appropriate CSS cursor and calls tao's drag_resize_window.
#[component]
fn WindowResizeZones() -> Element {
    use dioxus::desktop::tao::window::ResizeDirection;

    macro_rules! zone {
        ($style:expr, $dir:expr) => {{
            let dir = $dir;
            rsx! {
                div {
                    style: $style,
                    onmousedown: move |e| {
                        e.stop_propagation();
                        let _ = dioxus::desktop::window().window.drag_resize_window(dir);
                    },
                }
            }
        }};
    }

    rsx! {
        // edges
        {zone!("position:fixed;top:0;left:16px;right:16px;height:10px;cursor:n-resize;z-index:9999;",  ResizeDirection::North)}
        {zone!("position:fixed;bottom:0;left:16px;right:16px;height:10px;cursor:s-resize;z-index:9999;", ResizeDirection::South)}
        {zone!("position:fixed;left:0;top:16px;bottom:16px;width:10px;cursor:w-resize;z-index:9999;",  ResizeDirection::West)}
        {zone!("position:fixed;right:0;top:16px;bottom:16px;width:10px;cursor:e-resize;z-index:9999;",  ResizeDirection::East)}
        // corners
        {zone!("position:fixed;top:0;left:0;width:16px;height:16px;cursor:nw-resize;z-index:10000;",  ResizeDirection::NorthWest)}
        {zone!("position:fixed;top:0;right:0;width:16px;height:16px;cursor:ne-resize;z-index:10000;", ResizeDirection::NorthEast)}
        {zone!("position:fixed;bottom:0;left:0;width:16px;height:16px;cursor:sw-resize;z-index:10000;", ResizeDirection::SouthWest)}
        {zone!("position:fixed;bottom:0;right:0;width:16px;height:16px;cursor:se-resize;z-index:10000;", ResizeDirection::SouthEast)}
    }
}

// ── review drawer ─────────────────────────────────────────────────────────────

#[component]
fn ReviewDrawer(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{ArtifactBody, ArtifactKind, SessionArtifact};
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn session() -> Session {
        Session::new(7, PathBuf::from("/tmp/roy-layout-tests"), 10)
    }

    #[test]
    fn now_millis_returns_current_epoch_millis() {
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as Timestamp;
        let ts = now_millis();
        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as Timestamp;

        assert!(ts >= before, "timestamp must not move backwards");
        assert!(ts <= after, "timestamp must not exceed the current clock");
        assert!(ts > 1_000_000_000_000, "timestamp must be in modern epoch millis");
    }

    #[test]
    fn short_path_label_prefers_file_name() {
        assert_eq!(short_path_label(Path::new("/tmp/demo/file.txt")), "file.txt");
    }

    #[test]
    fn short_path_label_falls_back_to_display_for_root_like_paths() {
        assert_eq!(short_path_label(Path::new("/")), "/");
    }

    #[test]
    fn relative_scope_label_is_slash_for_workspace_root() {
        assert_eq!(
            relative_scope_label(Path::new("/tmp/project"), Path::new("/tmp/project")),
            "/"
        );
    }

    #[test]
    fn relative_scope_label_formats_child_path() {
        assert_eq!(
            relative_scope_label(
                Path::new("/tmp/project"),
                Path::new("/tmp/project/src/ui/layout")
            ),
            "/src/ui/layout"
        );
    }

    #[test]
    fn relative_scope_label_denies_outside_paths_with_root_marker() {
        assert_eq!(
            relative_scope_label(Path::new("/tmp/project"), Path::new("/outside")),
            "/"
        );
    }

    #[test]
    fn is_session_active_only_until_end_event() {
        let mut session = session();
        assert!(is_session_active(&session));

        session.end(9, 20);
        assert!(!is_session_active(&session));
    }

    #[test]
    fn build_cockpit_session_adds_host_notice_after_start_event() {
        let session = build_cockpit_session(PathBuf::from("/tmp/cockpit"), 42);
        let events = session.events();

        assert!(matches!(events[0], SessionEvent::SessionStarted { ts: 42 }));
        assert!(matches!(
            &events[1],
            SessionEvent::HostNotice { message, ts }
            if message == "ROY shell cockpit ready" && *ts == 43
        ));
    }

    #[test]
    fn drawer_selected_matches_only_the_active_drawer() {
        assert!(drawer_selected(Some("diag"), "diag"));
        assert!(!drawer_selected(Some("diag"), "review"));
        assert!(!drawer_selected(None, "diag"));
    }

    #[test]
    fn event_row_formats_invoked_command_with_args() {
        let row = event_row(&SessionEvent::CommandInvoked {
            command: "read".to_string(),
            args: vec!["Cargo.toml".to_string(), "--json".to_string()],
            ts: 12,
        });

        assert_eq!(
            row,
            Some((
                "CMD".to_string(),
                "read Cargo.toml --json".to_string(),
                INK_DIM,
            ))
        );
    }

    #[test]
    fn event_row_discards_blank_output_lines() {
        let row = event_row(&SessionEvent::CommandOutput {
            text: "   ".to_string(),
            is_error: false,
            ts: 12,
        });

        assert_eq!(row, None);
    }

    #[test]
    fn event_row_formats_artifacts_with_name_and_summary() {
        let row = event_row(&SessionEvent::ArtifactCreated {
            artifact: SessionArtifact {
                name: "neti-report.txt".to_string(),
                kind: ArtifactKind::Note,
                summary: "validation output".to_string(),
                body: ArtifactBody::Note {
                    text: "details".to_string(),
                },
            },
            ts: 13,
        });

        assert_eq!(
            row,
            Some((
                "ARTIFACT".to_string(),
                "neti-report.txt · validation output".to_string(),
                MINT,
            ))
        );
    }

    #[test]
    fn event_row_marks_missing_commands_as_errors() {
        let row = event_row(&SessionEvent::CommandNotFound {
            command: "unknown".to_string(),
            ts: 21,
        });

        assert_eq!(
            row,
            Some((
                "MISSING".to_string(),
                "unknown not in ROY world".to_string(),
                "#f85149",
            ))
        );
    }
}
