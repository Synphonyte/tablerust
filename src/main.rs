mod components;
mod enums;
mod color;

use dioxus::prelude::*;
use components::*;
use color::*;
use enums::*;

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
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

            Alert {
                alert_type: AlertType::Danger,
                icon_or_avatar: cx.render(rsx!{
                    Avatar { class: "me-3", "NE" }
                }),
                AlertTitle {
                    "Error in the matrix"
                }
                div {
                    class: "text-muted",
                    "The same cat has been seen multiple times. Dejavu!"
                }
            }

            div {
                AvatarList {
                    Avatar { image_url: "/img/avatars/000m.jpg" }
                    Avatar { "MS" }
                    Avatar { image_url: "/img/avatars/001m.jpg" }
                    Avatar { image_url: "/img/avatars/002m.jpg" }
                    Avatar { image_url: "/img/avatars/003m.jpg" }
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
                    Avatar { color: Color::bg(Hue::Cyan, true), "MS" }
                    Avatar { color: Color::bg(Hue::Green, true), "MS" }
                    Avatar { color: Color::bg(Hue::Pink, true), "MS" }
                    Avatar { color: Color::bg(Hue::Blue, true), shape: Shape::RoundedCircle "MS" }
                }
            }
        }
    })
}