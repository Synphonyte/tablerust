use dioxus::prelude::*;
use std::string::ToString;
use strum_macros::Display;
use crate::color::Color;
use crate::enums::{Shape, Size};

// TODO : badge with link

#[derive(Props)]
pub struct CloseButtonProps<'a> {
    #[props(default)]
    class: &'a str,

    pub onclick: EventHandler<'a>,
}

pub fn CloseButton<'a>(cx: Scope<'a, CloseButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        a {
            class: "btn-close {cx.props.class}",
            "aria-label": "close",
            onclick: move |evt| {
                evt.cancel_bubble();
                cx.props.onclick.call(())
            },
            ""
        }
    })
}