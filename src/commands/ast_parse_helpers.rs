use super::tokenise::Token;
use super::{LineRange, ParseError, Refiner, Span, Spanned};

pub(super) fn parse_refiner_chain(
    tokens: &[Token],
    pos: &mut usize,
) -> Result<Vec<Refiner>, ParseError> {
    let mut refiners = Vec::new();
    while *pos < tokens.len() {
        match &tokens[*pos] {
            Token::Pipe(_) => {
                *pos += 1;
                refiners.push(parse_refiner(tokens, pos)?);
            }
            _ => break,
        }
    }
    Ok(refiners)
}

pub(super) fn consume_about(
    tokens: &[Token],
    pos: &mut usize,
    about_off: usize,
) -> Result<super::Filter, ParseError> {
    let mut words: Vec<Spanned<String>> = Vec::new();
    while let Some(tok) = tokens.get(*pos) {
        match tok {
            Token::Pipe(_) => break,
            Token::Word(w, off) if !w.starts_with("--") && !w.contains(':') => {
                words.push(spanned(w.clone(), *off));
                *pos += 1;
            }
            _ => break,
        }
    }
    if words.is_empty() {
        return Err(
            ParseError::at(about_off, "`about` requires at least one word")
                .with_hint("e.g. `find about security`"),
        );
    }
    Ok(super::Filter::About(words))
}

pub(super) fn consume_lines(
    tokens: &[Token],
    pos: &mut usize,
    lines_off: usize,
) -> Result<super::Filter, ParseError> {
    match tokens.get(*pos) {
        Some(Token::Word(w, _)) => {
            let range = parse_line_range(w).ok_or_else(|| {
                ParseError::at(lines_off, format!("invalid line range `{}`", w))
                    .with_hint("e.g. `lines 5`, `lines 5..10`, `lines ..20`")
            })?;
            *pos += 1;
            Ok(super::Filter::Lines(range))
        }
        Some(t) => Err(ParseError::at(
            t.start(),
            "`lines` requires a range specifier",
        )),
        None => Err(ParseError::at(
            lines_off,
            "`lines` requires a range specifier",
        )),
    }
}

pub(super) fn take_word(tokens: &[Token], pos: &mut usize) -> Result<Spanned<String>, ParseError> {
    match tokens.get(*pos) {
        Some(Token::Word(w, o)) => {
            let s = spanned(w.clone(), *o);
            *pos += 1;
            Ok(s)
        }
        Some(Token::Quoted(q, o)) => {
            let s = spanned(q.clone(), *o);
            *pos += 1;
            Ok(s)
        }
        Some(t) => Err(ParseError::at(t.start(), "expected a word")),
        None => Err(ParseError::bare("unexpected end of input")),
    }
}

pub(super) fn take_target(tokens: &[Token], pos: &mut usize) -> Option<Spanned<String>> {
    match tokens.get(*pos) {
        Some(Token::Quoted(q, o)) => {
            let s = spanned(q.clone(), *o);
            *pos += 1;
            Some(s)
        }
        Some(Token::Word(w, o)) if is_target_word(w) => {
            let s = spanned(w.clone(), *o);
            *pos += 1;
            Some(s)
        }
        _ => None,
    }
}

pub(super) fn take_u64(
    tokens: &[Token],
    pos: &mut usize,
    ctx_off: usize,
) -> Result<u64, ParseError> {
    match tokens.get(*pos) {
        Some(Token::Word(w, o)) => {
            let n = w
                .parse::<u64>()
                .map_err(|_| ParseError::at(*o, format!("`{}` is not a valid number", w)))?;
            *pos += 1;
            Ok(n)
        }
        Some(t) => Err(ParseError::at(t.start(), "expected a number")),
        None => Err(ParseError::at(ctx_off, "expected a number after refiner")),
    }
}

pub(super) fn take_by(tokens: &[Token], pos: &mut usize, ctx_off: usize) -> Result<(), ParseError> {
    match tokens.get(*pos) {
        Some(Token::Word(w, _)) if w == "by" => {
            *pos += 1;
            Ok(())
        }
        Some(t) => Err(ParseError::at(t.start(), "expected `by`")),
        None => Err(ParseError::at(
            ctx_off,
            "expected `by` after sorted/grouped",
        )),
    }
}

pub(super) fn spanned(value: String, off: usize) -> Spanned<String> {
    let end = off + value.len();
    Spanned::new(value, Span::new(off, end))
}

fn parse_line_range(s: &str) -> Option<LineRange> {
    if let Some(i) = s.find("..") {
        let from = if s[..i].is_empty() {
            None
        } else {
            Some(s[..i].parse::<u64>().ok()?)
        };
        let after = &s[i + 2..];
        let to = if after.is_empty() {
            None
        } else {
            Some(after.parse::<u64>().ok()?)
        };
        Some(LineRange { from, to })
    } else {
        Some(LineRange {
            from: Some(s.parse().ok()?),
            to: None,
        })
    }
}

fn parse_refiner(tokens: &[Token], pos: &mut usize) -> Result<Refiner, ParseError> {
    let Some(tok) = tokens.get(*pos) else {
        return Err(ParseError::bare("expected refiner after `|`"));
    };
    let (verb, off) = match tok {
        Token::Word(w, o) => (w.clone(), *o),
        Token::Quoted(_, o) => return Err(ParseError::at(*o, "refiner must be a bare word")),
        Token::Pipe(o) => return Err(ParseError::at(*o, "expected refiner, got `|`")),
    };
    *pos += 1;
    match verb.as_str() {
        "top" => Ok(Refiner::Top(take_u64(tokens, pos, off)?)),
        "last" => Ok(Refiner::Last(take_u64(tokens, pos, off)?)),
        "first" => Ok(Refiner::First(take_u64(tokens, pos, off)?)),
        "nth" => Ok(Refiner::Nth(take_u64(tokens, pos, off)?)),
        "skip" => Ok(Refiner::Skip(take_u64(tokens, pos, off)?)),
        "sorted" => {
            take_by(tokens, pos, off)?;
            Ok(Refiner::SortedBy(take_word(tokens, pos)?.value))
        }
        "grouped" => {
            take_by(tokens, pos, off)?;
            Ok(Refiner::GroupedBy(take_word(tokens, pos)?.value))
        }
        "count" => Ok(Refiner::Count),
        other => Err(
            ParseError::at(off, format!("unknown refiner `{}`", other)).with_hint(
                "valid refiners: top, last, first, nth, skip, sorted by, grouped by, count",
            ),
        ),
    }
}

fn is_target_word(w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with("--")
        && !w.contains(':')
        && !w.starts_with('!')
        && w != "about"
        && w != "lines"
}
