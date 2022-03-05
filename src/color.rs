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
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
enum ColorPrefix {
    Bg,
    Text,
}

#[derive(Debug)]
pub struct Color {
    hue: Hue,
    light: bool,
    prefix: ColorPrefix,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}-{}{}", self.prefix, self.hue, if self.light { "-lt" } else { "" })
    }
}

impl Color {
    pub fn bg(hue: Hue, light: bool) -> Self {
        Self {
            hue,
            light,
            prefix: ColorPrefix::Bg,
        }
    }

    pub fn text(hue: Hue) -> Self {
        Self {
            hue,
            light: false,
            prefix: ColorPrefix::Text,
        }
    }
}