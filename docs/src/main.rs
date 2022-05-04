#![allow(non_snake_case)]

mod pages;
mod code;

use pages::*;

use tablerust::dioxus::{
    prelude::*,
    router::{Route, Router, Link},
};

use tablerust::{
    icon::*,
    components::*,
    enums::*,

};

fn main() {
    tablerust::dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            header {
                class: "navbar navbar-expand-md navbar-light d-print-none",
                div {
                    class: "container-xl",
                    // TODO : navabar toggler
                    h1 {
                        class: "navbar-brand navbar-brand-autodark d-none-navbar-horizontal pe-0 pe-md-3",
                        Link {
                            to: "/",
                            img {
                                class: "navbar-brand-image",
                                src: "/img/logo.svg",
                                alt: "Tablerust",
                                height: "32",
                            }
                        }
                    }
                    div {
                        class: "navbar-nav flex-row order-md-last",
                        div {
                            class: "nav-item d-none d-md-flex me-3",
                            ButtonList {
                                Button {
                                    href: "https://github.com/Synphonyte/tablerust",
                                    rel: "noreferrer",
                                    target: "_blank",
                                    Icon {icon: "brand-github"}
                                    "Source code"
                                }
                                Button {
                                    href: "https://github.com/sponsors/Synphonyte",
                                    rel: "noreferrer",
                                    target: "_blank",
                                    Icon {icon: "heart", color: IconColor::Red}
                                    "Sponsor"
                                }
                            }
                        }
                        div {
                            class: "d-none d-md-flex",
                            a {
                                class: "nav-link px-0 hide-theme-dark",
                                "data-bs-placement": "bottom",
                                "data-bs-toggle": "tooltip",
                                href: "?theme=dark",
                                title: "Enable dark mode",
                                Icon {
                                    icon: "moon",
                                }
                            }
                            a {
                                class: "nav-link px-0 hide-theme-light",
                                "data-bs-placement": "bottom",
                                "data-bs-toggle": "tooltip",
                                href: "?theme=light",
                                title: "Enable light mode",
                                Icon {
                                    icon: "sun",
                                }
                            }
                        }
                        // TODO : notifcation dropdown
                        div {
                            class: "nav-item dropdown d-none d-md-flex me-3",
                            a {
                                class: "nav-link px-0",
                                href: "#",
                                "aria-label": "Show notifications",
                                Icon {
                                    icon: "bell",
                                }
                                Badge {
                                    color: BadgeColor::base(BadgeHue::Red),
                                    ""
                                }
                            }
                        }
                        // TODO : profile dropdown
                        div {
                            class: "nav-item dropdown",
                            a {
                                class: "nav-link d-flex lh-1 text-reset p-0",
                                href: "#",
                                Avatar {
                                    image_url: "/img/avatars/000m.jpg",
                                    size: Size::Sm,
                                }
                                div {
                                    class: "d-none d-xl-block px-2",
                                    div {
                                        "Paweł Kuna"
                                    }
                                    div {
                                        class: "mt-1 small text-muted",
                                        "UI Designer"
                                    }
                                }
                            }
                        }
                    }
                }
            }
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
                                            li {
                                                class: "nav-item",
                                                Link {
                                                    to: "/components/buttons",
                                                    class: "nav-link",
                                                    "Buttons"
                                                }
                                            }
                                            li {
                                                class: "nav-item",
                                                Link {
                                                    to: "/components/cards",
                                                    class: "nav-link",
                                                    "Cards"
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
                                            Route { to: "/components/buttons", components::Buttons { } }
                                            Route { to: "/components/cards", components::Cards { } }
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