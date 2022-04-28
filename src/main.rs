#![allow(non_snake_case)]

mod components;
mod enums;
mod color;
mod icon;

use dioxus::prelude::*;
use components::*;
use color::*;
use enums::*;
use icon::*;

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    let alert_visible = use_state(&cx, || true);

    cx.render(rsx! {
        div {
            style: "padding: 50px",
            Alert {
                alert_type: AlertType::Success,
                AlertTitle {
                    "Bla success"
                }
                div {
                    class: "text-muted",
                    "Something went right for once"
                }
            }

            Alert {
                alert_type: AlertType::Warning,
                important: true,
                div {
                    "Oh no! You're in danger"
                }
            }

            alert_visible.then(|| rsx!{
                Alert {
                    alert_type: AlertType::Danger,
                    icon_or_avatar: cx.render(rsx!{
                        Avatar { class: "me-3", "NE" }
                    }),
                    dismissible: true,
                    ondismiss: |_| alert_visible.set(false),
                    AlertTitle {
                        "Error in the matrix"
                    }
                    div {
                        class: "text-muted",
                        "The same cat has been seen multiple times. Dejavu!"
                    }
                }
            })

            div {
                AvatarList {
                    Avatar { image_url: "/img/avatars/000m.jpg" }
                    Avatar { "MS" }
                    Avatar { image_url: "/img/avatars/001m.jpg" }
                    Avatar { image_url: "/img/avatars/002m.jpg" }
                    Avatar {
                        image_url: "/img/avatars/003m.jpg",
                        Badge { color: BadgeColor::base(BadgeHue::Success), "" }
                    }
                }
            }

            div {
                AvatarList {
                    stacked: true,
                    Avatar { image_url: "/img/avatars/000m.jpg" }
                    Avatar { image_url: "/img/avatars/001m.jpg" }
                    Avatar { image_url: "/img/avatars/002m.jpg" }
                    Avatar { image_url: "/img/avatars/003m.jpg" }
                    Avatar { "5+" }
                }
            }

            div {
                AvatarList {
                    Avatar { color: AvatarColor::light(AvatarHue::Cyan), "MS" }
                    Avatar { color: AvatarColor::light(AvatarHue::Green),"MS"}
                    Avatar { color: AvatarColor::light(AvatarHue::Pink), size: Size::Md "MS" }
                    Avatar { color: AvatarColor::light(AvatarHue::Blue), shape: Shape::RoundedCircle "MS" }
                }
            }

            div {
                Badge { color: Color::base(Hue::Blue), "Blue" }
                Badge { color: Color::base(Hue::Azure), "Azure" }
                Badge { color: Color::base(Hue::Indigo), "Indigo" }
                Badge { color: Color::base(Hue::Purple), "Purple" }
                Badge { color: Color::base(Hue::Pink), "Pink" }
                Badge { color: Color::base(Hue::Red), "Red" }
                Badge { color: Color::base(Hue::Orange), "Orange" }
                Badge { color: Color::base(Hue::Yellow), "Yellow" }
                Badge { color: Color::light(Hue::Lime), "Lime" }
                Badge { color: Color::base(Hue::Green), pill: true, "Green" }
                Badge { color: Color::base(Hue::Teal), outline: true, "Teal" }
                Badge { color: Color::base(Hue::Cyan), "Cyan" }
            }

            div {
                Breadcrumb {
                    separator: BreadcrumbSeparator::Dot,
                    BreadcrumbItem { a { href: "#", "Home" } }
                    BreadcrumbItem { a { href: "#", "Library" } }
                    BreadcrumbItem { a { href: "#", "Data" } }
                    BreadcrumbItem { a { href: "#", "File" } }
                    BreadcrumbItem { active: true, a { href: "#", "This & That" } }
                }
            }
        }
    })
}