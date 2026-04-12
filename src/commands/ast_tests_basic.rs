use super::*;

#[test]
fn verb_only() {
    let cmd = parse_ok("pwd");
    assert_eq!(cmd.verb.value, "pwd");
    assert!(cmd.target.is_none());
    assert!(cmd.filters.is_empty());
    assert!(cmd.refiners.is_empty());
}

#[test]
fn empty_input_is_error() {
    let e = parse_err("");
    assert!(e.message.contains("empty"));
}

#[test]
fn verb_and_target() {
    let cmd = parse_ok("read src/main.rs");
    assert_eq!(cmd.verb.value, "read");
    assert_eq!(cmd.target.as_ref().expect("target").value, "src/main.rs");
    assert!(cmd.filters.is_empty());
}

#[test]
fn quoted_target_double() {
    let cmd = parse_ok(r#"cd "my folder""#);
    assert_eq!(cmd.target.as_ref().expect("target").value, "my folder");
}

#[test]
fn quoted_target_single() {
    let cmd = parse_ok("cd 'my folder'");
    assert_eq!(cmd.target.as_ref().expect("target").value, "my folder");
}

#[test]
fn backslash_escaped_space_in_target() {
    let cmd = parse_ok(r"cd my\ folder");
    assert_eq!(cmd.target.as_ref().expect("target").value, "my folder");
}

#[test]
fn backslash_escape_in_double_quoted() {
    let cmd = parse_ok("read \"has \\\"inner\\\" quote\"");
    assert_eq!(
        cmd.target.as_ref().expect("target").value,
        "has \"inner\" quote"
    );
}

#[test]
fn empty_quoted_string_as_target() {
    let cmd = parse_ok(r#"env """#);
    assert_eq!(cmd.target.as_ref().expect("target").value, "");
}

#[test]
fn complex_command() {
    let cmd = parse_ok("find files role:test !lang:rust --dry | sorted by name | top 5");
    assert_eq!(cmd.verb.value, "find");
    assert_eq!(cmd.target.as_ref().expect("target").value, "files");
    assert!(cmd.flags.dry);
    assert_eq!(cmd.refiners.len(), 2);
    assert!(matches!(
        &cmd.filters[0],
        Filter::KeyValue { negated: false, .. }
    ));
    assert!(matches!(
        &cmd.filters[1],
        Filter::KeyValue { negated: true, .. }
    ));
}

#[test]
fn unterminated_single_quote_error() {
    let e = parse_err("cd 'oops");
    assert!(e.message.contains("unterminated single quote"));
}

#[test]
fn unterminated_double_quote_error() {
    let e = parse_err(r#"cd "oops"#);
    assert!(e.message.contains("unterminated double quote"));
}

#[test]
fn unterminated_escape_error() {
    let e = parse_err(r"cd foo\");
    assert!(e.message.contains("unterminated escape"));
}
