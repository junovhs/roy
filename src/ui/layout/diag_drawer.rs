use dioxus::prelude::*;

use crate::session::Session;
use crate::shell::ShellRuntime;

use super::drawer_shell::DrawerShell;
use super::{relative_scope_label, INK, INK_FAINT, LINE};

#[component]
pub(super) fn DiagDrawer(
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
