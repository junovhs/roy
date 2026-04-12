//! Typed AST for ROY v0.2 commands.
//!
//! Grammar (BNF-ish):
//! ```text
//! command  = verb [noun-target] filter* refiner-chain? flag*
//! verb     = word
//! noun-target  = word | quoted
//! filter   = kv-filter | neg-filter | about-filter | line-range | bare-filter
//! kv-filter    = word ':' word
//! neg-filter   = '!' word ':' word
//! about-filter = 'about' word+
//! line-range   = 'lines' ( N | N '..' M | '..' M )
//! bare-filter  = word
//! refiner-chain = '|' refiner ( '|' refiner )*
//! refiner  = 'top' N | 'last' N | 'first' N | 'nth' N | 'skip' N
//!          | 'sorted' 'by' word | 'grouped' 'by' word | 'count'
//! flag     = '--dry' | '--json' | '--ref' word
//! ```

#[path = "ast_tokenise.rs"]
mod tokenise;

#[path = "ast_parse.rs"]
mod parse_impl;

pub use parse_impl::parse;

// ── span ─────────────────────────────────────────────────────────────────────

/// Byte-offset range in the original input string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// A value with a source span for error reporting.
#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub(crate) fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

// ── AST nodes ─────────────────────────────────────────────────────────────────

/// A fully-parsed ROY command.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// The action word: `read`, `find`, `show`, `cd`, etc.
    pub verb: Spanned<String>,
    /// First positional after the verb — a path, name, or noun class.
    pub target: Option<Spanned<String>>,
    /// Structured filters applied before dispatch.
    pub filters: Vec<Filter>,
    /// Pipeline refiners applied after the result is produced.
    pub refiners: Vec<Refiner>,
    /// Option flags (`--dry`, `--json`, `--ref <name>`).
    pub flags: Flags,
}

impl Command {
    /// Flatten to an argv-style `(verb, args)` pair for the existing
    /// `ShellRuntime::dispatch` interface. Lossy — flags, refiners, and
    /// typed filters are not yet propagated. Will be removed in LANG-02.
    pub fn to_argv(&self) -> (&str, Vec<String>) {
        let mut args: Vec<String> = Vec::new();
        if let Some(t) = &self.target {
            args.push(t.value.clone());
        }
        for f in &self.filters {
            match f {
                Filter::Bare(s) => args.push(s.value.clone()),
                Filter::KeyValue {
                    key,
                    value,
                    negated,
                } => {
                    if *negated {
                        args.push(format!("!{}:{}", key.value, value.value));
                    } else {
                        args.push(format!("{}:{}", key.value, value.value));
                    }
                }
                Filter::About(words) => {
                    args.push("about".into());
                    args.extend(words.iter().map(|w| w.value.clone()));
                }
                Filter::Lines(r) => {
                    args.push("lines".into());
                    match (r.from, r.to) {
                        (Some(f), Some(t)) => args.push(format!("{}..{}", f, t)),
                        (Some(f), None) => args.push(f.to_string()),
                        (None, Some(t)) => args.push(format!("..{}", t)),
                        (None, None) => {}
                    }
                }
                Filter::Flag(name) => args.push(format!("--{}", name)),
            }
        }
        (&self.verb.value, args)
    }
}

/// A structured filter applied to restrict the noun before dispatch.
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// `key:value` or `!key:value`
    KeyValue {
        key: Spanned<String>,
        value: Spanned<String>,
        negated: bool,
    },
    /// `about <words>`
    About(Vec<Spanned<String>>),
    /// `lines N`, `lines N..M`, or `lines ..M`
    Lines(LineRange),
    /// Unclassified positional (preserved for compat and error recovery).
    Bare(Spanned<String>),
    /// Unrecognised flag token like `--verbose`.
    Flag(String),
}

/// A line-range specifier: `lines N..M`, `lines N`, or `lines ..M`.
#[derive(Debug, Clone, PartialEq)]
pub struct LineRange {
    pub from: Option<u64>,
    pub to: Option<u64>,
}

/// A pipeline refiner applied after the primary result is produced.
#[derive(Debug, Clone, PartialEq)]
pub enum Refiner {
    Top(u64),
    Last(u64),
    First(u64),
    Nth(u64),
    Skip(u64),
    SortedBy(String),
    GroupedBy(String),
    Count,
}

/// Option flags recognised at the top level.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Flags {
    /// `--dry` — describe what would happen without doing it.
    pub dry: bool,
    /// `--json` — output as JSON.
    pub json: bool,
    /// `--ref <name>` — bind the result to a named session ref.
    pub ref_name: Option<String>,
}

// ── parse error ───────────────────────────────────────────────────────────────

/// A descriptive parse error with optional source location and suggestion.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub offset: Option<usize>,
    pub suggestion: Option<String>,
}

impl ParseError {
    pub(crate) fn at(offset: usize, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            offset: Some(offset),
            suggestion: None,
        }
    }

    pub(crate) fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.suggestion = Some(hint.into());
        self
    }

    pub(crate) fn bare(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            offset: None,
            suggestion: None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(hint) = &self.suggestion {
            write!(f, " — {}", hint)?;
        }
        Ok(())
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[path = "ast_parse_helpers.rs"]
mod ast_parse_helpers;

#[cfg(test)]
#[path = "ast_tests.rs"]
mod tests;
