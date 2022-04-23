#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};
use strum_macros::Display;
use std::string::ToString;
use dioxus::prelude::*;

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum BreadcrumbSeparator {
    Slash,
    Dot,
    Arrow,
    Bullet,
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        BreadcrumbSeparator::Slash
    }
}

#[derive(Props)]
pub struct BreadcrumbProps<'a> {
    #[props(default)]
    separator: BreadcrumbSeparator,

    children: Element<'a>,
}

pub fn Breadcrumb<'a>(cx: Scope<'a, BreadcrumbProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        ol {
            class: "breadcrumb breadcrumb-{cx.props.separator}s",
            "aria-label": "breadcrumbs",
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct BreadcrumbItemProps<'a> {
    #[props(default)]
    active: bool,

    children: Element<'a>,
}

pub fn BreadcrumbItem<'a>(cx: Scope<'a, BreadcrumbItemProps<'a>>) -> Element<'a> {
    let (active, aria) = if cx.props.active { ("active", "page") } else { ("", "false") };

    cx.render(rsx! {
        li {
            class: "breadcrumb-item {active}",
            "aria-current": "{aria}",
            &cx.props.children
        }
    })
}