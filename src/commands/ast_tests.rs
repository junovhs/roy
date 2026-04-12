use super::*;

fn parse_ok(input: &str) -> Command {
    parse(input).expect("parse should succeed")
}

fn parse_err(input: &str) -> ParseError {
    parse(input).expect_err("parse should fail")
}

mod ast_tests_basic;
mod ast_tests_filters;
mod ast_tests_refiners;
