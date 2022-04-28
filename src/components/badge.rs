#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::string::ToString;

use crate::color::Color;


// TODO : badge with link

#[derive(Props)]
pub struct BadgeProps<'a> {
    #[props(default)]
    class: &'a str,

    #[props(default)]
    href: &'a str,

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

    let class = format!("badge {color} {pill} {outline} {}", cx.props.class);

    if cx.props.href.is_empty() {
        cx.render(rsx! {
            span {
                class: "{class}",
                &cx.props.children
            }
            " "
        })
    } else {
        cx.render(rsx! {
            a {
                class: "{class}",
                href: "{cx.props.href}",
                &cx.props.children
            }
            " "
        })
    }
}