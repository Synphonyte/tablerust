#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::components::{Card};
use crate::code::Code;
use super::component_doc_page::*;

pub fn Cards(cx: Scope) -> Element {
    cx.render(rsx! {
        ComponentDocPage {
            title: "Cards",
            href: "https://preview.tabler.io/docs/cards.html",
            description: "Cards are flexible user interface elements, which help organize content into meaningful sections and make it easier to display on different screen sizes. Cards contain various smaller components, such as images, text, links and buttons and may act as an entry to more detailed information, helping users scan the page quickly and find the most relevant content.",
            h2 {
                id: "default-card",
                "Default card"
            }
            p {
                "Use the "
                Code { "Card" }
                " and "
                Code { "CardBody" }
                " components to create a card and use it as the basis for a more advanced card design. A card is a perfect way to organize content and make it look neat and tidy."
            }
        }
    })
}