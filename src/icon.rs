#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::color::Color;

#[derive(Props)]
pub struct IconProps<'a> {
    #[props(default)]
    class: &'a str,

    icon: &'a str,

    #[props(default, strip_option)]
    color: Option<Color>,
}

pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let color = match &cx.props.color {
        Some(c) => c.to_string_with_prefix("text"),
        None => "".to_string()
    };

    cx.render(rsx! {
        i {
            class: "ti ti-{cx.props.icon} {color} {cx.props.class}",
        }
    })
}