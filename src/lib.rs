mod colors;
mod style;

/// Simple ANSI ESCAPES CODES
pub use colors::Color;
pub use style::{AnsiStyle, Styled};

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

pub const fn ansi256(value: u8) -> Color {
    Color::Ansi256(value)
}

pub mod prelude {
    pub use crate::{ansi256, rgb, AnsiStyle, Color, Styled};
}