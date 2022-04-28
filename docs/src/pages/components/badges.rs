use tablerust::dioxus::prelude::*;
use tablerust::components::{Badge};
use tablerust::enums::*;
use tablerust::icon::*;
use tablerust::color::*;
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
            Badge { color: Color::base(Hue::Blue), "Blue" }
            Badge { color: Color::base(Hue::Azure), "Azure" }
            Badge { color: Color::base(Hue::Indigo), "Indigo" }
            Badge { color: Color::base(Hue::Purple), "Purple" }
            Badge { color: Color::base(Hue::Pink), "Pink" }
            Badge { color: Color::base(Hue::Red), "Red" }
            Badge { color: Color::base(Hue::Orange), "Orange" }
            Badge { color: Color::base(Hue::Yellow), "Yellow" }
            Badge { color: Color::base(Hue::Lime), "Lime" }
            Badge { color: Color::base(Hue::Green), "Green" }
            Badge { color: Color::base(Hue::Teal), "Teal" }
            Badge { color: Color::base(Hue::Cyan), "Cyan" }
        }
        ExampleCode {
            "Badge {{ color: Color::base(Hue::Blue), \"Blue\" }}
Badge {{ color: Color::base(Hue::Azure), \"Azure\" }}
Badge {{ color: Color::base(Hue::Indigo, \"Indigo\" }}
Badge {{ color: Color::base(Hue::Purple), \"Purple\" }}
Badge {{ color: Color::base(Hue::Pink), \"Pink\" }}
Badge {{ color: Color::base(Hue::Red), \"Red\" }}
Badge {{ color: Color::base(Hue::Orange), \"Orange\" }}
Badge {{ color: Color::base(Hue::Yellow), \"Yellow\" }}
Badge {{ color: Color::base(Hue::Lime), \"Lime\" }}
Badge {{ color: Color::base(Hue::Green), \"Green\" }}
Badge {{ color: Color::base(Hue::Teal), \"Teal\" }}
Badge {{ color: Color::base(Hue::Cyan), \"Cyan\" }}"
        }
        h2 {
            id: "headings",
            "Headings"
        }
        Example {
            h1 {
                "Example heading ",
                Badge { color: Color::base(Hue::Gray), "New" }
            }
            h2 {
                "Example heading ",
                Badge { color: Color::base(Hue::Gray), "New" }
            }
            h3 {
                "Example heading ",
                Badge { color: Color::base(Hue::Gray), "New" }
            }
            h4 {
                "Example heading ",
                Badge { color: Color::base(Hue::Gray), "New" }
            }
            h5 {
                "Example heading ",
                Badge { color: Color::base(Hue::Gray), "New" }
            }
            h6 {
                "Example heading ",
                Badge { color: Color::base(Hue::Gray), "New" }
            }
        }
        ExampleCode {
                "h1 {{
    \"Example heading \",
    Badge {{ color: Color::base(Hue::Gray), \"New\" }}
}}
h2 {{
    \"Example heading \",
    Badge {{ color: Color::base(Hue::Gray), \"New\" }}
}}
h3 {{
    \"Example heading \",
    Badge {{ color: Color::base(Hue::Gray), \"New\" }}
}}
h4 {{
    \"Example heading \",
    Badge {{ color: Color::base(Hue::Gray), \"New\" }}
}}
h5 {{
    \"Example heading \",
    Badge {{ color: Color::base(Hue::Gray), \"New\" }}
}}
h6 {{
    \"Example heading \",
    Badge {{ color: Color::base(Hue::Gray), \"New\" }}
}}"
        }
        h2 {
            id: "outline-badges",
            "Outline badges"
        }
        Example {
            centered: true,
            Badge { color: Color::base(Hue::Blue), outline: true, "Blue" }
            Badge { color: Color::base(Hue::Azure), outline: true, "Azure" }
            Badge { color: Color::base(Hue::Indigo), outline: true, "Indigo" }
            Badge { color: Color::base(Hue::Purple), outline: true, "Purple" }
            Badge { color: Color::base(Hue::Pink), outline: true, "Pink" }
            Badge { color: Color::base(Hue::Red), outline: true, "Red" }
            Badge { color: Color::base(Hue::Orange), outline: true, "Orange" }
            Badge { color: Color::base(Hue::Yellow), outline: true, "Yellow" }
            Badge { color: Color::base(Hue::Lime), outline: true, "Lime" }
            Badge { color: Color::base(Hue::Green), outline: true, "Green" }
            Badge { color: Color::base(Hue::Teal), outline: true, "Teal" }
            Badge { color: Color::base(Hue::Cyan), outline: true, "Cyan" }
        }
        ExampleCode {
            "Badge {{ color: Color::base(Hue::Blue), \"Blue\" }}
Badge {{ color: Color::base(Hue::Azure), outline: true, \"Azure\" }}
Badge {{ color: Color::base(Hue::Indigo, outline: true, \"Indigo\" }}
Badge {{ color: Color::base(Hue::Purple), outline: true, \"Purple\" }}
Badge {{ color: Color::base(Hue::Pink), outline: true, \"Pink\" }}
Badge {{ color: Color::base(Hue::Red), outline: true, \"Red\" }}
Badge {{ color: Color::base(Hue::Orange), outline: true, \"Orange\" }}
Badge {{ color: Color::base(Hue::Yellow), outline: true, \"Yellow\" }}
Badge {{ color: Color::base(Hue::Lime), outline: true, \"Lime\" }}
Badge {{ color: Color::base(Hue::Green), outline: true, \"Green\" }}
Badge {{ color: Color::base(Hue::Teal), outline: true, \"Teal\" }}
Badge {{ color: Color::base(Hue::Cyan), outline: true, \"Cyan\" }}"
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
            Badge { color: Color::base(Hue::Blue), pill: true, "1" }
            Badge { color: Color::base(Hue::Azure), pill: true, "2" }
            Badge { color: Color::base(Hue::Indigo), pill: true, "3" }
            Badge { color: Color::base(Hue::Purple), pill: true, "4" }
            Badge { color: Color::base(Hue::Pink), pill: true, "5" }
            Badge { color: Color::base(Hue::Red), pill: true, "6" }
            Badge { color: Color::base(Hue::Orange), pill: true, "7" }
            Badge { color: Color::base(Hue::Yellow), pill: true, "8" }
            Badge { color: Color::base(Hue::Lime), pill: true, "9" }
            Badge { color: Color::base(Hue::Green), pill: true, "10" }
            Badge { color: Color::base(Hue::Teal), pill: true, "11" }
            Badge { color: Color::base(Hue::Cyan), pill: true, "12" }
        }
        ExampleCode {
            "Badge {{ color: Color::base(Hue::Blue), \"1\" }}
Badge {{ color: Color::base(Hue::Azure), pill: true, \"2\" }}
Badge {{ color: Color::base(Hue::Indigo, pill: true, \"3\" }}
Badge {{ color: Color::base(Hue::Purple), pill: true, \"4\" }}
Badge {{ color: Color::base(Hue::Pink), pill: true, \"5\" }}
Badge {{ color: Color::base(Hue::Red), pill: true, \"6\" }}
Badge {{ color: Color::base(Hue::Orange), pill: true, \"7\" }}
Badge {{ color: Color::base(Hue::Yellow), pill: true, \"8\" }}
Badge {{ color: Color::base(Hue::Lime), pill: true, \"9\" }}
Badge {{ color: Color::base(Hue::Green), pill: true, \"10\" }}
Badge {{ color: Color::base(Hue::Teal), pill: true, \"11\" }}
Badge {{ color: Color::base(Hue::Cyan), pill: true, \"12\" }}"
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
            Badge { color: Color::light(Hue::Blue), "Blue" }
            Badge { color: Color::light(Hue::Azure), "Azure" }
            Badge { color: Color::light(Hue::Indigo), "Indigo" }
            Badge { color: Color::light(Hue::Purple), "Purple" }
            Badge { color: Color::light(Hue::Pink), "Pink" }
            Badge { color: Color::light(Hue::Red), "Red" }
            Badge { color: Color::light(Hue::Orange), "Orange" }
            Badge { color: Color::light(Hue::Yellow), "Yellow" }
            Badge { color: Color::light(Hue::Lime), "Lime" }
            Badge { color: Color::light(Hue::Green), "Green" }
            Badge { color: Color::light(Hue::Teal), "Teal" }
            Badge { color: Color::light(Hue::Cyan), "Cyan" }
        }
        ExampleCode {
            "Badge {{ color: Color::light(Hue::Blue), \"Blue\" }}
Badge {{ color: Color::light(Hue::Azure), \"Azure\" }}
Badge {{ color: Color::light(Hue::Indigo, \"Indigo\" }}
Badge {{ color: Color::light(Hue::Purple), \"Purple\" }}
Badge {{ color: Color::light(Hue::Pink), \"Pink\" }}
Badge {{ color: Color::light(Hue::Red), \"Red\" }}
Badge {{ color: Color::light(Hue::Orange), \"Orange\" }}
Badge {{ color: Color::light(Hue::Yellow), \"Yellow\" }}
Badge {{ color: Color::light(Hue::Lime), \"Lime\" }}
Badge {{ color: Color::light(Hue::Green), \"Green\" }}
Badge {{ color: Color::light(Hue::Teal), \"Teal\" }}
Badge {{ color: Color::light(Hue::Cyan), \"Cyan\" }}"
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
              Badge { href: "#", color: Color::base(Hue::Blue), "Blue" }
              Badge { href: "#", color: Color::base(Hue::Azure), "Azure" }
              Badge { href: "#", color: Color::base(Hue::Indigo), "Indigo" }
              Badge { href: "#", color: Color::base(Hue::Purple), "Purple" }
              Badge { href: "#", color: Color::base(Hue::Pink), "Pink" }
              Badge { href: "#", color: Color::base(Hue::Red), "Red" }
              Badge { href: "#", color: Color::base(Hue::Orange), "Orange" }
              Badge { href: "#", color: Color::base(Hue::Yellow), "Yellow" }
              Badge { href: "#", color: Color::base(Hue::Lime), "Lime" }
              Badge { href: "#", color: Color::base(Hue::Green), "Green" }
              Badge { href: "#", color: Color::base(Hue::Teal), "Teal" }
              Badge { href: "#", color: Color::base(Hue::Cyan), "Cyan" }
        }
        ExampleCode {
            "Badge {{ href: \"#\", color: Color::base(Hue::Blue), \"Blue\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Azure), \"Azure\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Indigo), \"Indigo\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Purple), \"Purple\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Pink), \"Pink\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Red), \"Red\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Orange), \"Orange\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Yellow), \"Yellow\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Lime), \"Lime\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Green), \"Green\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Teal), \"Teal\" }}
Badge {{ href: \"#\", color: Color::base(Hue::Cyan), \"Cyan\" }}"
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
