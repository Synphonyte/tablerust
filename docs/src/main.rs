mod pages;

use pages::*;

use tablerust::dioxus::{
    prelude::*,
    router::{Route, Router, Link},
};

fn main() {
    tablerust::dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            div {
                class: "page-wrapper",
                div {
                    class: "container-xl",
                    div {
                        class: "page-header d-print-none",
                        div {
                            class: "row g-2 align-items-center",
                            div {
                                class: "col",
                                h2 {
                                    class: "page-title",
                                    "Documentation"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "page-body",
                    div {
                        class: "container-xl",
                        div {
                            class: "row gx-lg-5",
                            div {
                                class: "d-none d-lg-block col-lg-3",
                                ul {
                                    class: "nav nav-pills nav-vertical",
                                    li {
                                        class: "nav-item",
                                        Link {
                                            to: "/",
                                            class: "nav-link",
                                            "Introduction"
                                        }
                                    }
                                    li {
                                        class: "nav-item",
                                        a { // TODO : open when route starts with "components"
                                            class: "nav-link",
                                            href: "#menu-components",
                                            "data-bs-toggle": "collapse",
                                            "aria-expanded": "true",
                                            "Components"
                                            span { class: "nav-link-toggle" }
                                        }
                                        ul {
                                            id: "menu-components",
                                            class: "nav nav-pills collapse show",
                                            li {
                                                class: "nav-item",
                                                Link {
                                                    to: "/components/alerts",
                                                    class: "nav-link",
                                                    "Alerts"
                                                }
                                            }
                                            li {
                                                class: "nav-item",
                                                Link {
                                                    to: "/components/avatars",
                                                    class: "nav-link",
                                                    "Avatars"
                                                }
                                            }
                                            li {
                                                class: "nav-item",
                                                Link {
                                                    to: "/components/badges",
                                                    class: "nav-link",
                                                    "Badges"
                                                }
                                            }
                                            li {
                                                class: "nav-item",
                                                Link {
                                                    to: "/components/breadcrumbs",
                                                    class: "nav-link",
                                                    "Breadcrumbs"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                class: "col-lg-9",
                                div {
                                    class: "card card-lg", // TODO : use component
                                    div {
                                        class: "card-body",
                                        div {
                                            class: "markdown",
                                            Route { to: "/", Introduction { } }
                                            Route { to: "/components/alerts", components::Alerts { } }
                                            Route { to: "/components/avatars", components::Avatars { } }
                                            Route { to: "/components/badges", components::Badges { } }
                                            Route { to: "/components/breadcrumbs", components::Breadcrumbs { } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}