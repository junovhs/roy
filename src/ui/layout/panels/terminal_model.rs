pub(super) const TEXT_ERROR: &str = "#f85149";

#[cfg(test)]
pub(super) use super::terminal::terminal_submit::record_session_outcome;
#[cfg(test)]
pub(super) use super::terminal::{handle_submit, SubmitContext};

// ── line kind ─────────────────────────────────────────────────────────────────

/// Visual classification of a terminal line, driving rendering in terminal.rs.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum LineKind {
    /// Normal command output.
    Output,
    /// Raw stderr / generic error.
    Error,
    /// Input echo (prompt + command).
    Echo,
    /// First line of a denial block: "⊘ command — blocked".
    DenialHeader,
    /// Continuation of a denial: the ROY-world suggestion.
    DenialHint,
    /// Command not in the ROY registry.
    NotFound,
}

// ── shell line ────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub(super) struct ShellLine {
    pub(super) prefix: String,
    pub(super) text: String,
    pub(super) kind: LineKind,
}

impl ShellLine {
    pub(super) fn output(text: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            text: text.into(),
            kind: LineKind::Output,
        }
    }

    pub(super) fn error(text: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            text: text.into(),
            kind: LineKind::Error,
        }
    }

    pub(super) fn echo(prompt: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            prefix: prompt.into(),
            text: text.into(),
            kind: LineKind::Echo,
        }
    }

    /// Denial block header — "⊘ {command} — blocked by policy".
    pub(super) fn denial_header(command: impl Into<String>) -> Self {
        Self {
            prefix: "⊘".into(),
            text: format!("{} — blocked by policy", command.into()),
            kind: LineKind::DenialHeader,
        }
    }

    /// Denial suggestion — the ROY-world alternative.
    pub(super) fn denial_hint(hint: impl Into<String>) -> Self {
        Self {
            prefix: "→".into(),
            text: hint.into(),
            kind: LineKind::DenialHint,
        }
    }

    /// Command not found in the ROY registry.
    pub(super) fn not_found(command: impl Into<String>) -> Self {
        Self {
            prefix: "?".into(),
            text: format!(
                "{} — not in the ROY world  ·  run `help` to see available commands",
                command.into()
            ),
            kind: LineKind::NotFound,
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

pub(super) fn initial_shell_lines() -> Vec<ShellLine> {
    vec![
        ShellLine::output("ROY - shell runtime ready"),
        ShellLine::output("type 'help' for available commands"),
    ]
}

pub(super) fn flatten_chunks(chunks: Vec<String>) -> Vec<String> {
    chunks
        .into_iter()
        .flat_map(|chunk| chunk.split('\n').map(str::to_string).collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
#[path = "terminal_model_tests.rs"]
mod tests;
