#![allow(non_snake_case)]

use dioxus::prelude::*;
use strum::Display;
use dioxus::events::MouseEvent;

// TODO: Dropdown

#[derive(Props)]
pub struct ButtonProps<'a> {
    color: Option<ButtonColor>,

    #[props(default)]
    disabled: bool,

    flair: Option<ButtonFlair>,
    shape: Option<ButtonShape>,
    size: Option<ButtonSize>,

    #[props(default)]
    icon_only: bool,

    #[props(default)]
    loading: bool,

    // TODO: if both are provided make a frog that hops all over the screen
    href: Option<&'a str>,
    to: Option<&'a str>,

    rel: Option<&'a str>,
    target: Option<&'a str>,

    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,

    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let color = match cx.props.color {
        Some(ref c) => c.string_with_flair(&cx.props.flair),
        None => "".to_owned(),
    };

    let disabled = if cx.props.disabled { " disabled" } else { "" };

    let shape = match cx.props.shape {
        Some(ref s) => format_args!(" btn-{}", s),
        None => "".to_owned(),
    };

    let size = match cx.props.size {
        Some(ref s) => format_args!(" btn-{}", s),
        None => "".to_owned(),
    };

    let loading = if cx.props.loading { " btn-loading" } else { "" };

    let icon_only = if cx.props.icon_only { " btn-icon" } else { "" };

    let class = format_args!("btn{color}{disabled}{shape}{size}{loading}{icon_only}");

    match cx.props.to {
        Some(to) => cx.render(rsx! {
            Link {
                class: "{class}",
                to: to,
                &cx.props.children,
            }
        }),
        None => match cx.props.href {

            Some(href) => {
                let rel = cx.props.rel.unwrap_or("");
                let target = cx.props.target.unwrap_or("");

                cx.render(rsx! {
                    a {
                        class: "{class}",
                        href: "{href}",
                        rel: "{rel}",
                        target: "{target}",
                        &cx.props.children,
                    }
                })
            },
            None => cx.render(rsx! {
                button {
                    class: "{class}",
                    onclick:  move |evt| cx.props.onclick.call(evt),
                    &cx.props.children,
                }
            }),
        }
    }
}

#[derive(Props)]
pub struct ButtonListProps<'a> {
    children: Element<'a>,
    align: Option<ButtonListAlign>,
}

pub fn ButtonList<'a>(cx: Scope<'a, ButtonListProps<'a>>) -> Element<'a> {
    let align = match cx.props.align {
        Some(ref a) => format_args!(" justify-content-{a}"),
        None => "".to_owned(),
    };

    cx.render(rsx! {
        div {
            class: "btn-list{align}",
            &cx.props.children,
        }
    })
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ButtonListAlign {
    Start,
    Center,
    End,
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ButtonSize {
    Lg,
    Sm,
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ButtonShape {
    Square,
    Pill,
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ButtonFlair {
    Outline,
    Ghost,
}

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ButtonColor {
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

    Light,
    Dark,

    // semantic
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,

    // social
    Facebook,
    Twitter,
    Google,
    Youtube,
    Vimeo,
    Dribbble,
    Github,
    Instagram,
    Pinterest,
    Vk,
    Rss,
    Flickr,
    Bitbucket,
    Tabler,
}

impl ButtonColor {
    fn string_with_flair(&self, flair: &Option<ButtonFlair>) -> String {
        let flair = match flair {
            Some(flair) => format_args!("-{}", flair.to_string()),
            None => "".into(),
        };

        format_args!(" btn{flair}-{self}")
    }
}