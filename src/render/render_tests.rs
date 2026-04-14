//! Tests proving the same CommandResult can be rendered multiple ways without
//! re-execution or semantic drift (REND-01 core contract).

use std::path::PathBuf;

use super::{CommandResult, Diagnostic, NounValue, Severity, human, json};

// ── helpers ───────────────────────────────────────────────────────────────────

fn env_result() -> CommandResult {
    CommandResult::env_map(vec![
        ("HOME".into(), "/home/user".into()),
        ("PATH".into(), "/usr/bin".into()),
    ])
}

fn file_list_result() -> CommandResult {
    CommandResult::file_list(vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/lib.rs"),
    ])
}

fn text_result() -> CommandResult {
    CommandResult::text("hello world", "help")
}

// ── human renderer ────────────────────────────────────────────────────────────

#[test]
fn human_renders_text() {
    let r = text_result();
    assert_eq!(human::render(&r), "hello world");
}

#[test]
fn human_renders_env_map_as_key_eq_value_lines() {
    let r = env_result();
    let out = human::render(&r);
    assert!(out.contains("HOME=/home/user"), "missing HOME line");
    assert!(out.contains("PATH=/usr/bin"), "missing PATH line");
}

#[test]
fn human_renders_file_list_as_paths() {
    let r = file_list_result();
    let out = human::render(&r);
    assert!(out.contains("src/main.rs"));
    assert!(out.contains("src/lib.rs"));
}

#[test]
fn human_renders_empty_value_as_empty_string() {
    let r = CommandResult {
        value: NounValue::Empty,
        schema_key: "session",
        render_mode: crate::commands::plan::RenderMode::Human,
        diagnostics: Vec::new(),
        artifacts: Vec::new(),
    };
    assert_eq!(human::render(&r), "");
}

#[test]
fn human_exit_zero_is_empty() {
    let r = CommandResult {
        value: NounValue::ExitStatus(0),
        schema_key: "session",
        render_mode: crate::commands::plan::RenderMode::Human,
        diagnostics: Vec::new(),
        artifacts: Vec::new(),
    };
    assert_eq!(human::render(&r), "");
}

#[test]
fn human_exit_nonzero_shows_code() {
    let r = CommandResult {
        value: NounValue::ExitStatus(1),
        schema_key: "session",
        render_mode: crate::commands::plan::RenderMode::Human,
        diagnostics: Vec::new(),
        artifacts: Vec::new(),
    };
    assert_eq!(human::render(&r), "exit 1");
}

#[test]
fn human_appends_warnings_after_value() {
    let mut r = text_result();
    r.diagnostics.push(Diagnostic::warning("something odd"));
    let out = human::render(&r);
    assert!(out.contains("hello world"));
    assert!(out.contains("warning: something odd"));
}

#[test]
fn human_info_diagnostics_are_silent() {
    let mut r = text_result();
    r.diagnostics.push(Diagnostic::info("quiet note"));
    let out = human::render(&r);
    assert!(!out.contains("quiet note"), "info must be silent in human mode");
}

// ── json renderer ─────────────────────────────────────────────────────────────

#[test]
fn json_renders_schema_key() {
    let r = env_result();
    let out = json::render(&r);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["schema"], "env");
}

#[test]
fn json_renders_env_map_as_object() {
    let r = env_result();
    let out = json::render(&r);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["value"]["HOME"], "/home/user");
    assert_eq!(v["value"]["PATH"], "/usr/bin");
}

#[test]
fn json_renders_file_list_as_array() {
    let r = file_list_result();
    let out = json::render(&r);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let arr = v["value"].as_array().unwrap();
    assert_eq!(arr.len(), 2);
}

#[test]
fn json_renders_diagnostics_array() {
    let mut r = text_result();
    r.diagnostics.push(Diagnostic::error("bad thing"));
    let out = json::render(&r);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let diags = v["diagnostics"].as_array().unwrap();
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0]["severity"], "error");
    assert_eq!(diags[0]["message"], "bad thing");
}

#[test]
fn json_empty_diagnostics_array() {
    let r = text_result();
    let out = json::render(&r);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(v["diagnostics"].as_array().unwrap().is_empty());
}

// ── same-value, different-render invariant ────────────────────────────────────

#[test]
fn same_env_result_renders_both_ways_without_mutation() {
    let r = env_result();
    let human_out = human::render(&r);
    let json_out = json::render(&r);

    // Human and JSON are different formats.
    assert_ne!(human_out, json_out);

    // The underlying result is unchanged — render again and get the same output.
    assert_eq!(human::render(&r), human_out);
    assert_eq!(json::render(&r), json_out);
}

#[test]
fn same_file_list_renders_both_ways_without_mutation() {
    let r = file_list_result();
    let h1 = human::render(&r);
    let j1 = json::render(&r);
    assert_eq!(human::render(&r), h1);
    assert_eq!(json::render(&r), j1);
}

// ── severity ordering ─────────────────────────────────────────────────────────

#[test]
fn severity_ordering() {
    assert!(Severity::Info < Severity::Warning);
    assert!(Severity::Warning < Severity::Error);
}
