//! JSON renderer for [`CommandResult`].
//!
//! Produces a stable envelope:
//! ```json
//! {
//!   "schema": "<schema_key>",
//!   "value":  <type-specific payload>,
//!   "diagnostics": [...]
//! }
//! ```
//!
//! The `"schema"` field is the stable [`CommandResult::schema_key`] that
//! downstream tooling can use to parse `"value"`. Schemas are formalised
//! in JSON-01; this renderer produces forward-compatible JSON now.

use serde_json::{json, Value};

use super::{CommandResult, NounValue, Severity};

/// Render `result` as a JSON string.
///
/// The output is compact (no pretty-printing) unless the internal
/// pretty flag is set — use [`render_pretty`] for indented output.
pub fn render(result: &CommandResult) -> String {
    render_impl(result, false)
}

/// Render `result` as indented JSON (2-space indent).
pub fn render_pretty(result: &CommandResult) -> String {
    render_impl(result, true)
}

fn render_impl(result: &CommandResult, pretty: bool) -> String {
    let value = build_value_json(&result.value);
    let diagnostics: Vec<Value> = result
        .diagnostics
        .iter()
        .map(|d| {
            let sev = match d.severity {
                Severity::Info => "info",
                Severity::Warning => "warning",
                Severity::Error => "error",
            };
            let mut obj = json!({ "severity": sev, "message": d.message });
            if let Some(code) = d.code {
                obj["code"] = json!(code);
            }
            obj
        })
        .collect();

    let envelope = json!({
        "schema": result.schema_key,
        "value":  value,
        "diagnostics": diagnostics,
    });

    if pretty {
        serde_json::to_string_pretty(&envelope).unwrap_or_else(|e| {
            json!({"error": e.to_string()}).to_string()
        })
    } else {
        serde_json::to_string(&envelope).unwrap_or_else(|e| {
            json!({"error": e.to_string()}).to_string()
        })
    }
}

fn build_value_json(value: &NounValue) -> Value {
    match value {
        NounValue::Text(s) => json!(s),
        NounValue::FileContent { path, content } => json!({
            "path": path.display().to_string(),
            "content": content,
        }),
        NounValue::FileList(paths) => {
            let strs: Vec<&str> = paths
                .iter()
                .filter_map(|p| p.to_str())
                .collect();
            json!(strs)
        }
        NounValue::EnvMap(pairs) => {
            let obj: serde_json::Map<String, Value> = pairs
                .iter()
                .map(|(k, v)| (k.clone(), json!(v)))
                .collect();
            Value::Object(obj)
        }
        NounValue::ExitStatus(code) => json!(code),
        NounValue::Empty => Value::Null,
    }
}
