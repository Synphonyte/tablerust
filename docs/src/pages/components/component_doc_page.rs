#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::enums::*;
use tablerust::icon::*;
use tablerust::color::*;

#[derive(Props)]
pub struct ComponentDocPageProps<'a> {
    title: &'a str,
    href: &'a str,
    description: &'a str,

    children: Element<'a>,
}

pub fn ComponentDocPage<'a>(cx: Scope<'a, ComponentDocPageProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            div {
                class: "d-flex mb-3",
                h1 {
                    class: "m-0",
                    "{cx.props.title}",
                }
                p {
                    class: "ms-auto",
                    a {
                        class: "d-flex align-items-center",
                        href: "{cx.props.href}",
                        target: "_blank",
                        Icon {
                            icon: "external-link",
                            color: Color::base(Hue::Blue),
                        }
                        "Tabler documentation"
                    }
                }
            }
            p {
                class: "mb-4 text-muted",
                "{cx.props.description}"
            }
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct ExampleProps<'a> {
    #[props(default)]
    centered: bool,

    children: Element<'a>,
}

pub fn Example<'a>(cx: Scope<'a, ExampleProps<'a>>) -> Element<'a> {
    let centered = if cx.props.centered { "example-centered" } else { "" };

    cx.render(rsx! {
        div {
            class: "example no_toc_section {centered}",
            div {
                class: "example-content",
                &cx.props.children
            }
        }
    })
}

#[derive(Props)]
pub struct ExampleCodeProps<'a> {
    children: Element<'a>,
}

pub fn ExampleCode<'a>(cx: Scope<'a, ExampleCodeProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "example-code",
            figure {
                class: "highlight",
                pre {
                    code {
                        class: "language-rust",
                        "data-lang": "rust",
                        // TODO : syntax highlighting
                        &cx.props.children
                    }
                }
            }
        }
    })
}