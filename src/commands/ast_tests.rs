use super::*;

fn parse_ok(input: &str) -> Command {
    parse(input).expect("parse should succeed")
}
fn parse_err(input: &str) -> ParseError {
    parse(input).expect_err("parse should fail")
}

// ── verb only ─────────────────────────────────────────────────────────────────

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

// ── target ────────────────────────────────────────────────────────────────────

#[test]
fn verb_and_target() {
    let cmd = parse_ok("read src/main.rs");
    assert_eq!(cmd.verb.value, "read");
    assert_eq!(cmd.target.as_ref().unwrap().value, "src/main.rs");
    assert!(cmd.filters.is_empty());
}

#[test]
fn quoted_target_double() {
    let cmd = parse_ok(r#"cd "my folder""#);
    assert_eq!(cmd.target.as_ref().unwrap().value, "my folder");
}

#[test]
fn quoted_target_single() {
    let cmd = parse_ok("cd 'my folder'");
    assert_eq!(cmd.target.as_ref().unwrap().value, "my folder");
}

#[test]
fn backslash_escaped_space_in_target() {
    let cmd = parse_ok(r"cd my\ folder");
    assert_eq!(cmd.target.as_ref().unwrap().value, "my folder");
}

#[test]
fn backslash_escape_in_double_quoted() {
    let cmd = parse_ok("read \"has \\\"inner\\\" quote\"");
    assert_eq!(cmd.target.as_ref().unwrap().value, "has \"inner\" quote");
}

#[test]
fn empty_quoted_string_as_target() {
    let cmd = parse_ok(r#"env """#);
    assert_eq!(cmd.target.as_ref().unwrap().value, "");
}

// ── filters: bare ─────────────────────────────────────────────────────────────

#[test]
fn bare_filter_after_target() {
    let cmd = parse_ok("env PATH TERM");
    assert_eq!(cmd.target.as_ref().unwrap().value, "PATH");
    let Filter::Bare(word) = &cmd.filters[0] else { panic!("expected Bare") };
    assert_eq!(word.value, "TERM");
}

// ── filters: key:value ────────────────────────────────────────────────────────

#[test]
fn kv_filter() {
    let cmd = parse_ok("find role:admin");
    assert!(matches!(
        &cmd.filters[0],
        Filter::KeyValue { key, value, negated: false }
            if key.value == "role" && value.value == "admin"
    ));
}

#[test]
fn negated_kv_filter() {
    let cmd = parse_ok("find !role:admin");
    assert!(matches!(
        &cmd.filters[0],
        Filter::KeyValue { key, value, negated: true }
            if key.value == "role" && value.value == "admin"
    ));
}

#[test]
fn kv_filter_no_target() {
    let cmd = parse_ok("find type:note");
    assert!(cmd.target.is_none());
    assert!(matches!(&cmd.filters[0], Filter::KeyValue { negated: false, .. }));
}

// ── filters: about ────────────────────────────────────────────────────────────

#[test]
fn about_filter_single_word() {
    let cmd = parse_ok("find about security");
    assert!(cmd.target.is_none());
    assert!(matches!(&cmd.filters[0], Filter::About(words) if words[0].value == "security"));
}

#[test]
fn about_filter_multiple_words() {
    let cmd = parse_ok("find about rust async programming");
    let Filter::About(words) = &cmd.filters[0] else { panic!("expected About") };
    assert_eq!(words.iter().map(|w| w.value.as_str()).collect::<Vec<_>>(), ["rust", "async", "programming"]);
}

#[test]
fn about_filter_stops_at_kv() {
    let cmd = parse_ok("find about security lang:rust");
    let Filter::About(words) = &cmd.filters[0] else { panic!() };
    assert_eq!(words.len(), 1);
    assert_eq!(words[0].value, "security");
    assert!(matches!(&cmd.filters[1], Filter::KeyValue { .. }));
}

#[test]
fn about_filter_stops_at_pipe() {
    let cmd = parse_ok("find about security | top 3");
    let Filter::About(words) = &cmd.filters[0] else { panic!() };
    assert_eq!(words.len(), 1, "pipe must terminate about, got: {:?}", words.iter().map(|w| &w.value).collect::<Vec<_>>());
    assert_eq!(cmd.refiners, vec![Refiner::Top(3)]);
}

#[test]
fn about_filter_requires_word() {
    let e = parse_err("find about");
    assert!(e.message.contains("about"));
}

// ── filters: lines ────────────────────────────────────────────────────────────

#[test]
fn lines_filter_single() {
    let cmd = parse_ok("read file lines 5");
    assert!(matches!(&cmd.filters[0], Filter::Lines(LineRange { from: Some(5), to: None })));
}

#[test]
fn lines_filter_range() {
    let cmd = parse_ok("read file lines 5..20");
    assert!(matches!(&cmd.filters[0], Filter::Lines(LineRange { from: Some(5), to: Some(20) })));
}

#[test]
fn lines_filter_upper_only() {
    let cmd = parse_ok("read file lines ..20");
    assert!(matches!(&cmd.filters[0], Filter::Lines(LineRange { from: None, to: Some(20) })));
}

// ── flags ─────────────────────────────────────────────────────────────────────

#[test]
fn flag_dry() {
    let cmd = parse_ok("write file.txt --dry");
    assert!(cmd.flags.dry);
    assert!(!cmd.flags.json);
}

#[test]
fn flag_json() {
    let cmd = parse_ok("ls --json");
    assert!(cmd.flags.json);
}

#[test]
fn flag_ref() {
    let cmd = parse_ok("find files --ref results");
    assert_eq!(cmd.flags.ref_name.as_deref(), Some("results"));
}

// ── refiners ──────────────────────────────────────────────────────────────────

#[test]
fn refiner_top() {
    let cmd = parse_ok("find files | top 10");
    assert_eq!(cmd.refiners, vec![Refiner::Top(10)]);
}

#[test]
fn refiner_last() {
    let cmd = parse_ok("ls | last 3");
    assert_eq!(cmd.refiners[0], Refiner::Last(3));
}

#[test]
fn refiner_first() {
    let cmd = parse_ok("ls | first 1");
    assert_eq!(cmd.refiners[0], Refiner::First(1));
}

#[test]
fn refiner_nth() {
    let cmd = parse_ok("ls | nth 2");
    assert_eq!(cmd.refiners[0], Refiner::Nth(2));
}

#[test]
fn refiner_skip() {
    let cmd = parse_ok("ls | skip 5");
    assert_eq!(cmd.refiners[0], Refiner::Skip(5));
}

#[test]
fn refiner_sorted_by() {
    let cmd = parse_ok("find files | sorted by name");
    assert_eq!(cmd.refiners[0], Refiner::SortedBy("name".into()));
}

#[test]
fn refiner_grouped_by() {
    let cmd = parse_ok("find files | grouped by type");
    assert_eq!(cmd.refiners[0], Refiner::GroupedBy("type".into()));
}

#[test]
fn refiner_count() {
    let cmd = parse_ok("find files | count");
    assert_eq!(cmd.refiners[0], Refiner::Count);
}

#[test]
fn multiple_refiners_chained() {
    let cmd = parse_ok("find files | sorted by name | top 5");
    assert_eq!(cmd.refiners, vec![Refiner::SortedBy("name".into()), Refiner::Top(5)]);
}

#[test]
fn refiner_unknown_is_error() {
    let e = parse_err("ls | explode");
    assert!(e.message.contains("unknown refiner"));
}

#[test]
fn sorted_requires_by_keyword() {
    let e = parse_err("ls | sorted name");
    assert!(e.message.contains("by"), "expected 'by' error, got: {}", e.message);
}

#[test]
fn grouped_requires_by_keyword() {
    let e = parse_err("ls | grouped type");
    assert!(e.message.contains("by"), "expected 'by' error, got: {}", e.message);
}

// ── complex ───────────────────────────────────────────────────────────────────

#[test]
fn complex_command() {
    let cmd = parse_ok("find files role:test !lang:rust --dry | sorted by name | top 5");
    assert_eq!(cmd.verb.value, "find");
    assert_eq!(cmd.target.as_ref().unwrap().value, "files");
    assert!(cmd.flags.dry);
    assert_eq!(cmd.refiners.len(), 2);
    assert!(matches!(&cmd.filters[0], Filter::KeyValue { negated: false, .. }));
    assert!(matches!(&cmd.filters[1], Filter::KeyValue { negated: true, .. }));
}

// ── errors ────────────────────────────────────────────────────────────────────

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
