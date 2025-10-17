use crate::config::CONFIG;
use crate::log_level::Levels;
use crate::time::Time;
use colored::{Color, ColoredString, Colorize};
use std::str::FromStr;

impl From<Levels> for ColoredString {
    fn from(level: Levels) -> ColoredString {
        match level {
            Levels::Debug => Levels::Debug.to_string().color(Levels::Debug).into(),
            Levels::Info => Levels::Info.to_string().color(Levels::Info).into(),
            Levels::Warning => Levels::Warning.to_string().color(Levels::Warning).into(),
            Levels::Error => Levels::Error.to_string().color(Levels::Error).bold().into(),
            Levels::Fatal => Levels::Fatal.to_string().color(Levels::Fatal).bold().into(),
        }
    }
}

impl From<Levels> for Color {
    fn from(level: Levels) -> Color {
        match level {
            Levels::Error => {
                Color::from_str(&CONFIG.error).unwrap_or_else(|_| hex_to_rgb(&CONFIG.error))
            }
            Levels::Info => {
                Color::from_str(&CONFIG.info).unwrap_or_else(|_| hex_to_rgb(&CONFIG.info))
            }
            Levels::Debug => {
                Color::from_str(&CONFIG.debug).unwrap_or_else(|_| hex_to_rgb(&CONFIG.debug))
            }
            Levels::Fatal => {
                Color::from_str(&CONFIG.fatal).unwrap_or_else(|_| hex_to_rgb(&CONFIG.fatal))
            }
            Levels::Warning => {
                Color::from_str(&CONFIG.warning).unwrap_or_else(|_| hex_to_rgb(&CONFIG.warning))
            }
        }
    }
}

impl From<Time> for ColoredString {
    fn from(value: Time) -> Self {
        let color = Color::from_str(&CONFIG.time).unwrap_or_else(|_| hex_to_rgb(&CONFIG.time));

        value.to_string().color(color).into()
    }
}

pub fn color_message(message: &str) -> ColoredString {
    let color = Color::from_str(&CONFIG.message).unwrap_or_else(|_| hex_to_rgb(&CONFIG.message));

    message.to_string().color(color)
}

fn hex_to_rgb(s: &str) -> Color {
    let hex = s.trim_start_matches('#');

    if hex.len() != 6 {
        return Color::White;
    }

    let r_str = &hex[0..2];
    let g_str = &hex[2..4];
    let b_str = &hex[4..6];

    let r = match u8::from_str_radix(r_str, 16) {
        Ok(val) => val,
        Err(_) => return Color::White,
    };
    let g = match u8::from_str_radix(g_str, 16) {
        Ok(val) => val,
        Err(_) => return Color::White,
    };
    let b = match u8::from_str_radix(b_str, 16) {
        Ok(val) => val,
        Err(_) => return Color::White,
    };

    Color::TrueColor { r, g, b }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FFFFFF"), color(255, 255, 255));
        assert_eq!(hex_to_rgb("#000000"), color(0, 0, 0));
        assert_eq!(hex_to_rgb("#32a852"), color(50, 168, 82));
        assert_eq!(hex_to_rgb("#4287f5"), color(66, 135, 245));
        assert_eq!(hex_to_rgb("#fcba03"), color(252, 186, 3));
        assert_eq!(hex_to_rgb("#505f94"), color(80, 95, 148));
        assert_eq!(hex_to_rgb("#4f0f45"), color(79, 15, 69));
        assert_eq!(hex_to_rgb("#2b272b"), color(43, 39, 43));
        assert_eq!(hex_to_rgb("#0f9638"), color(15, 150, 56));
        assert_eq!(hex_to_rgb("#6b4b3e"), color(107, 75, 62));
        assert_eq!(hex_to_rgb("#030b1f"), color(3, 11, 31));
        assert_eq!(hex_to_rgb("#3f61b5"), color(63, 97, 181));

        assert_eq!(hex_to_rgb("FFFFFF"), color(255, 255, 255));
        assert_eq!(hex_to_rgb("000000"), color(0, 0, 0));
        assert_eq!(hex_to_rgb("32a852"), color(50, 168, 82));
        assert_eq!(hex_to_rgb("4287f5"), color(66, 135, 245));
        assert_eq!(hex_to_rgb("fcba03"), color(252, 186, 3));
        assert_eq!(hex_to_rgb("505f94"), color(80, 95, 148));
        assert_eq!(hex_to_rgb("4f0f45"), color(79, 15, 69));
        assert_eq!(hex_to_rgb("2b272b"), color(43, 39, 43));
        assert_eq!(hex_to_rgb("0f9638"), color(15, 150, 56));
        assert_eq!(hex_to_rgb("6b4b3e"), color(107, 75, 62));
        assert_eq!(hex_to_rgb("030b1f"), color(3, 11, 31));
        assert_eq!(hex_to_rgb("3f61b5"), color(63, 97, 181));

        assert_eq!(hex_to_rgb("aboba"), Color::White);
        assert_eq!(hex_to_rgb("alksjfdlksdj"), Color::White);
        assert_eq!(hex_to_rgb("#fffffff"), Color::White);
    }

    fn color(r: u8, g: u8, b: u8) -> Color {
        Color::TrueColor { r, g, b }
    }
}
