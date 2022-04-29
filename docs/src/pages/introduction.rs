#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;

pub fn Introduction(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            div {
                class: "d-flex mb-3",
                h1 {
                    class: "m-0",
                    "Introduction"
                }
            }
        }
        p {
            "Tabler is a UI kit that speeds up the development process and makes it easier than ever! Built on the latest version of Bootstrap, it helps you create templates based on fully customizable and ready-to-use UI components, which can be used by both simple websites and sophisticated systems. With basic knowledge of HTML and CSS, you’ll be able to create dashboards that are fully functional and beautifully designed!"
        }
        div {
            class: "mt-4",
            div {
                class: "row",
                div {
                    class: "col-sm-6",
                    h3 { "Components" }
                    ul {
                        class: "list-unstyled",
                        li {
                            "- "
                            Link {
                                to: "/components/alerts",
                                "Alerts"
                            }
                        }
                        li {
                            "- "
                            Link {
                                to: "/components/avatars",
                                "Avatars"
                            }
                        }
                        li {
                            "- "
                            Link {
                                to: "/components/badges",
                                "Badges"
                            }
                        }
                        li {
                            "- "
                            Link {
                                to: "/components/breadcrumbs",
                                "Breadcrumbs"
                            }
                        }
                    }
                }
            }
        }
    })
}