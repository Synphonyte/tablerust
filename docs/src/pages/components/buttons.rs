#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::components::{Button, ButtonList, ButtonListAlign, ButtonColor};
use crate::code::Code;
use super::component_doc_page::*;

pub fn Buttons(cx: Scope) -> Element {
    cx.render( rsx!{
        ComponentDocPage {
            title: "Buttons",
            href: "https://preview.tabler.io/docs/buttons.html",
            description: "Use button styles that best suit your designs and encourage users to take the desired actions. You can customize the button's properties to improve the user experience of your website or system, changing the size, shape, color and many more.",
            h2 {
                id: "default-markup",
                "Default markup"
            }
            p {
                "As one of the most common elements of UI design, buttons have a very important function of engaging users with your website or app and guiding them in their actions. You can add additional styling that will make your buttons serve their purpose and draw users’ attention."
            }
        }
        Example {
            centered: true,
            ButtonList {
                Button { href: "#", "Link" }
                Button { "Button" }
                Button { to: "#", "RouterLink" }
            }
        }
        ExampleCode {
            "Button {{ href: \"#\", \"Link\" }}
Button {{ \"Button\" }}
Button {{ to: \"#\", \"RouterLink\" }}"
        }
        h2 {
            id: "default-button",
            "Default button"
        }
        p {
            "The standard button creates a white background and subtle hover animation. It’s meant to look and behave as an interactive element of your page."
        }
        Example {
            centered: true,
            Button { "Button" }
        }
        ExampleCode {
            "Button {{ \"Button\" }}"
        }
        h2 {
            id: "button-variations",
            "Button variations"
        }
        p {
            "Use the button classes that correspond to the function of your button. The big range of available colors will help you show your buttons’ purpose and make them easy to spot."
        }
        Example {
            centered: true,
            ButtonList {
                Button { color: ButtonColor::Primary, "Primary" }
                Button { color: ButtonColor::Secondary, "Secondary" }
                Button { color: ButtonColor::Success, "Success" }
                Button { color: ButtonColor::Warning, "Warning" }
                Button { color: ButtonColor::Danger, "Danger" }
                Button { color: ButtonColor::Info, "Info" }
                Button { color: ButtonColor::Light, "Light" }
                Button { color: ButtonColor::Dark, "Dark" }
            }
        }
        ExampleCode {
            "Button {{ color: ButtonColor::Primary, \"Primary\" }}
Button {{ color: ButtonColor::Secondary, \"Secondary\" }}
Button {{ color: ButtonColor::Success, \"Success\" }}
Button {{ color: ButtonColor::Warning, \"Warning\" }}
Button {{ color: ButtonColor::Danger, \"Danger\" }}
Button {{ color: ButtonColor::Info, \"Info\" }}
Button {{ color: ButtonColor::Light, \"Light\" }}
Button {{ color: ButtonColor::Dark, \"Dark\" }}"
        }
        h2 {
            id: "disabled-buttons",
            "Disabled buttons"
        }
        p {
            "Make buttons look inactive to show that an action is possible once the user meets certain criteria, such as completing the required fields to submit a form."
        }
        Example {
            centered: true,
            ButtonList {
                Button { color: ButtonColor::Primary, disabled: true, "Primary" }
                Button { color: ButtonColor::Secondary, disabled: true, "Secondary" }
                Button { color: ButtonColor::Success, disabled: true, "Success" }
                Button { color: ButtonColor::Warning, disabled: true, "Warning" }
                Button { color: ButtonColor::Danger, disabled: true, "Danger" }
                Button { color: ButtonColor::Info, disabled: true, "Info" }
                Button { color: ButtonColor::Light, disabled: true, "Light" }
                Button { color: ButtonColor::Dark, disabled: true, "Dark" }
            }
        }
        ExampleCode {
            "Button {{ color: ButtonColor::Primary, disabled: true, \"Primary\" }}
Button {{ color: ButtonColor::Secondary, disabled: true, \"Secondary\" }}
Button {{ color: ButtonColor::Success, disabled: true, \"Success\" }}
Button {{ color: ButtonColor::Warning, disabled: true, \"Warning\" }}
Button {{ color: ButtonColor::Danger, disabled: true, \"Danger\" }}
Button {{ color: ButtonColor::Info, disabled: true, \"Info\" }}
Button {{ color: ButtonColor::Light, disabled: true, \"Light\" }}
Button {{ color: ButtonColor::Dark, disabled: true, \"Dark\" }}"
        }
    })
}