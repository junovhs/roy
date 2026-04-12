use super::*;

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
    assert_eq!(
        cmd.refiners,
        vec![Refiner::SortedBy("name".into()), Refiner::Top(5)]
    );
}

#[test]
fn refiner_unknown_is_error() {
    let e = parse_err("ls | explode");
    assert!(e.message.contains("unknown refiner"));
}

#[test]
fn sorted_requires_by_keyword() {
    let e = parse_err("ls | sorted name");
    assert!(
        e.message.contains("by"),
        "expected 'by' error, got: {}",
        e.message
    );
}

#[test]
fn grouped_requires_by_keyword() {
    let e = parse_err("ls | grouped type");
    assert!(
        e.message.contains("by"),
        "expected 'by' error, got: {}",
        e.message
    );
}
