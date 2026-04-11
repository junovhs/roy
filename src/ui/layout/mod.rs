mod artifacts_row;
mod atoms;
mod chrome;
mod footer;
mod panels;

use dioxus::prelude::*;

use crate::session::{Session, SessionEvent, Timestamp};
use crate::shell::ShellRuntime;
use artifacts_row::ArtifactsRow;
use chrome::Header;
use footer::DiagnosticsPane;
use panels::{ActivityPanel, ShellPane, WorkspacePanel};

// ── palette ──────────────────────────────────────────────────────────────────

pub(crate) const BG_BASE: &str = "#0d0f12";
pub(crate) const BG_PANEL: &str = "#161b22";
pub(crate) const BG_SHELL: &str = "#010409";
pub(crate) const BG_HEADER: &str = "#0d1117";
pub(crate) const BORDER: &str = "#30363d";
pub(crate) const TEXT_PRIMARY: &str = "#c9d1d9";
pub(crate) const TEXT_DIM: &str = "#6e7681";
pub(crate) const TEXT_ACCENT: &str = "#e8944a";
pub(crate) const TEXT_YELLOW: &str = "#d29922";

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

// ── root cockpit ─────────────────────────────────────────────────────────────

/// Root shell cockpit. Owns the [`ShellRuntime`] session and wraps the
/// workspace header, three-column body (workspace context | shell pane |
/// activity/approvals), artifacts row, and collapsible diagnostics pane.
#[component]
pub fn Cockpit() -> Element {
    let diag_open = use_signal(|| false);
    let workspace_root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    let runtime_root = workspace_root.clone();
    let runtime = use_signal(move || ShellRuntime::new(runtime_root.clone()));

    let session_root = workspace_root.clone();
    let session = use_signal(move || {
        let ts = now_millis();
        let mut session = Session::new(ts, session_root.clone(), ts);
        session.push(SessionEvent::HostNotice {
            message: "ROY shell cockpit ready".to_string(),
            ts: ts + 1,
        });
        session
    });

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                width: 100vw;
                background: {BG_BASE};
                color: {TEXT_PRIMARY};
                font-family: 'JetBrains Mono', 'Cascadia Code', Consolas, monospace;
                font-size: 13px;
                line-height: 1.5;
                overflow: hidden;
                box-sizing: border-box;
            ",

            Header { runtime, session }

            div {
                style: "display: flex; flex: 1; overflow: hidden; border-top: 1px solid {BORDER};",

                WorkspacePanel { runtime, session }
                ShellPane { runtime, session }
                ActivityPanel { session }
            }

            ArtifactsRow { session }
            DiagnosticsPane { open: diag_open, runtime, session }
        }
    }
}
