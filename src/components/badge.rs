#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::string::ToString;
use strum_macros::Display;
use crate::color::Color;
use crate::enums::{Shape, Size};

// TODO : badge with link

#[derive(Props)]
pub struct BadgeProps<'a> {
    #[props(default)]
    class: &'a str,

    #[props(default)]
    pill: bool,

    #[props(default)]
    outline: bool,

    color: Option<Color>,

    children: Element<'a>,
}

pub fn Badge<'a>(cx: Scope<'a, BadgeProps<'a>>) -> Element<'a> {
    let color = match &cx.props.color {
        Some(c) => c.to_string_with_prefix(if cx.props.outline { "text" } else { "bg" }),
        None => "".to_string()
    };

    let pill = if cx.props.pill { "badge-pill" } else { "" };
    let outline = if cx.props.outline { "badge-outline" } else { "" };

    cx.render(rsx! {
        span {
            class: "badge {color} {pill} {outline} {cx.props.class}",
            &cx.props.children
        }
    })
}