//! alacritty_terminal state and styled-cell helpers for the agent terminal view.

use std::sync::{Arc, Mutex};

use alacritty_terminal::Term;
use alacritty_terminal::event::VoidListener;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::term::Config as TermConfig;
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::vte::ansi::{Color, NamedColor, Processor};

pub(super) const TERM_COLS: usize = 220;
const TERM_ROWS: usize = 50;

struct TermDims;
impl Dimensions for TermDims {
    fn total_lines(&self) -> usize { TERM_ROWS }
    fn screen_lines(&self) -> usize { TERM_ROWS }
    fn columns(&self) -> usize { TERM_COLS }
}

/// Alacritty terminal + VTE parser, wrapped for lock-based sharing.
pub(super) struct TermState {
    term: Term<VoidListener>,
    parser: Processor,
}

pub(super) type TermHandle = Arc<Mutex<TermState>>;

pub(super) fn new_term_handle() -> TermHandle {
    Arc::new(Mutex::new(TermState {
        term: Term::new(TermConfig::default(), &TermDims, VoidListener),
        parser: Processor::default(),
    }))
}

impl TermState {
    pub(super) fn feed(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.term, bytes);
    }

    /// Styled rows: `(char, fg_rgba, bg_rgba, flags_bits)`.
    /// 0-rgba means "default" (transparent/inherit).  Trailing empty rows
    /// are trimmed so the rendered grid doesn't show a sea of whitespace.
    pub(super) fn styled_rows(&self) -> Vec<Vec<(char, u32, u32, u16)>> {
        let content = self.term.renderable_content();
        let rows = self.term.screen_lines();
        let cols = self.term.columns();
        let default = (' ', 0u32, 0u32, 0u16);
        let mut grid = vec![vec![default; cols]; rows];

        for indexed in content.display_iter {
            let ln = indexed.point.line.0;
            let cn = indexed.point.column.0;
            if ln >= 0 && (ln as usize) < rows && cn < cols {
                let cell = indexed.cell;
                grid[ln as usize][cn] = (
                    cell.c,
                    color_rgba(cell.fg),
                    color_rgba(cell.bg),
                    cell.flags.bits(),
                );
            }
        }

        let last = grid.iter().rposition(|r| {
            r.iter().any(|(c, fg, bg, _)| *c != ' ' || *fg != 0 || *bg != 0)
        }).unwrap_or(0);
        grid.truncate(last + 1);
        grid
    }

    /// Plain text rows for appending to transcript on agent exit.
    pub(super) fn text_rows(&self) -> Vec<String> {
        let content = self.term.renderable_content();
        let rows = self.term.screen_lines();
        let cols = self.term.columns();
        let mut grid = vec![vec![' '; cols]; rows];

        for indexed in content.display_iter {
            let ln = indexed.point.line.0;
            let cn = indexed.point.column.0;
            if ln >= 0 && (ln as usize) < rows && cn < cols {
                grid[ln as usize][cn] = indexed.cell.c;
            }
        }

        let last = grid.iter().rposition(|r| r.iter().any(|c| *c != ' ')).unwrap_or(0);
        grid[..=last].iter().map(|r| r.iter().collect::<String>().trim_end().to_string()).collect()
    }
}

// ── color helpers ─────────────────────────────────────────────────────────────

fn color_rgba(c: Color) -> u32 {
    match c {
        Color::Named(NamedColor::Foreground | NamedColor::Background) => 0,
        Color::Named(n) => named_rgba(n),
        Color::Indexed(idx) => indexed_rgba(idx),
        Color::Spec(rgb) => pack_rgb(rgb.r, rgb.g, rgb.b),
    }
}

fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xff
}

