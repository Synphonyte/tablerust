#![allow(non_snake_case)]

use dioxus::prelude::*;
use strum::Display;

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum CardPadding {
    Sm,
    Md,
    Lg,
}

#[derive(Props)]
pub struct CardProps<'a> {
    padding: Option<CardPadding>,

    children: Element<'a>,
}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element<'a> {
    let padding = cx.props.padding.as_ref().map(|p| format_args!(" card-{p}").to_string()).unwrap_or("".to_owned());

    cx.render(rsx! {
        div {
            class: "card{padding}",
            &cx.props.children,
        }
    })
}

#[inline_props]
pub fn CardBody<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "card-body",
            &cx.props.children,
        }
    })
}


#[inline_props]
pub fn CardHeader<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "card-header",
            &cx.props.children,
        }
    })
}

#[inline_props]
pub fn CardTitle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        h3 {
            class: "card-title",
            &cx.props.children,
        }
    })
}

#[inline_props]
pub fn CardRow<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "row row-cards",
            &cx.props.children,
        }
    })
}

// TODO
