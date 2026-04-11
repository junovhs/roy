use dioxus::prelude::*;

use crate::session::{ArtifactBody, ArtifactKind, SessionArtifact};

const TEXT_PRIMARY: &str = "#c9d1d9";
const TEXT_DIM: &str = "#6e7681";
const BORDER: &str = "#30363d";
const BG_SHELL: &str = "#010409";

pub(crate) fn artifact_kind_label(kind: &ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Diff => "DIFF",
        ArtifactKind::ValidationRun => "VALIDATION",
        ArtifactKind::DeniedCommandTrace => "DENIED",
        ArtifactKind::IssueDraft => "ISSUE",
        ArtifactKind::Note => "NOTE",
    }
}

pub(crate) fn artifact_kind_color(kind: &ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Diff => "#4ac26b",
        ArtifactKind::ValidationRun => "#e8944a",
        ArtifactKind::DeniedCommandTrace => "#f85149",
        ArtifactKind::IssueDraft => "#58a6ff",
        ArtifactKind::Note => TEXT_DIM,
    }
}

#[component]
pub(crate) fn ArtifactViewer(artifact: SessionArtifact) -> Element {
    match artifact.body.clone() {
        ArtifactBody::Diff {
            path,
            previous,
            current,
            diff,
        } => rsx! {
            div {
                style: "display:flex;flex-direction:column;gap:8px;min-width:0;",
                div {
                    style: "display:flex;justify-content:space-between;gap:12px;flex-wrap:wrap;",
                    span { style: "color:{TEXT_PRIMARY};font-size:12px;", "{path.display()}" }
                    span { style: "color:{TEXT_DIM};font-size:11px;", if previous.is_some() { "updated file" } else { "new file" } }
                }
                if current.is_empty() {
                    div {
                        style: "color:{TEXT_DIM};font-size:11px;",
                        "current contents are empty"
                    }
                }
                pre {
                    style: "margin:0;padding:10px 12px;background:{BG_SHELL};border:1px solid {BORDER};overflow:auto;white-space:pre-wrap;font-size:11px;line-height:1.5;",
                    for line in diff.lines() {
                        span {
                            style: "display:block;color:{diff_line_color(line)};",
                            "{line}"
                        }
                    }
                }
            }
        },
        ArtifactBody::ValidationRun {
            command,
            cwd,
            exit_code,
            stdout,
            stderr,
        } => {
            let stdout_empty = stdout.trim().is_empty();
            let stderr_empty = stderr.trim().is_empty();

            rsx! {
                div {
                style: "display:flex;flex-direction:column;gap:8px;min-width:0;",
                div {
                    style: "display:flex;justify-content:space-between;gap:12px;flex-wrap:wrap;",
                    span { style: "color:{TEXT_PRIMARY};font-size:12px;", "{command}" }
                    span {
                        style: "color:{artifact_kind_color(&artifact.kind)};font-size:11px;",
                        if exit_code == 0 { "PASS" } else { "FAIL" }
                    }
                }
                div {
                    style: "color:{TEXT_DIM};font-size:11px;",
                    "{cwd.display()}"
                }
                if !stdout_empty {
                    ArtifactTextBlock {
                        title: "STDOUT".to_string(),
                        text: stdout.clone(),
                        color: TEXT_PRIMARY.to_string(),
                    }
                }
                if !stderr_empty {
                    ArtifactTextBlock {
                        title: "STDERR".to_string(),
                        text: stderr.clone(),
                        color: "#f85149".to_string(),
                    }
                }
                if stdout_empty && stderr_empty {
                    div {
                        style: "color:{TEXT_DIM};font-size:11px;",
                        "validation completed without transcript output"
                    }
                }
            }
            }
        },
        ArtifactBody::DeniedCommandTrace { command, args, reason } => {
            let rendered = if args.is_empty() {
                command.clone()
            } else {
                format!("{command} {}", args.join(" "))
            };
            rsx! {
                div {
                    style: "display:flex;flex-direction:column;gap:8px;min-width:0;",
                    div {
                        style: "color:{TEXT_PRIMARY};font-size:12px;",
                        "{rendered}"
                    }
                    ArtifactTextBlock {
                        title: "WHY ROY BLOCKED IT".to_string(),
                        text: reason,
                        color: "#f85149".to_string(),
                    }
                }
            }
        }
        ArtifactBody::IssueDraft { title, body } => rsx! {
            div {
                style: "display:flex;flex-direction:column;gap:8px;min-width:0;",
                div {
                    style: "color:{TEXT_PRIMARY};font-size:12px;font-weight:600;",
                    "{title}"
                }
                ArtifactTextBlock {
                    title: "DRAFT".to_string(),
                    text: body,
                    color: TEXT_PRIMARY.to_string(),
                }
            }
        },
        ArtifactBody::Note { text } => rsx! {
            ArtifactTextBlock {
                title: "NOTE".to_string(),
                text: text,
                color: TEXT_PRIMARY.to_string(),
            }
        },
    }
}

#[component]
fn ArtifactTextBlock(title: String, text: String, color: String) -> Element {
    rsx! {
        div {
            style: "display:flex;flex-direction:column;gap:4px;min-width:0;",
            div {
                style: "color:{TEXT_DIM};font-size:10px;letter-spacing:0.08em;",
                "{title}"
            }
            pre {
                style: "margin:0;padding:10px 12px;background:{BG_SHELL};border:1px solid {BORDER};overflow:auto;white-space:pre-wrap;font-size:11px;line-height:1.5;color:{color};",
                "{text}"
            }
        }
    }
}

fn diff_line_color(line: &str) -> &'static str {
    if line.starts_with("+++") || line.starts_with("---") || line.starts_with("@@") {
        "#58a6ff"
    } else if line.starts_with('+') {
        "#4ac26b"
    } else if line.starts_with('-') {
        "#f85149"
    } else {
        TEXT_PRIMARY
    }
}
