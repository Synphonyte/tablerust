#![allow(non_snake_case)]

use dioxus::prelude::*;





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
            prevent_default: "onclick",
            "aria-label": "close",
            onclick: move |evt| {
                evt.cancel_bubble();
                cx.props.onclick.call(())
            },
        }
    })
}