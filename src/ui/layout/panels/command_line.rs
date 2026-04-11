#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedCommand {
    pub(crate) command: String,
    pub(crate) args: Vec<String>,
}

/// Minimal shell-like command line parser.
///
/// Supports:
/// - whitespace-delimited tokens
/// - single quotes
/// - double quotes
/// - backslash escapes outside single quotes
/// - empty quoted arguments (`""`, `''`)
///
/// It is intentionally small, but materially better than `split_whitespace()`.
pub(crate) fn parse_command_line(input: &str) -> Result<ParsedCommand, String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Mode {
        Normal,
        SingleQuoted,
        DoubleQuoted,
    }

    let mut mode = Mode::Normal;
    let mut current = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut token_started = false;
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        match mode {
            Mode::Normal => match ch {
                '\'' => {
                    mode = Mode::SingleQuoted;
                    token_started = true;
                }
                '"' => {
                    mode = Mode::DoubleQuoted;
                    token_started = true;
                }
                '\\' => {
                    let Some(escaped) = chars.next() else {
                        return Err("unterminated escape".to_string());
                    };
                    current.push(escaped);
                    token_started = true;
                }
                c if c.is_whitespace() => {
                    if token_started {
                        tokens.push(std::mem::take(&mut current));
                        token_started = false;
                    }
                }
                _ => {
                    current.push(ch);
                    token_started = true;
                }
            },
            Mode::SingleQuoted => match ch {
                '\'' => mode = Mode::Normal,
                _ => {
                    current.push(ch);
                    token_started = true;
                }
            },
            Mode::DoubleQuoted => match ch {
                '"' => mode = Mode::Normal,
                '\\' => {
                    let Some(escaped) = chars.next() else {
                        return Err("unterminated escape in double-quoted string".to_string());
                    };
                    current.push(escaped);
                    token_started = true;
                }
                _ => {
                    current.push(ch);
                    token_started = true;
                }
            },
        }
    }

    match mode {
        Mode::SingleQuoted => return Err("unterminated single quote".to_string()),
        Mode::DoubleQuoted => return Err("unterminated double quote".to_string()),
        Mode::Normal => {}
    }

    if token_started {
        tokens.push(current);
    }

    if tokens.is_empty() {
        return Err("empty command".to_string());
    }

    let command = tokens.remove(0);
    Ok(ParsedCommand {
        command,
        args: tokens,
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
