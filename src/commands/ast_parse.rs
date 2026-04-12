//! Parser for the ROY v0.2 command grammar.

use super::tokenise::{tokenise, Token};
use super::{Command, Filter, Flags, LineRange, ParseError, Refiner, Span, Spanned};

// ── public entry point ────────────────────────────────────────────────────────

/// Parse a raw command string into a typed [`Command`] AST.
pub fn parse(input: &str) -> Result<Command, ParseError> {
    let tokens = tokenise(input).map_err(|(off, msg)| ParseError::at(off, msg))?;
    if tokens.is_empty() {
        return Err(ParseError::bare("empty command"));
    }
    let mut pos = 0;
    let verb = take_word(&tokens, &mut pos)?;
    let target = take_target(&tokens, &mut pos);
    let mut filters: Vec<Filter> = Vec::new();
    let mut flags = Flags::default();

    while pos < tokens.len() {
        match &tokens[pos] {
            Token::Pipe(_) => break,
            Token::Quoted(q, off) => {
                filters.push(Filter::Bare(spanned(q.clone(), *off)));
                pos += 1;
            }
            Token::Word(w, off) => {
                let (w, off) = (w.clone(), *off);
                consume_word(&tokens, &mut pos, w, off, &mut filters, &mut flags)?;
            }
        }
    }

    let refiners = parse_refiner_chain(&tokens, &mut pos)?;
    Ok(Command { verb, target, filters, refiners, flags })
}

// ── pre-pipe: filters and flags ───────────────────────────────────────────────

fn consume_word(
    tokens: &[Token],
    pos: &mut usize,
    w: String,
    off: usize,
    filters: &mut Vec<Filter>,
    flags: &mut Flags,
) -> Result<(), ParseError> {
    if w == "--dry" {
        flags.dry = true;
        *pos += 1;
    } else if w == "--json" {
        flags.json = true;
        *pos += 1;
    } else if w == "--ref" {
        *pos += 1;
        flags.ref_name = Some(take_word(tokens, pos)?.value);
    } else if let Some(flag_name) = w.strip_prefix("--") {
        filters.push(Filter::Flag(flag_name.to_string()));
        *pos += 1;
    } else if w == "about" {
        *pos += 1;
        consume_about(tokens, pos, off, filters)?;
    } else if w == "lines" {
        *pos += 1;
        consume_lines(tokens, pos, off, filters)?;
    } else if w.starts_with('!') && w.len() > 1 && w[1..].contains(':') {
        let rest = &w[1..];
        if let Some(c) = rest.find(':') {
            filters.push(Filter::KeyValue {
                key: spanned(rest[..c].to_string(), off + 1),
                value: spanned(rest[c + 1..].to_string(), off + 1 + c + 1),
                negated: true,
            });
        }
        *pos += 1;
    } else if let Some(c) = w.find(':') {
        filters.push(Filter::KeyValue {
            key: spanned(w[..c].to_string(), off),
            value: spanned(w[c + 1..].to_string(), off + c + 1),
            negated: false,
        });
        *pos += 1;
    } else {
        filters.push(Filter::Bare(spanned(w, off)));
        *pos += 1;
    }
    Ok(())
}

fn consume_about(
    tokens: &[Token],
    pos: &mut usize,
    about_off: usize,
    filters: &mut Vec<Filter>,
) -> Result<(), ParseError> {
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
    filters.push(Filter::About(words));
    Ok(())
}

fn consume_lines(
    tokens: &[Token],
    pos: &mut usize,
    lines_off: usize,
    filters: &mut Vec<Filter>,
) -> Result<(), ParseError> {
    match tokens.get(*pos) {
        Some(Token::Word(w, _)) => {
            let range = parse_line_range(w).ok_or_else(|| {
                ParseError::at(lines_off, format!("invalid line range `{}`", w))
                    .with_hint("e.g. `lines 5`, `lines 5..10`, `lines ..20`")
            })?;
            filters.push(Filter::Lines(range));
            *pos += 1;
            Ok(())
        }
        Some(t) => Err(ParseError::at(t.start(), "`lines` requires a range specifier")),
        None => Err(ParseError::at(lines_off, "`lines` requires a range specifier")),
    }
}

fn parse_line_range(s: &str) -> Option<LineRange> {
    if let Some(i) = s.find("..") {
        let from = if s[..i].is_empty() { None } else { Some(s[..i].parse::<u64>().ok()?) };
        let after = &s[i + 2..];
        let to = if after.is_empty() { None } else { Some(after.parse::<u64>().ok()?) };
        Some(LineRange { from, to })
    } else {
        Some(LineRange { from: Some(s.parse().ok()?), to: None })
    }
}

// ── refiner chain ─────────────────────────────────────────────────────────────

fn parse_refiner_chain(tokens: &[Token], pos: &mut usize) -> Result<Vec<Refiner>, ParseError> {
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

// ── token-level helpers ───────────────────────────────────────────────────────

fn take_word(tokens: &[Token], pos: &mut usize) -> Result<Spanned<String>, ParseError> {
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

fn take_target(tokens: &[Token], pos: &mut usize) -> Option<Spanned<String>> {
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

fn is_target_word(w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with("--")
        && !w.contains(':')
        && !w.starts_with('!')
        && w != "about"
        && w != "lines"
}

fn take_u64(tokens: &[Token], pos: &mut usize, ctx_off: usize) -> Result<u64, ParseError> {
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

fn take_by(tokens: &[Token], pos: &mut usize, ctx_off: usize) -> Result<(), ParseError> {
    match tokens.get(*pos) {
        Some(Token::Word(w, _)) if w == "by" => {
            *pos += 1;
            Ok(())
        }
        Some(t) => Err(ParseError::at(t.start(), "expected `by`")),
        None => Err(ParseError::at(ctx_off, "expected `by` after sorted/grouped")),
    }
}

fn spanned(value: String, off: usize) -> Spanned<String> {
    let end = off + value.len();
    Spanned::new(value, Span::new(off, end))
}
