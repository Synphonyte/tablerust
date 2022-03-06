use std::fmt::{Display, Formatter};
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Hue {
    Blue,
    Azure,
    Indigo,
    Purple,
    Pink,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Teal,
    Cyan,
    Danger,
    Success,
    Warning,
    Info,
    Gray
}

#[derive(Debug)]
pub struct Color {
    hue: Hue,
    light: bool,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.hue, if self.light { "-lt" } else { "" })
    }
}

impl Color {
    pub fn base(hue: Hue) -> Self {
        Self {
            hue,
            light: false,
        }
    }

    pub fn light(hue: Hue) -> Self {
        Self {
            hue,
            light: true,
        }
    }

    pub fn to_string_with_prefix(&self, prefix: &str) -> String {
        format!("{prefix}-{self}")
    }
}

// #[derive(Display, Debug)]
// #[strum(serialize_all = "kebab_case")]
// enum ColorType {
//     Bg,
//     Text,
// }
//
// #[derive(Debug)]
// pub struct ColorWithType {
//     color_type: ColorType,
//     color: Color,
// }
//
// impl Display for ColorWithType {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{}-{}", self.color_type, self.color)
//     }
// }
//
// impl ColorWithType {
//     pub fn text(hue: Hue) -> Self {
//         Self {
//             color: Color::base(hue),
//             color_type: ColorType::Text
//         }
//     }
//
//     pub fn bg(hue: Hue) -> Self {
//         Self {
//             color: Color::base(hue),
//             color_type: ColorType::Bg
//         }
//     }
//
//     pub fn bg_lt(hue: Hue) -> Self {
//         Self {
//             color: Color::light(hue),
//             color_type: ColorType::Bg
//         }
//     }
//
// }
