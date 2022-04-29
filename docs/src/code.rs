#![allow(non_snake_case)]

use dioxus::prelude::*;

// pub struct CodeProps<'a> {
//     children: Element<'a>
// }

#[inline_props]
pub fn Code<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx!{
        code {
            class: "language-plaintext highlighter-rouge",
            &cx.props.children
        }
    })
}