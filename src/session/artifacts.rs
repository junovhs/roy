use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// High-value output classes that ROY promotes above transcript text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactKind {
    Diff,
    ValidationRun,
    DeniedCommandTrace,
    IssueDraft,
    Note,
}

impl ArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Diff => "diff",
            Self::ValidationRun => "validation",
            Self::DeniedCommandTrace => "denied_trace",
            Self::IssueDraft => "issue_draft",
            Self::Note => "note",
        }
    }
}

/// Typed artifact payload rendered by dedicated ROY viewers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactBody {
    Diff {
        path: PathBuf,
        previous: Option<String>,
        current: String,
        diff: String,
    },
    ValidationRun {
        command: String,
        cwd: PathBuf,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
    DeniedCommandTrace {
        command: String,
        args: Vec<String>,
        reason: String,
    },
    IssueDraft {
        title: String,
        body: String,
    },
    Note {
        text: String,
    },
}

/// Significant structured output attached to a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionArtifact {
    pub name: String,
    pub kind: ArtifactKind,
    pub summary: String,
    pub body: ArtifactBody,
}

impl SessionArtifact {
    pub fn kind_str(&self) -> &'static str {
        self.kind.as_str()
    }

    pub fn diff(path: PathBuf, previous: Option<String>, current: String) -> Option<Self> {
        if previous.as_deref() == Some(current.as_str()) {
            return None;
        }

        let display_name = short_name(&path);
        let summary = if previous.is_some() {
            format!("updated {}", path.display())
        } else {
            format!("created {}", path.display())
        };
        let diff = build_unified_diff(&path, previous.as_deref(), &current);

        Some(Self {
            name: display_name,
            kind: ArtifactKind::Diff,
            summary,
            body: ArtifactBody::Diff {
                path,
                previous,
                current,
                diff,
            },
        })
    }

    pub fn validation_run(
        command: String,
        cwd: PathBuf,
        exit_code: i32,
        stdout: String,
        stderr: String,
    ) -> Self {
        let status = if exit_code == 0 { "passed" } else { "failed" };
        Self {
            name: command.clone(),
            kind: ArtifactKind::ValidationRun,
            summary: format!("{command} {status} in {}", cwd.display()),
            body: ArtifactBody::ValidationRun {
                command,
                cwd,
                exit_code,
                stdout,
                stderr,
            },
        }
    }

    pub fn denied_command(command: &str, args: &[&str], reason: String) -> Self {
        let rendered = if args.is_empty() {
            command.to_string()
        } else {
            format!("{command} {}", args.join(" "))
        };
        Self {
            name: rendered.clone(),
            kind: ArtifactKind::DeniedCommandTrace,
            summary: format!("blocked: {rendered}"),
            body: ArtifactBody::DeniedCommandTrace {
                command: command.to_string(),
                args: args.iter().map(|value| (*value).to_string()).collect(),
                reason,
            },
        }
    }
}

fn short_name(path: &Path) -> String {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

#[derive(Clone, Copy)]
enum DiffOp<'a> {
    Keep(&'a str),
    Add(&'a str),
    Remove(&'a str),
}

fn build_unified_diff(path: &Path, previous: Option<&str>, current: &str) -> String {
    let old_label = previous
        .map(|_| format!("a/{}", path.display()))
        .unwrap_or_else(|| "/dev/null".to_string());
    let new_label = format!("b/{}", path.display());
    let previous_lines: Vec<&str> = previous.unwrap_or_default().lines().collect();
    let current_lines: Vec<&str> = current.lines().collect();
    let ops = diff_ops(&previous_lines, &current_lines);

    let mut rendered = vec![
        format!("--- {old_label}"),
        format!("+++ {new_label}"),
        "@@".to_string(),
    ];
    let mut changed = false;

    for op in ops {
        match op {
            DiffOp::Keep(line) => rendered.push(format!(" {line}")),
            DiffOp::Add(line) => {
                changed = true;
                rendered.push(format!("+{line}"));
            }
            DiffOp::Remove(line) => {
                changed = true;
                rendered.push(format!("-{line}"));
            }
        }
    }

    if !changed {
        rendered.push(" (no textual changes)".to_string());
    }

    rendered.join("\n")
}

fn diff_ops<'a>(previous: &'a [&'a str], current: &'a [&'a str]) -> Vec<DiffOp<'a>> {
    let mut lcs = vec![vec![0usize; current.len() + 1]; previous.len() + 1];

    for left in (0..previous.len()).rev() {
        for right in (0..current.len()).rev() {
            lcs[left][right] = if previous[left] == current[right] {
                lcs[left + 1][right + 1] + 1
            } else {
                lcs[left + 1][right].max(lcs[left][right + 1])
            };
        }
    }

    let mut left = 0;
    let mut right = 0;
    let mut ops = Vec::new();

    while left < previous.len() && right < current.len() {
        if previous[left] == current[right] {
            ops.push(DiffOp::Keep(previous[left]));
            left += 1;
            right += 1;
        } else if lcs[left + 1][right] >= lcs[left][right + 1] {
            ops.push(DiffOp::Remove(previous[left]));
            left += 1;
        } else {
            ops.push(DiffOp::Add(current[right]));
            right += 1;
        }
    }

    while left < previous.len() {
        ops.push(DiffOp::Remove(previous[left]));
        left += 1;
    }

    while right < current.len() {
        ops.push(DiffOp::Add(current[right]));
        right += 1;
    }

    ops
}

#[cfg(test)]
#[path = "artifacts_tests.rs"]
mod tests;
