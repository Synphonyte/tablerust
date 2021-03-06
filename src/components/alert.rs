#![allow(non_snake_case)]

use dioxus::prelude::*;
use strum_macros::Display;
use crate::components::CloseButton;

#[derive(Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum AlertType {
    Success,
    Info,
    Warning,
    Danger,
}

#[derive(Props)]
pub struct AlertProps<'a> {
    alert_type: AlertType,

    children: Element<'a>,

    #[props(default)]
    icon_or_avatar: Element<'a>,

    #[props(default)]
    important: bool,

    #[props(default)]
    dismissible: bool,

    #[props(default)]
    pub ondismiss: EventHandler<'a>,
}

pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element<'a> {
    let children = match &cx.props.icon_or_avatar {
        Some(_) => rsx! {
            div {
                class: "d-flex",
                div {
                    class: "alert-icon-or-avatar",
                    &cx.props.icon_or_avatar
                }
                div {
                    &cx.props.children
                }
            }
        },
        None => rsx! {
            &cx.props.children,
        }
    };

    let extra_class = if cx.props.important {
        "alert-important"
    } else {
        ""
    };

    let (close_button, dismissible) = if cx.props.dismissible {
        (
            cx.render(rsx! {
                CloseButton {
                    onclick: move |_| cx.props.ondismiss.call(()),
                    class: if cx.props.important { "btn-close-white" } else { "" }
                }
            }),
            "alert-dismissible"
        )
    } else {
        (None, "")
    };

    cx.render(rsx! {
        div {
            class: "alert alert-{cx.props.alert_type} {extra_class} {dismissible}",
            role: "alert",
            children
            close_button
        }
    })
}

#[derive(Props)]
pub struct AlertTitleProps<'a> {
    children: Element<'a>,
}

pub fn AlertTitle<'a>(cx: Scope<'a, AlertTitleProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        h4 {
            class: "alert-title",
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct AlertLinkProps<'a> {
    children: Element<'a>,
}

pub fn AlertLink<'a>(cx: Scope<'a, AlertLinkProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        a {
            class: "alert-link",
            &cx.props.children
        }
    })
}