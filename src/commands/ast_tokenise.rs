//! Raw tokeniser for the ROY v0.2 command grammar.

/// A single lexical unit with its byte offset in the source string.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    /// Unquoted word (may contain `:`, `!`, `--`, `.`).
    Word(String, usize),
    /// Single- or double-quoted string (quotes stripped).
    Quoted(String, usize),
    /// The `|` pipe character.
    Pipe(usize),
}

impl Token {
    pub(crate) fn start(&self) -> usize {
        match self {
            Self::Word(_, s) | Self::Quoted(_, s) | Self::Pipe(s) => *s,
        }
    }
}

/// Tokenise `input` or return `(byte_offset, error_message)`.
pub(crate) fn tokenise(input: &str) -> Result<Vec<Token>, (usize, String)> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'\t' => i += 1,
            b'|' => {
                out.push(Token::Pipe(i));
                i += 1;
            }
            b'\'' => {
                let start = i;
                i += 1;
                let (s, end) = read_single_quoted(bytes, i)
                    .map_err(|()| (start, "unterminated single quote".to_string()))?;
                out.push(Token::Quoted(s, start));
                i = end;
            }
            b'"' => {
                let start = i;
                i += 1;
                let (s, end) = read_double_quoted(bytes, i)
                    .map_err(|msg| (start, msg))?;
                out.push(Token::Quoted(s, start));
                i = end;
            }
            _ => {
                let start = i;
                let (text, end) = read_bare_word(bytes, i)?;
                out.push(Token::Word(text, start));
                i = end;
            }
        }
    }
    Ok(out)
}

fn read_bare_word(bytes: &[u8], mut i: usize) -> Result<(String, usize), (usize, String)> {
    let mut text = String::new();
    while i < bytes.len() && !matches!(bytes[i], b' ' | b'\t' | b'|' | b'\'' | b'"') {
        if bytes[i] == b'\\' {
            if i + 1 < bytes.len() {
                text.push(bytes[i + 1] as char);
                i += 2;
            } else {
                return Err((i, "unterminated escape".to_string()));
            }
        } else {
            text.push(bytes[i] as char);
            i += 1;
        }
    }
    Ok((text, i))
}

fn read_single_quoted(bytes: &[u8], mut i: usize) -> Result<(String, usize), ()> {
    let mut out = String::new();
    while i < bytes.len() {
        if bytes[i] == b'\'' {
            return Ok((out, i + 1));
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    Err(())
}

fn read_double_quoted(bytes: &[u8], mut i: usize) -> Result<(String, usize), String> {
    let mut out = String::new();
    while i < bytes.len() {
        match bytes[i] {
            b'"' => return Ok((out, i + 1)),
            b'\\' if i + 1 < bytes.len() => {
                out.push(bytes[i + 1] as char);
                i += 2;
            }
            b'\\' => return Err("unterminated escape".to_string()),
            b => {
                out.push(b as char);
                i += 1;
            }
        }
    }
    Err("unterminated double quote".to_string())
}