fn named_rgba(n: NamedColor) -> u32 {
    let (r, g, b): (u8, u8, u8) = match n {
        NamedColor::Black       => (0x1c, 0x1c, 0x1c),
        NamedColor::Red         => (0xcc, 0x55, 0x55),
        NamedColor::Green       => (0x55, 0xaa, 0x55),
        NamedColor::Yellow      => (0xaa, 0xaa, 0x55),
        NamedColor::Blue        => (0x55, 0x55, 0xcc),
        NamedColor::Magenta     => (0xaa, 0x55, 0xaa),
        NamedColor::Cyan        => (0x55, 0xaa, 0xaa),
        NamedColor::White       => (0xe6, 0xe4, 0xdf),
        NamedColor::BrightBlack => (0x55, 0x55, 0x55),
        NamedColor::BrightRed   => (0xff, 0x55, 0x55),
        NamedColor::BrightGreen => (0x55, 0xff, 0x55),
        NamedColor::BrightYellow=> (0xff, 0xff, 0x55),
        NamedColor::BrightBlue  => (0x55, 0x55, 0xff),
        NamedColor::BrightMagenta=>(0xff, 0x55, 0xff),
        NamedColor::BrightCyan  => (0x55, 0xff, 0xff),
        NamedColor::BrightWhite | NamedColor::BrightForeground => (0xff, 0xff, 0xff),
        NamedColor::DimBlack    => (0x0e, 0x0e, 0x0e),
        NamedColor::DimRed      => (0x88, 0x33, 0x33),
        NamedColor::DimGreen    => (0x33, 0x77, 0x33),
        NamedColor::DimYellow   => (0x77, 0x77, 0x33),
        NamedColor::DimBlue     => (0x33, 0x33, 0x88),
        NamedColor::DimMagenta  => (0x77, 0x33, 0x77),
        NamedColor::DimCyan     => (0x33, 0x77, 0x77),
        NamedColor::DimWhite    => (0x99, 0x97, 0x94),
        NamedColor::DimForeground => (0x9b, 0x98, 0x92),
        _ => (0xe6, 0xe4, 0xdf), // Cursor, Foreground (already handled), Background (ditto)
    };
    pack_rgb(r, g, b)
}

fn indexed_rgba(idx: u8) -> u32 {
    if idx < 16 {
        named_rgba(named_from_index(idx))
    } else if idx < 232 {
        let i = idx - 16;
        let b = cube_val(i % 6);
        let g = cube_val((i / 6) % 6);
        let r = cube_val(i / 36);
        pack_rgb(r, g, b)
    } else {
        let v = 8u8.saturating_add((idx - 232).saturating_mul(10));
        pack_rgb(v, v, v)
    }
}

fn cube_val(level: u8) -> u8 { if level == 0 { 0 } else { 55u8.saturating_add(level.saturating_mul(40)) } }

fn named_from_index(idx: u8) -> NamedColor {
    match idx {
        0  => NamedColor::Black,        1  => NamedColor::Red,
        2  => NamedColor::Green,        3  => NamedColor::Yellow,
        4  => NamedColor::Blue,         5  => NamedColor::Magenta,
        6  => NamedColor::Cyan,         7  => NamedColor::White,
        8  => NamedColor::BrightBlack,  9  => NamedColor::BrightRed,
        10 => NamedColor::BrightGreen,  11 => NamedColor::BrightYellow,
        12 => NamedColor::BrightBlue,   13 => NamedColor::BrightMagenta,
        14 => NamedColor::BrightCyan,   15 => NamedColor::BrightWhite,
        _  => NamedColor::Foreground,
    }
}

// ── span grouping + CSS ───────────────────────────────────────────────────────

/// Group adjacent same-style cells into `(text, css)` pairs.
pub(super) fn row_spans(row: &[(char, u32, u32, u16)]) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    let mut cur_text = String::new();
    let mut cur_style: Option<(u32, u32, u16)> = None;

    for &(ch, fg, bg, fl) in row {
        let style = (fg, bg, fl);
        if cur_style == Some(style) {
            cur_text.push(ch);
        } else {
            if let Some(s) = cur_style.take() {
                result.push((std::mem::take(&mut cur_text), span_css(s.0, s.1, s.2)));
            }
            cur_text.push(ch);
            cur_style = Some(style);
        }
    }
    if let Some(s) = cur_style {
        result.push((cur_text, span_css(s.0, s.1, s.2)));
    }

    // Trim trailing whitespace from the last span, drop empty spans.
    if let Some(last) = result.last_mut() {
        let trimmed = last.0.trim_end().to_string();
        last.0 = trimmed;
    }
    result.retain(|(t, _)| !t.is_empty());
    result
}

fn rgba_css(rgba: u32) -> Option<String> {
    if rgba == 0 { return None; }
    let r = (rgba >> 24) & 0xff;
    let g = (rgba >> 16) & 0xff;
    let b = (rgba >> 8)  & 0xff;
    Some(format!("#{r:02x}{g:02x}{b:02x}"))
}

fn span_css(fg: u32, bg: u32, flags: u16) -> String {
    let mut css = String::new();
    if let Some(c) = rgba_css(fg) { css.push_str(&format!("color:{c};")); }
    if let Some(c) = rgba_css(bg) { css.push_str(&format!("background:{c};")); }
    let f = Flags::from_bits_truncate(flags);
    if f.contains(Flags::BOLD)    { css.push_str("font-weight:bold;"); }
    if f.contains(Flags::ITALIC)  { css.push_str("font-style:italic;"); }
    if f.intersects(Flags::ALL_UNDERLINES) { css.push_str("text-decoration:underline;"); }
    css
}
