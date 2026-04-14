use alacritty_terminal::term::color::Colors;
use alacritty_terminal::vte::ansi::{Color, NamedColor};

/// Resolve a terminal `Color` to a packed RGBA `u32` using the terminal's live palette.
///
/// Returns `0` for the default foreground/background sentinel so that the CSS layer can
/// apply the configured theme color without baking it into every cell.
pub(super) fn color_rgba_with_palette(color: Color, palette: &Colors) -> u32 {
    match color {
        Color::Named(NamedColor::Foreground | NamedColor::Background) => {
            // Let the CSS default color handle these so the theme is respected.
            0
        }
        Color::Named(named) => {
            // Prefer OSC 4/10/11 palette override, fall back to built-in table.
            if let Some(rgb) = palette[named as usize] {
                pack_rgb(rgb.r, rgb.g, rgb.b)
            } else {
                named_rgba(named)
            }
        }
        Color::Indexed(index) => {
            if let Some(rgb) = palette[index as usize] {
                pack_rgb(rgb.r, rgb.g, rgb.b)
            } else {
                indexed_rgba(index)
            }
        }
        Color::Spec(rgb) => pack_rgb(rgb.r, rgb.g, rgb.b),
    }
}

fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xff
}

fn named_rgba(named: NamedColor) -> u32 {
    let (r, g, b) = match named {
        NamedColor::Black => (0x1c, 0x1c, 0x1c),
        NamedColor::Red => (0xcc, 0x55, 0x55),
        NamedColor::Green => (0x55, 0xaa, 0x55),
        NamedColor::Yellow => (0xaa, 0xaa, 0x55),
        NamedColor::Blue => (0x55, 0x55, 0xcc),
        NamedColor::Magenta => (0xaa, 0x55, 0xaa),
        NamedColor::Cyan => (0x55, 0xaa, 0xaa),
        NamedColor::White => (0xe6, 0xe4, 0xdf),
        NamedColor::BrightBlack => (0x55, 0x55, 0x55),
        NamedColor::BrightRed => (0xff, 0x55, 0x55),
        NamedColor::BrightGreen => (0x55, 0xff, 0x55),
        NamedColor::BrightYellow => (0xff, 0xff, 0x55),
        NamedColor::BrightBlue => (0x55, 0x55, 0xff),
        NamedColor::BrightMagenta => (0xff, 0x55, 0xff),
        NamedColor::BrightCyan => (0x55, 0xff, 0xff),
        NamedColor::BrightWhite | NamedColor::BrightForeground => (0xff, 0xff, 0xff),
        NamedColor::DimBlack => (0x0e, 0x0e, 0x0e),
        NamedColor::DimRed => (0x88, 0x33, 0x33),
        NamedColor::DimGreen => (0x33, 0x77, 0x33),
        NamedColor::DimYellow => (0x77, 0x77, 0x33),
        NamedColor::DimBlue => (0x33, 0x33, 0x88),
        NamedColor::DimMagenta => (0x77, 0x33, 0x77),
        NamedColor::DimCyan => (0x33, 0x77, 0x77),
        NamedColor::DimWhite => (0x99, 0x97, 0x94),
        NamedColor::DimForeground => (0x9b, 0x98, 0x92),
        _ => (0xe6, 0xe4, 0xdf),
    };
    pack_rgb(r, g, b)
}

fn indexed_rgba(index: u8) -> u32 {
    if index < 16 {
        named_rgba(named_from_index(index))
    } else if index < 232 {
        let scaled = index - 16;
        let blue = cube_val(scaled % 6);
        let green = cube_val((scaled / 6) % 6);
        let red = cube_val(scaled / 36);
        pack_rgb(red, green, blue)
    } else {
        let value = 8u8.saturating_add((index - 232).saturating_mul(10));
        pack_rgb(value, value, value)
    }
}

fn cube_val(level: u8) -> u8 {
    if level == 0 {
        0
    } else {
        55u8.saturating_add(level.saturating_mul(40))
    }
}

fn named_from_index(index: u8) -> NamedColor {
    match index {
        0 => NamedColor::Black,
        1 => NamedColor::Red,
        2 => NamedColor::Green,
        3 => NamedColor::Yellow,
        4 => NamedColor::Blue,
        5 => NamedColor::Magenta,
        6 => NamedColor::Cyan,
        7 => NamedColor::White,
        8 => NamedColor::BrightBlack,
        9 => NamedColor::BrightRed,
        10 => NamedColor::BrightGreen,
        11 => NamedColor::BrightYellow,
        12 => NamedColor::BrightBlue,
        13 => NamedColor::BrightMagenta,
        14 => NamedColor::BrightCyan,
        15 => NamedColor::BrightWhite,
        _ => NamedColor::Foreground,
    }
}
