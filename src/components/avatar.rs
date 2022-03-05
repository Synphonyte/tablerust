use dioxus::prelude::*;
use std::string::ToString;
use strum_macros::Display;
use crate::color::Color;
use crate::enums::{Shape, Size};


#[derive(Props)]
pub struct AvatarProps<'a> {
    #[props(default)]
    class: &'a str,

    #[props(default, strip_option)]
    shape: Option<Shape>,

    #[props(default, strip_option)]
    size: Option<Size>,

    #[props(default, strip_option)]
    color: Option<Color>,

    #[props(default, strip_option)]
    image_url: Option<&'a str>,

    children: Element<'a>,
}

pub fn Avatar<'a>(cx: Scope<'a, AvatarProps<'a>>) -> Element<'a> {

    let style = match &cx.props.image_url {
        Some(url) => format!("background-image: url({url});"),
        None => "".to_string()
    };

    let color = match &cx.props.color {
        Some(c) => c.to_string(),
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