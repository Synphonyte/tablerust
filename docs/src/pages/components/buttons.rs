#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::components::{Button, ButtonList, ButtonListAlign, ButtonColor, ButtonFlair, ButtonShape, ButtonSize};
use tablerust::icon::*;
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
        h2 {
            id: "buttons-with-icons",
            "Buttons with icons"
        }
        p {
            "Label your button with text and add an icon to communicate the action and make it easy to identify for users. Icons are easily recognized and improve the aesthetics of your button design, giving it a modern and attractive look."
        }
        // p { // Todo: maybe
        //     "Icons can be found"
        //     a {
        //         href: "/docs/icons.html#icons",strong {
        //             "here"
        //         }
        //     }
        // }
        Example {
            centered: true,
            ButtonList {
                Button {
                    Icon { icon: "upload" },
                    "Upload"
                }
                Button {
                    color: ButtonColor::Warning,
                    Icon { icon: "heart" },
                    "I like"
                }
                Button {
                    color: ButtonColor::Success,
                    Icon { icon: "check" },
                    "I agree"
                }
                Button {
                    color: ButtonColor::Primary,
                    Icon { icon: "plus" },
                    "More"
                }
                Button {
                    color: ButtonColor::Danger,
                    Icon { icon: "link" },
                    "Link"
                }
                Button {
                    color: ButtonColor::Info,
                    Icon { icon: "message-circle" },
                    "Comment"
                }
            }
        }
        ExampleCode {
            "Button {{
    Icon {{ icon: \"upload\" }},
    \"Upload\"
}}
Button {{
    color: ButtonColor::Warning,
    Icon {{ icon: \"heart\" }},
    \"I like\"
}}
Button {{
    color: ButtonColor::Success,
    Icon {{ icon: \"check\" }},
    \"I agree\"
}}
Button {{
    color: ButtonColor::Primary,
    Icon {{ icon: \"plus\" }},
    \"More\"
}}
Button {{
    color: ButtonColor::Danger,
    Icon {{ icon: \"link\" }},
    \"Link\"
}}
Button {{
    color: ButtonColor::Info,
    Icon {{ icon: \"message-circle\" }},
    \"Comment\"
}}"
        }
        h2 {
            id: "social-buttons",
            "Social buttons"
        }
        p {
            "You can use the icons of popular social networking sites, which users are familiar with. Thanks to buttons with social media icons users can share content or follow a website with just one click, without leaving the website."
        }
        Example {
            ButtonList {
                Button {
                    color: ButtonColor::Facebook,
                    Icon { icon: "brand-facebook" },
                    "Facebook"
                }
                Button {
                    color: ButtonColor::Twitter,
                    Icon { icon: "brand-twitter" },
                    "Twitter"
                }
                Button {
                    color: ButtonColor::Google,
                    Icon { icon: "brand-google" },
                    "Google"
                }
                Button {
                    color: ButtonColor::Youtube,
                    Icon { icon: "brand-youtube" },
                    "Youtube"
                }
                Button {
                    color: ButtonColor::Vimeo,
                    Icon { icon: "brand-vimeo" },
                    "Vimeo"
                }
                Button {
                    color: ButtonColor::Dribbble,
                    Icon { icon: "brand-dribbble" },
                    "Dribbble"
                }
                Button {
                    color: ButtonColor::Github,
                    Icon { icon: "brand-github" },
                    "Github"
                }
                Button {
                    color: ButtonColor::Instagram,
                    Icon { icon: "brand-instagram" },
                    "Instagram"
                }
                Button {
                    color: ButtonColor::Pinterest,
                    Icon { icon: "brand-pinterest" },
                    "Pinterest"
                }
                Button {
                    color: ButtonColor::Vk,
                    Icon { icon: "brand-vk" },
                    "Vk"
                }
                Button {
                    color: ButtonColor::Rss,
                    Icon { icon: "rss" },
                    "Rss"
                }
                Button {
                    color: ButtonColor::Flickr,
                    Icon { icon: "brand-flickr" },
                    "Flickr"
                }
                Button {
                    color: ButtonColor::Bitbucket,
                    Icon { icon: "brand-bitbucket" },
                    "Bitbucket"
                }
                Button {
                    color: ButtonColor::Tabler,
                    Icon { icon: "brand-tabler" },
                    "Tabler"
                }
            }
        }
        ExampleCode {
            "Button {{
    color: ButtonColor::Facebook,
    Icon {{ icon: \"brand-facebook\" }},
    \"Facebook\"
}}
Button {{
    color: ButtonColor::Twitter,
    Icon {{ icon: \"brand-twitter\" }},
    \"Twitter\"
}}
Button {{
    color: ButtonColor::Google,
    Icon {{ icon: \"brand-google\" }},
    \"Google\"
}}
Button {{
    color: ButtonColor::Youtube,
    Icon {{ icon: \"brand-youtube\" }},
    \"Youtube\"
}}
Button {{
    color: ButtonColor::Vimeo,
    Icon {{ icon: \"brand-vimeo\" }},
    \"Vimeo\"
}}
Button {{
    color: ButtonColor::Dribbble,
    Icon {{ icon: \"brand-dribbble\" }},
    \"Dribbble\"
}}
Button {{
    color: ButtonColor::Github,
    Icon {{ icon: \"brand-github\" }},
    \"Github\"
}}
Button {{
    color: ButtonColor::Instagram,
    Icon {{ icon: \"brand-instagram\" }},
    \"Instagram\"
}}
Button {{
    color: ButtonColor::Pinterest,
    Icon {{ icon: \"brand-pinterest\" }},
    \"Pinterest\"
}}
Button {{
    color: ButtonColor::Vk,
    Icon {{ icon: \"brand-vk\" }},
    \"Vk\"
}}
Button {{
    color: ButtonColor::Rss,
    Icon {{ icon: \"rss\" }},
    \"Rss\"
}}
Button {{
    color: ButtonColor::Flickr,
    Icon {{ icon: \"brand-flickr\" }},
    \"Flickr\"
}}
Button {{
    color: ButtonColor::Bitbucket,
    Icon {{ icon: \"brand-bitbucket\" }},
    \"Bitbucket\"
}}
Button {{
    color: ButtonColor::Tabler,
    Icon {{ icon: \"brand-tabler\" }},
    \"Tabler\"
}}"
        }
        p {
            "You can also add an icon without the name of a social networking site, if you want to display more buttons on a small space. Just add the "
            Code {
                "icon_only"
            }
            " property instead of a name."
        }
        Example {
            centered: true,
            ButtonList {
                Button {
                    color: ButtonColor::Facebook,
                    Icon { icon: "brand-facebook" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Twitter,
                    Icon { icon: "brand-twitter" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Google,
                    Icon { icon: "brand-google" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Youtube,
                    Icon { icon: "brand-youtube" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Vimeo,
                    Icon { icon: "brand-vimeo" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Dribbble,
                    Icon { icon: "brand-dribbble" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Github,
                    Icon { icon: "brand-github" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Instagram,
                    Icon { icon: "brand-instagram" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Pinterest,
                    Icon { icon: "brand-pinterest" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Vk,
                    Icon { icon: "brand-vk" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Rss,
                    Icon { icon: "rss" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Flickr,
                    Icon { icon: "brand-flickr" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Bitbucket,
                    Icon { icon: "brand-bitbucket" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Tabler,
                    Icon { icon: "brand-tabler" },
                    icon_only: true
                }
            }
        }
        ExampleCode {
            "Button {{
    color: ButtonColor::Facebook,
    Icon {{ icon: \"brand-facebook\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Twitter,
    Icon {{ icon: \"brand-twitter\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Google,
    Icon {{ icon: \"brand-google\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Youtube,
    Icon {{ icon: \"brand-youtube\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Vimeo,
    Icon {{ icon: \"brand-vimeo\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Dribbble,
    Icon {{ icon: \"brand-dribbble\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Github,
    Icon {{ icon: \"brand-github\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Instagram,
    Icon {{ icon: \"brand-instagram\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Pinterest,
    Icon {{ icon: \"brand-pinterest\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Vk,
    Icon {{ icon: \"brand-vk\" }},
    \"Vk\"
}}
Button {{
    color: ButtonColor::Rss,
    Icon {{ icon: \"rss\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Flickr,
    Icon {{ icon: \"brand-flickr\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Bitbucket,
    Icon {{ icon: \"brand-bitbucket\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Tabler,
    Icon {{ icon: \"brand-tabler\" }},
    icon_only: true
}}"
        }
        h2 {
            id: "icon-buttons",
            "Icon buttons"
        }
        p {
            "Add the "
            Code     {
                "icon_only"
            }
            " property to remove unnecessary padding from your button and use an icon without any additional label. Thanks to that, you can save space and make the action easy to recognize for international users."
        }
        Example {
            centered: true,
            ButtonList {
                Button {
                    color: ButtonColor::Primary,
                    Icon { icon: "activity" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Github,
                    Icon { icon: "brand-github" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Success,
                    Icon { icon: "bell" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Warning,
                    Icon { icon: "star" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Danger,
                    Icon { icon: "trash" },
                    icon_only: true
                }
                Button {
                    color: ButtonColor::Purple,
                    Icon { icon: "chart-bar" },
                    icon_only: true
                }
                Button {
                    Icon { icon: "git-merge" },
                    icon_only: true
                }
            }
        }
        ExampleCode {
            "Button {{
    color: ButtonColor::Primary,
    Icon {{ icon: \"activity\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Github,
    Icon {{ icon: \"brand-github\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Success,
    Icon {{ icon: \"bell\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Warning,
    Icon {{ icon: \"star\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Danger,
    Icon {{ icon: \"trash\" }},
    icon_only: true
}}
Button {{
    color: ButtonColor::Purple,
    Icon {{ icon: \"chart-bar\" }},
    icon_only: true
}}
Button {{
    Icon {{ icon: \"git-merge\" }},
    icon_only: true
}}"
        }
        // Todo: https://preview.tabler.io/docs/buttons.html#dropdown-buttons
        h2 {
            id: "loading-buttons",
            "Loading buttons"
        }
        p {
            "Add the "
            Code {
                "loading"
            }
            " property to show a button’s loading state, which can be useful in the case of operations that take longer to process. Thanks to that, users will be aware of the current state of their action and won’t give it up before it’s finished."
        }
        Example {
            centered: true,
            ButtonList {
                Button { 
                    loading: true, 
                    color: ButtonColor::Primary,
                    "Button"
                }
                Button { 
                    loading: true, 
                    color: ButtonColor::Primary,
                    "Loading button with loooong content" 
                }
            }
        }
        ExampleCode {
            "Button {{ 
    loading: true,
    color: ButtonColor::Primary,
    \"Button\"
}}
Button {{
    loading: true,
    color: ButtonColor::Primary,
    \"Loading button with loooong content\"
}}"
        }
        // Todo: Label next to loading circle https://preview.tabler.io/docs/buttons.html#loading-buttons
        h2 {
            id: "list-of-buttons",
            "List of buttons"
        }
        p {
            "Create a list of buttons using the "
            Code {
                "ButtonList"
            }
            " component to display different actions a user can take. If you add additional styling, such as colours, you will be able to focus users’ attention on a particular action or suggest the result."
        }
        Example {
            ButtonList {
                Button { color: ButtonColor::Success, "Save changes" }
                Button { "Save and continue" }
                Button { color: ButtonColor::Danger, "Cancel" }
            }
        }
        ExampleCode {
            "ButtonList {{
    Button {{ color: ButtonColor::Success, \"Save changes\" }}
    Button {{ \"Save and continue\" }}
    Button {{ color: ButtonColor::Danger, \"Cancel\" }}
}}"
        }
        p {
            "If the list is long, it will be wrapped and some buttons will be moved to the next line, keeping them all evenly spaced."
        }
        Example {
            ButtonList {
                Button { "One" }
                Button { "Two" }
                Button { "Three" }
                Button { "Four" }
                Button { "Five" }
                Button { "Six" }
                Button { "Seven" }
                Button { "Eight" }
                Button { "Nine" }
                Button { "Ten" }
                Button { "Eleven" }
                Button { "Twelve" }
                Button { "Thirteen" }
                Button { "Fourteen" }
                Button { "Fifteen" }
                Button { "Sixteen" }
                Button { "Seventeen" }
                Button { "Eighteen" }
                Button { "Nineteen" }
            }
        }
        ExampleCode {
            "ButtonList {{
    Button {{ \"One\" }}
    Button {{ \"Two\" }}
    Button {{ \"Three\" }}
    Button {{ \"Four\" }}
    Button {{ \"Five\" }}
    Button {{ \"Six\" }}
    Button {{ \"Seven\" }}
    Button {{ \"Eight\" }}
    Button {{ \"Nine\" }}
    Button {{ \"Ten\" }}
    Button {{ \"Eleven\" }}
    Button {{ \"Twelve\" }}
    Button {{ \"Thirteen\" }}
    Button {{ \"Fourteen\" }}
    Button {{ \"Fifteen\" }}
    Button {{ \"Sixteen\" }}
    Button {{ \"Seventeen\" }}
    Button {{ \"Eighteen\" }}
    Button {{ \"Nineteen\" }}
}}"
        }
        p {
            "Use the align property with "
            Code { "ButtonListAlign::Start" }
            " , "
            Code { "ButtonListAlign::Center" }
            " or the "
            Code { "ButtonListAlign::End" }
            " modifiers to change the buttons’ alignment and place them where they suit best."
        }
        Example {
            ButtonList {
                align: ButtonListAlign::Center,
                Button { "Save and continue" }
                Button { color: ButtonColor::Primary, "Save changes" }
            }
        }
        ExampleCode {
            "ButtonList {{
    align: ButtonListAlign::Center,
    Button {{ \"Save and continue\" }}
    Button {{ color: ButtonColor::Primary, \"Save changes\" }}
}}"
        }
        Example {
            ButtonList {
                align: ButtonListAlign::End,
                Button { "Save and continue" }
                Button { color: ButtonColor::Primary, "Save changes" }
            }
        }
        ExampleCode {
            "ButtonList {{
    align: ButtonListAlign::End,
    Button {{ \"Save and continue\" }}
    Button {{ color: ButtonColor::Primary, \"Save changes\" }}
}}"
        }
        Example {
            ButtonList {
                align: ButtonListAlign::Start,
                Button { flair: ButtonFlair::Outline, color: ButtonColor::Danger, "Delete" }
                Button { "Save and continue" }
                Button { color: ButtonColor::Primary, "Save changes" }
            }
        }
        ExampleCode {
            "ButtonList {{
    align: ButtonListAlign::Start,
    Button {{ flair: ButtonFlair::Outline, color: ButtonColor::Danger, \"Save changes\" }}
    Button {{ \"Save and continue\" }}
    Button {{ color: ButtonColor::Primary, \"Delete\" }}
}}"
        }
        // Todo? por qué button with avatar?? https://preview.tabler.io/docs/buttons.html#buttons-with-avatars
    })
}