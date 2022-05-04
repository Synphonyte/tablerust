#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::components::{Button, ButtonList, ButtonColor, ButtonFlair, ButtonShape, ButtonSize};
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
        h2 {
            id: "color-variations",
            "Color variations"
        }
        p {
            "Choose the right color for your button to make it go well with your design and draw users’ attention. Button colors can have a big influence on users’ decisions, which is why it’s important to choose them based on the intended purpose."
        }
        Example {
            centered: true,
            ButtonList {
                Button { color: ButtonColor::Blue, "Blue" }
                Button { color: ButtonColor::Azure, "Azure" }
                Button { color: ButtonColor::Indigo, "Indigo" }
                Button { color: ButtonColor::Purple, "Purple" }
                Button { color: ButtonColor::Pink, "Pink" }
                Button { color: ButtonColor::Red, "Red" }
                Button { color: ButtonColor::Orange, "Orange" }
                Button { color: ButtonColor::Yellow, "Yellow" }
                Button { color: ButtonColor::Lime, "Lime" }
                Button { color: ButtonColor::Green, "Green" }
                Button { color: ButtonColor::Teal, "Teal" }
                Button { color: ButtonColor::Cyan, "Cyan" }
            }
        }
        ExampleCode {
            "Button {{ color: ButtonColor::Blue, \"Blue\" }}
Button {{ color: ButtonColor::Azure, \"Azure\" }}
Button {{ color: ButtonColor::Indigo, \"Indigo\" }}
Button {{ color: ButtonColor::Purple, \"Purple\" }}
Button {{ color: ButtonColor::Pink, \"Pink\" }}
Button {{ color: ButtonColor::Red, \"Red\" }}
Button {{ color: ButtonColor::Orange, \"Orange\" }}
Button {{ color: ButtonColor::Yellow, \"Yellow\" }}
Button {{ color: ButtonColor::Lime, \"Lime\" }}
Button {{ color: ButtonColor::Green, \"Green\" }}
Button {{ color: ButtonColor::Teal, \"Teal\" }}
Button {{ color: ButtonColor::Cyan, \"Cyan\" }}"
        }
        h2 {
            id: "button-flair",
            "Button Flair"
        }
        p {
            "Use the "
            Code {
                "flair"
            }
            " property to make your button look simple yet aesthetically appealing. Ghost buttons help focus users’ attention on the website’s primary design, at the same time encouraging them to take action. While Outline buttons are perfect to use as secondary buttons, as they don’t distract users from the main action."
        }
        p {

        }
        Example {
            centered: true,
            ButtonList {
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Primary, "Primary" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Secondary, "Secondary" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Success, "Success" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Warning, "Warning" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Danger, "Danger" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Info, "Info" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Light, "Light" }
                Button { flair: ButtonFlair::Ghost, color: ButtonColor::Dark, "Dark" }
            }
        }
        ExampleCode {
            "Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Primary, \"Primary\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Secondary, \"Secondary\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Success, \"Success\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Warning, \"Warning\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Danger, \"Danger\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Info, \"Info\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Light, \"Light\" }}
Button {{ flair: ButtonFlair::Ghost, color: ButtonColor::Dark, \"Dark\" }}"
        }
        Example {
            centered: true,
            ButtonList {
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Primary, "Primary" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Secondary, "Secondary" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Success, "Success" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Warning, "Warning" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Danger, "Danger" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Info, "Info" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Light, "Light" }
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Dark, "Dark" }
            }
        }
        ExampleCode {
            "Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Primary, \"Primary\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Secondary, \"Secondary\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Success, \"Success\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Warning, \"Warning\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Danger, \"Danger\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Info, \"Info\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Light, \"Light\" }}
Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Dark, \"Dark\" }}"
        }
        h2 {
            id: "button-shape",
            "Button Shape"
        }
        p {
            "Add "
            Code {
                "shape: ButtonShape::Square"
            }
            " to remove the border radius, if you want the corners of your button to be square rather than rounded. Or use "
            Code {
                "shape: ButtonShape::Pill"
            }
            " to make your button rounded and give it a modern and attractive look."
        }
        Example {
            centered: true,
            ButtonList {
                Button { shape: ButtonShape::Square, "White" }
                Button { shape: ButtonShape::Pill, "White" }
            }
        }
        ExampleCode {
             "Button {{ shape: ButtonShape::Square, \"White\" }}
Button {{ shape: ButtonShape::Square, \"White\" }}"
        }
        h2 {
            id: "button-size",
            "Button size"
        }
        p {
            "Add "
            Code {
                "size: ButtonSize::Sm"
            }
            " or "
            Code {
                "size: ButtonSize::Lg"
            }
            " to change the size of your button and differentiate those which should have primary focus from those of secondary importance. Adapt the button size to your design and encourage users to take actions."
        }
        Example {
            centered: true,
            ButtonList {
                Button { size: ButtonSize::Lg, color: ButtonColor::Primary, "Large Button" }
                Button { size: ButtonSize::Lg, "Large Button" }
            }
        }
        ExampleCode {
            "Button {{ size: ButtonSize::Lg, \"Large Button\" }}
Button {{ size: ButtonSize::Lg, \"Large Button\" }}"
        }
        Example {
            centered: true,
            ButtonList {
                Button { size: ButtonSize::Sm, color: ButtonColor::Primary, "Small Button" }
                Button { size: ButtonSize::Sm, "Small Button" }
            }
        }
        ExampleCode {
            "Button {{ size: ButtonSize::Sm, \"Small Button\" }}
Button {{ size: ButtonSize::Sm, \"Small Button\" }}"
        }
    })
}