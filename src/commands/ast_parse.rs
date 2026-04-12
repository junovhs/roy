//! Parser for the ROY v0.2 command grammar.

use super::ast_parse_helpers::{
    consume_about, consume_lines, parse_refiner_chain, spanned, take_target, take_word,
};
use super::tokenise::{tokenise, Token};
use super::{Command, Filter, Flags, ParseError};

struct PrefixState {
    pos: usize,
    filters: Vec<Filter>,
    flags: Flags,
}

impl PrefixState {
    fn new(pos: usize) -> Self {
        Self {
            pos,
            filters: Vec::new(),
            flags: Flags::default(),
        }
    }

    fn consume_word(
        &mut self,
        tokens: &[Token],
        word: String,
        off: usize,
    ) -> Result<(), ParseError> {
        if word == "--dry" {
            self.flags.dry = true;
            self.pos += 1;
        } else if word == "--json" {
            self.flags.json = true;
            self.pos += 1;
        } else if word == "--ref" {
            self.pos += 1;
            self.flags.ref_name = Some(take_word(tokens, &mut self.pos)?.value);
        } else if let Some(flag_name) = word.strip_prefix("--") {
            self.filters.push(Filter::Flag(flag_name.to_string()));
            self.pos += 1;
        } else if word == "about" {
            self.pos += 1;
            self.filters
                .push(consume_about(tokens, &mut self.pos, off)?);
        } else if word == "lines" {
            self.pos += 1;
            self.filters
                .push(consume_lines(tokens, &mut self.pos, off)?);
        } else if word.starts_with('!') && word.len() > 1 && word[1..].contains(':') {
            let rest = &word[1..];
            if let Some(colon) = rest.find(':') {
                self.filters.push(Filter::KeyValue {
                    key: spanned(rest[..colon].to_string(), off + 1),
                    value: spanned(rest[colon + 1..].to_string(), off + colon + 2),
                    negated: true,
                });
            }
            self.pos += 1;
        } else if let Some(colon) = word.find(':') {
            self.filters.push(Filter::KeyValue {
                key: spanned(word[..colon].to_string(), off),
                value: spanned(word[colon + 1..].to_string(), off + colon + 1),
                negated: false,
            });
            self.pos += 1;
        } else {
            self.filters.push(Filter::Bare(spanned(word, off)));
            self.pos += 1;
        }
        Ok(())
    }
}

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
    let mut prefix = PrefixState::new(pos);

    while prefix.pos < tokens.len() {
        match &tokens[prefix.pos] {
            Token::Pipe(_) => break,
            Token::Quoted(q, off) => {
                prefix.filters.push(Filter::Bare(spanned(q.clone(), *off)));
                prefix.pos += 1;
            }
            Token::Word(w, off) => {
                let (w, off) = (w.clone(), *off);
                prefix.consume_word(&tokens, w, off)?;
            }
        }
    }

    let refiners = parse_refiner_chain(&tokens, &mut prefix.pos)?;
    Ok(Command {
        verb,
        target,
        filters: prefix.filters,
        refiners,
        flags: prefix.flags,
    })
}
