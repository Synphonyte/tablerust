#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::color;
use strum::Display;

#[derive(Props)]
pub struct IconProps<'a> {
    #[props(default)]
    class: &'a str,

    icon: &'a str,

    #[props(default, strip_option)]
    color: Option<IconColor>,
}

pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let color = match &cx.props.color {
        Some(c) => format!(" text-{c}"),
        None => "".to_string()
    };

    cx.render(rsx! {
        i {
            class: "ti ti-{cx.props.icon}{color} {cx.props.class}",
        }
    })
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum IconColor {
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
}