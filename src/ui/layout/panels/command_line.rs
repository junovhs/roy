#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedCommand {
    pub(crate) command: String,
    pub(crate) args: Vec<String>,
}

/// Parse a command line using the ROY v0.2 AST parser, then flatten the
/// result to the legacy `ParsedCommand` interface for existing dispatch.
///
/// The full typed AST is available via `crate::commands::ast::parse` directly.
pub(crate) fn parse_command_line(input: &str) -> Result<ParsedCommand, String> {
    let ast = crate::commands::ast::parse(input).map_err(|e| e.to_string())?;
    let (verb, args) = ast.to_argv();
    Ok(ParsedCommand {
        command: verb.to_string(),
        args,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_command() {
        let parsed = parse_command_line("pwd").unwrap();
        assert_eq!(parsed.command, "pwd");
        assert!(parsed.args.is_empty());
    }

    #[test]
    fn parses_whitespace_delimited_args() {
        let parsed = parse_command_line("env PATH TERM").unwrap();
        assert_eq!(parsed.command, "env");
        assert_eq!(parsed.args, vec!["PATH", "TERM"]);
    }

    #[test]
    fn parses_double_quoted_arg() {
        let parsed = parse_command_line(r#"cd "my folder""#).unwrap();
        assert_eq!(parsed.command, "cd");
        assert_eq!(parsed.args, vec!["my folder"]);
    }

    #[test]
    fn parses_single_quoted_arg() {
        let parsed = parse_command_line("cd 'my folder'").unwrap();
        assert_eq!(parsed.command, "cd");
        assert_eq!(parsed.args, vec!["my folder"]);
    }

    #[test]
    fn parses_escaped_space() {
        let parsed = parse_command_line(r#"cd my\ folder"#).unwrap();
        assert_eq!(parsed.command, "cd");
        assert_eq!(parsed.args, vec!["my folder"]);
    }

    #[test]
    fn preserves_empty_quoted_arg() {
        let parsed = parse_command_line(r#"env "" PATH"#).unwrap();
        assert_eq!(parsed.command, "env");
        assert_eq!(parsed.args, vec!["", "PATH"]);
    }

    #[test]
    fn rejects_unterminated_single_quote() {
        let err = parse_command_line("cd 'oops").unwrap_err();
        assert!(err.contains("unterminated single quote"));
    }

    #[test]
    fn rejects_unterminated_double_quote() {
        let err = parse_command_line(r#"cd "oops"#).unwrap_err();
        assert!(err.contains("unterminated double quote"));
    }

    #[test]
    fn rejects_unterminated_escape() {
        let err = parse_command_line(r#"cd foo\"#).unwrap_err();
        assert!(err.contains("unterminated escape"));
    }
}
