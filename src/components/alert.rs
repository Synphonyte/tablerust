use dioxus::prelude::*;
use std::string::ToString;
use strum_macros::Display;

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
    // TODO : dismissable
}

pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element<'a> {
    let children = match &cx.props.icon_or_avatar {
        Some(_) => rsx! {
            div {
                class: "d-flex",
                div {
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

    cx.render(rsx! {
        div {
            class: "alert alert-{cx.props.alert_type} {extra_class}",
            role: "alert",
            children
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