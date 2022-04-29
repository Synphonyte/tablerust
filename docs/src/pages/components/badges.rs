#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::components::{Badge, BadgeColor, BadgeHue};
use super::component_doc_page::*;

pub fn Badges(cx: Scope) -> Element {
    cx.render( rsx!{
        ComponentDocPage {
            title: "Badges",
            href: "https://preview.tabler.io/docs/badges.html",
            description: "Badges are small count and labeling components, which are used to add extra information to an interface element. You can use them to draw users' attention to a new element, notify about unread messages or provide any kind of additional info.",
            h2 {
                id: "default-markup",
                "Default markup"
            }
            p {
                "The default badges are square and come in the basic set of colors."
            }
        }
        Example {
            centered: true,
            Badge { color: BadgeColor::base(BadgeHue::Blue), "Blue" }
            Badge { color: BadgeColor::base(BadgeHue::Azure), "Azure" }
            Badge { color: BadgeColor::base(BadgeHue::Indigo), "Indigo" }
            Badge { color: BadgeColor::base(BadgeHue::Purple), "Purple" }
            Badge { color: BadgeColor::base(BadgeHue::Pink), "Pink" }
            Badge { color: BadgeColor::base(BadgeHue::Red), "Red" }
            Badge { color: BadgeColor::base(BadgeHue::Orange), "Orange" }
            Badge { color: BadgeColor::base(BadgeHue::Yellow), "Yellow" }
            Badge { color: BadgeColor::base(BadgeHue::Lime), "Lime" }
            Badge { color: BadgeColor::base(BadgeHue::Green), "Green" }
            Badge { color: BadgeColor::base(BadgeHue::Teal), "Teal" }
            Badge { color: BadgeColor::base(BadgeHue::Cyan), "Cyan" }
        }
        ExampleCode {
            "Badge {{ color: BadgeColor::base(BadgeHue::Blue), \"Blue\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Azure), \"Azure\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Indigo, \"Indigo\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Purple), \"Purple\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Pink), \"Pink\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Red), \"Red\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Orange), \"Orange\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Yellow), \"Yellow\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Lime), \"Lime\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Green), \"Green\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Teal), \"Teal\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Cyan), \"Cyan\" }}"
        }
        h2 {
            id: "headings",
            "Headings"
        }
        Example {
            h1 {
                "Example heading ",
                Badge { color: BadgeColor::base(BadgeHue::Gray), "New" }
            }
            h2 {
                "Example heading ",
                Badge { color: BadgeColor::base(BadgeHue::Gray), "New" }
            }
            h3 {
                "Example heading ",
                Badge { color: BadgeColor::base(BadgeHue::Gray), "New" }
            }
            h4 {
                "Example heading ",
                Badge { color: BadgeColor::base(BadgeHue::Gray), "New" }
            }
            h5 {
                "Example heading ",
                Badge { color: BadgeColor::base(BadgeHue::Gray), "New" }
            }
            h6 {
                "Example heading ",
                Badge { color: BadgeColor::base(BadgeHue::Gray), "New" }
            }
        }
        ExampleCode {
                "h1 {{
    \"Example heading \",
    Badge {{ color: BadgeColor::base(BadgeHue::Gray), \"New\" }}
}}
h2 {{
    \"Example heading \",
    Badge {{ color: BadgeColor::base(BadgeHue::Gray), \"New\" }}
}}
h3 {{
    \"Example heading \",
    Badge {{ color: BadgeColor::base(BadgeHue::Gray), \"New\" }}
}}
h4 {{
    \"Example heading \",
    Badge {{ color: BadgeColor::base(BadgeHue::Gray), \"New\" }}
}}
h5 {{
    \"Example heading \",
    Badge {{ color: BadgeColor::base(BadgeHue::Gray), \"New\" }}
}}
h6 {{
    \"Example heading \",
    Badge {{ color: BadgeColor::base(BadgeHue::Gray), \"New\" }}
}}"
        }
        h2 {
            id: "outline-badges",
            "Outline badges"
        }
        Example {
            centered: true,
            Badge { color: BadgeColor::base(BadgeHue::Blue), outline: true, "Blue" }
            Badge { color: BadgeColor::base(BadgeHue::Azure), outline: true, "Azure" }
            Badge { color: BadgeColor::base(BadgeHue::Indigo), outline: true, "Indigo" }
            Badge { color: BadgeColor::base(BadgeHue::Purple), outline: true, "Purple" }
            Badge { color: BadgeColor::base(BadgeHue::Pink), outline: true, "Pink" }
            Badge { color: BadgeColor::base(BadgeHue::Red), outline: true, "Red" }
            Badge { color: BadgeColor::base(BadgeHue::Orange), outline: true, "Orange" }
            Badge { color: BadgeColor::base(BadgeHue::Yellow), outline: true, "Yellow" }
            Badge { color: BadgeColor::base(BadgeHue::Lime), outline: true, "Lime" }
            Badge { color: BadgeColor::base(BadgeHue::Green), outline: true, "Green" }
            Badge { color: BadgeColor::base(BadgeHue::Teal), outline: true, "Teal" }
            Badge { color: BadgeColor::base(BadgeHue::Cyan), outline: true, "Cyan" }
        }
        ExampleCode {
            "Badge {{ color: BadgeColor::base(BadgeHue::Blue), \"Blue\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Azure), outline: true, \"Azure\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Indigo, outline: true, \"Indigo\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Purple), outline: true, \"Purple\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Pink), outline: true, \"Pink\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Red), outline: true, \"Red\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Orange), outline: true, \"Orange\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Yellow), outline: true, \"Yellow\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Lime), outline: true, \"Lime\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Green), outline: true, \"Green\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Teal), outline: true, \"Teal\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Cyan), outline: true, \"Cyan\" }}"
        }
        h2 {
            id: "pill-badges",
            "Pill badges"
        }
        p {
            "Add the 'pill' property if you want to create a badge with rounded corners. Its width will adjust to the label text."
        }
        Example {
            centered: true,
            Badge { color: BadgeColor::base(BadgeHue::Blue), pill: true, "1" }
            Badge { color: BadgeColor::base(BadgeHue::Azure), pill: true, "2" }
            Badge { color: BadgeColor::base(BadgeHue::Indigo), pill: true, "3" }
            Badge { color: BadgeColor::base(BadgeHue::Purple), pill: true, "4" }
            Badge { color: BadgeColor::base(BadgeHue::Pink), pill: true, "5" }
            Badge { color: BadgeColor::base(BadgeHue::Red), pill: true, "6" }
            Badge { color: BadgeColor::base(BadgeHue::Orange), pill: true, "7" }
            Badge { color: BadgeColor::base(BadgeHue::Yellow), pill: true, "8" }
            Badge { color: BadgeColor::base(BadgeHue::Lime), pill: true, "9" }
            Badge { color: BadgeColor::base(BadgeHue::Green), pill: true, "10" }
            Badge { color: BadgeColor::base(BadgeHue::Teal), pill: true, "11" }
            Badge { color: BadgeColor::base(BadgeHue::Cyan), pill: true, "12" }
        }
        ExampleCode {
            "Badge {{ color: BadgeColor::base(BadgeHue::Blue), \"1\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Azure), pill: true, \"2\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Indigo, pill: true, \"3\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Purple), pill: true, \"4\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Pink), pill: true, \"5\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Red), pill: true, \"6\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Orange), pill: true, \"7\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Yellow), pill: true, \"8\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Lime), pill: true, \"9\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Green), pill: true, \"10\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Teal), pill: true, \"11\" }}
Badge {{ color: BadgeColor::base(BadgeHue::Cyan), pill: true, \"12\" }}"
        }
        h2 {
            id: "soft-color-badges",
            "Soft color badges"
        }
        p {
            "You can create a soft colour variant of a corresponding contextual badge variation, to make it look more subtle. Click "
            a {
                href: "#","here" // Todo: link to colors
            }
            " to see the list of available colors and choose ones that best suit your design."
        }
            Example {
            centered: true,
            Badge { color: BadgeColor::light(BadgeHue::Blue), "Blue" }
            Badge { color: BadgeColor::light(BadgeHue::Azure), "Azure" }
            Badge { color: BadgeColor::light(BadgeHue::Indigo), "Indigo" }
            Badge { color: BadgeColor::light(BadgeHue::Purple), "Purple" }
            Badge { color: BadgeColor::light(BadgeHue::Pink), "Pink" }
            Badge { color: BadgeColor::light(BadgeHue::Red), "Red" }
            Badge { color: BadgeColor::light(BadgeHue::Orange), "Orange" }
            Badge { color: BadgeColor::light(BadgeHue::Yellow), "Yellow" }
            Badge { color: BadgeColor::light(BadgeHue::Lime), "Lime" }
            Badge { color: BadgeColor::light(BadgeHue::Green), "Green" }
            Badge { color: BadgeColor::light(BadgeHue::Teal), "Teal" }
            Badge { color: BadgeColor::light(BadgeHue::Cyan), "Cyan" }
        }
        ExampleCode {
            "Badge {{ color: BadgeColor::light(BadgeHue::Blue), \"Blue\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Azure), \"Azure\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Indigo, \"Indigo\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Purple), \"Purple\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Pink), \"Pink\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Red), \"Red\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Orange), \"Orange\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Yellow), \"Yellow\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Lime), \"Lime\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Green), \"Green\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Teal), \"Teal\" }}
Badge {{ color: BadgeColor::light(BadgeHue::Cyan), \"Cyan\" }}"
        }
        h2 {
            id: "links",
            "Links"
        }
        p {
            "Add the 'href' property if you want it to perform the function of a link and make it clickable."
        }
        Example {
              centered: true,
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Blue), "Blue" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Azure), "Azure" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Indigo), "Indigo" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Purple), "Purple" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Pink), "Pink" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Red), "Red" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Orange), "Orange" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Yellow), "Yellow" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Lime), "Lime" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Green), "Green" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Teal), "Teal" }
              Badge { href: "#", color: BadgeColor::base(BadgeHue::Cyan), "Cyan" }
        }
        ExampleCode {
            "Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Blue), \"Blue\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Azure), \"Azure\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Indigo), \"Indigo\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Purple), \"Purple\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Pink), \"Pink\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Red), \"Red\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Orange), \"Orange\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Yellow), \"Yellow\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Lime), \"Lime\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Green), \"Green\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Teal), \"Teal\" }}
Badge {{ href: \"#\", color: BadgeColor::base(BadgeHue::Cyan), \"Cyan\" }}"
        }
        h2 {
            id: "button-with-badge",
            "Button with badge"
        }
        p {
            "Badges can be used as part of links or buttons to provide a counter."
        }
        // TODO : move button-with-badge to buttons
    })
}
