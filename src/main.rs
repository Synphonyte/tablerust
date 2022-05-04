#![allow(non_snake_case)]

mod components;
mod enums;
mod icon;

use dioxus::prelude::*;
use components::*;
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
                Badge { color: BadgeColor::base(BadgeHue::Blue), "Blue" }
                Badge { color: BadgeColor::base(BadgeHue::Azure), "Azure" }
                Badge { color: BadgeColor::base(BadgeHue::Indigo), "Indigo" }
                Badge { color: BadgeColor::base(BadgeHue::Purple), "Purple" }
                Badge { color: BadgeColor::base(BadgeHue::Pink), "Pink" }
                Badge { color: BadgeColor::base(BadgeHue::Red), "Red" }
                Badge { color: BadgeColor::base(BadgeHue::Orange), "Orange" }
                Badge { color: BadgeColor::base(BadgeHue::Yellow), "Yellow" }
                Badge { color: BadgeColor::light(BadgeHue::Lime), "Lime" }
                Badge { color: BadgeColor::base(BadgeHue::Green), pill: true, "Green" }
                Badge { color: BadgeColor::base(BadgeHue::Teal), outline: true, "Teal" }
                Badge { color: BadgeColor::base(BadgeHue::Cyan), "Cyan" }
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

            div {
                ButtonList {
                    align: ButtonListAlign::Center,
                    Button { "Test" }
                    Button { color: ButtonColor::Primary, "Primary" }
                    Button { flair: ButtonFlair::Ghost, color: ButtonColor::Purple, "Ghost" }
                    Button { shape: ButtonShape::Pill, "Pill" }
                    Button { loading: true, color: ButtonColor::Primary, "Loading" }
                    Button { size: ButtonSize::Sm, disabled: true, "small" }
                    Button { href: "", "href" }
                    Button {
                        icon_only: true,
                        Icon { icon: "star" }
                    }
                    Button {
                        color: ButtonColor::Facebook,
                        Icon { icon: "brand-facebook" },
                        "Facebook"
                    }
                }

            }
        }
    })
}