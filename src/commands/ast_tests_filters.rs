use super::*;

#[test]
fn bare_filter_after_target() {
    let cmd = parse_ok("env PATH TERM");
    assert_eq!(cmd.target.as_ref().expect("target").value, "PATH");
    let Filter::Bare(word) = &cmd.filters[0] else {
        panic!("expected Bare");
    };
    assert_eq!(word.value, "TERM");
}

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
    assert!(matches!(
        &cmd.filters[0],
        Filter::KeyValue { negated: false, .. }
    ));
}

#[test]
fn about_filter_single_word() {
    let cmd = parse_ok("find about security");
    assert!(cmd.target.is_none());
    assert!(matches!(&cmd.filters[0], Filter::About(words) if words[0].value == "security"));
}

#[test]
fn about_filter_multiple_words() {
    let cmd = parse_ok("find about rust async programming");
    let Filter::About(words) = &cmd.filters[0] else {
        panic!("expected About");
    };
    assert_eq!(
        words.iter().map(|w| w.value.as_str()).collect::<Vec<_>>(),
        ["rust", "async", "programming"]
    );
}

#[test]
fn about_filter_stops_at_kv() {
    let cmd = parse_ok("find about security lang:rust");
    let Filter::About(words) = &cmd.filters[0] else {
        panic!("expected About");
    };
    assert_eq!(words.len(), 1);
    assert_eq!(words[0].value, "security");
    assert!(matches!(&cmd.filters[1], Filter::KeyValue { .. }));
}

#[test]
fn about_filter_stops_at_pipe() {
    let cmd = parse_ok("find about security | top 3");
    let Filter::About(words) = &cmd.filters[0] else {
        panic!("expected About");
    };
    assert_eq!(
        words.iter().map(|w| w.value.as_str()).collect::<Vec<_>>(),
        ["security"]
    );
    assert_eq!(cmd.refiners, vec![Refiner::Top(3)]);
}

#[test]
fn about_filter_requires_word() {
    let e = parse_err("find about");
    assert!(e.message.contains("about"));
}

#[test]
fn lines_filter_single() {
    let cmd = parse_ok("read file lines 5");
    assert!(matches!(
        &cmd.filters[0],
        Filter::Lines(LineRange {
            from: Some(5),
            to: None
        })
    ));
}

#[test]
fn lines_filter_range() {
    let cmd = parse_ok("read file lines 5..20");
    assert!(matches!(
        &cmd.filters[0],
        Filter::Lines(LineRange {
            from: Some(5),
            to: Some(20)
        })
    ));
}

#[test]
fn lines_filter_upper_only() {
    let cmd = parse_ok("read file lines ..20");
    assert!(matches!(
        &cmd.filters[0],
        Filter::Lines(LineRange {
            from: None,
            to: Some(20)
        })
    ));
}

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
