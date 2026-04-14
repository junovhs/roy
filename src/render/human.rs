//! Human-readable text renderer for [`CommandResult`].

use super::{CommandResult, NounValue};

/// Render `result` as a human-readable string for terminal display.
///
/// - Diagnostics are appended after the main value (warnings and errors only;
///   info-level diagnostics are silent in human mode).
/// - The render does **not** mutate `result`; it is safe to render the same
///   result multiple times.
pub fn render(result: &CommandResult) -> String {
    let mut parts: Vec<String> = Vec::new();

    let value_text = render_value(&result.value);
    if !value_text.is_empty() {
        parts.push(value_text);
    }

    for d in &result.diagnostics {
        use super::Severity;
        match d.severity {
            Severity::Warning => parts.push(format!("warning: {}", d.message)),
            Severity::Error => parts.push(format!("error: {}", d.message)),
            Severity::Info => {} // info is silent in human mode
        }
    }

    parts.join("\n")
}

fn render_value(value: &NounValue) -> String {
    match value {
        NounValue::Text(s) => s.clone(),
        NounValue::FileContent { content, .. } => content.clone(),
        NounValue::FileList(paths) => paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        NounValue::EnvMap(pairs) => pairs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n"),
        NounValue::ExitStatus(code) => {
            if *code == 0 {
                String::new()
            } else {
                format!("exit {code}")
            }
        }
        NounValue::Empty => String::new(),
    }
}
