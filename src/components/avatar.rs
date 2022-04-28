#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::string::ToString;
use std::fmt::{Display, Formatter};
use crate::enums::{Shape, Size};
use strum_macros::Display;


#[derive(Props)]
pub struct AvatarProps<'a> {
    #[props(default)]
    class: &'a str,

    shape: Option<Shape>,
    size: Option<Size>,
    color: Option<AvatarColor>,
    image_url: Option<&'a str>,

    children: Element<'a>,
}

pub fn Avatar<'a>(cx: Scope<'a, AvatarProps<'a>>) -> Element<'a> {

    let style = match &cx.props.image_url {
        Some(url) => format!("background-image: url({url});"),
        None => "".to_string()
    };

    let color = match &cx.props.color {
        Some(c) => c.to_string_with_prefix("bg"),
        None => "".to_string()
    };

    let size = match &cx.props.size {
        Some(s) => format!("avatar-{s}"),
        None => "".to_string(),
    };

    let shape = match &cx.props.shape {
        Some(s) => s.to_string(),
        None => "".to_string(),
    };

    cx.render(rsx! {
        span {
            class: "avatar {color} {size} {shape} {cx.props.class}",
            style: "{style}",
            &cx.props.children
        }
        " "
    })
}

#[derive(Props)]
pub struct AvatarListProps<'a> {
    #[props(default)]
    stacked: bool,

    children: Element<'a>,
}

pub fn AvatarList<'a>(cx: Scope<'a, AvatarListProps<'a>>) -> Element<'a> {
    let stacked = if cx.props.stacked {
        "avatar-list-stacked"
    } else {
        ""
    };

    cx.render(rsx! {
         div {
            class: "avatar-list {stacked}",
            &cx.props.children
        }
    })
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum AvatarHue {
    Blue, // TODO: use macro for some hues?
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
    Gray,
    // semantic
    Danger,
    Success,
    Warning,
    Info,
}

#[derive(Debug)]
pub struct AvatarColor {
    hue: AvatarHue,
    light: bool,
}

impl Display for AvatarColor {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.hue, if self.light { "-lt" } else { "" })
    }
}

impl AvatarColor {
    pub fn base(hue: AvatarHue) -> Self {
        Self {
            hue,
            light: false,
        }
    }

    pub fn light(hue: AvatarHue) -> Self {
        Self {
            hue,
            light: true,
        }
    }

    pub fn to_string_with_prefix(&self, prefix: &str) -> String {
        format!("{prefix}-{self}")
    }
}