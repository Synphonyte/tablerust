use tablerust::dioxus::prelude::*;
use tablerust::components::{Avatar, AvatarList, Badge};
use tablerust::enums::*;
use tablerust::icon::*;
use tablerust::color::*;
use super::component_doc_page::*;

pub fn Avatars(cx: Scope) -> Element {
    cx.render(rsx! {
        ComponentDocPage {
            title: "Avatars",
            href: "https://preview.tabler.io/docs/avatars.html",
            description: "Avatars help customise various elements of a user interface and make the product experience more personalised. They are often used in communication apps, collaboration tools and social media.",
            h2 {
                id: "default-markup",
                "Default markup"
            }
            p {
                "Use the"
                code {
                    class: "language-plaintext highlighter-rouge",
                    "avatar"
                }
                "class to add an avatar to your interface design for greater customisation."
            }
            Example {
                centered: true,
                Avatar {
                    image_url: "/img/avatars/000m.jpg",
                }
                Avatar {
                    "JL"
                }
                Avatar {
                    image_url: "/img/avatars/002m.jpg",
                }
            }
            ExampleCode {
                "Avatar {{ image_url: \"/img/avatars/000m.jpg\", }}
Avatar {{ \"JL\" }}
Avatar {{ image_url: \"/img/avatars/002m.jpg\", }}"
            }
            h2 {
                id: "avatar-icons",
                "Avatar icons"
            }
            p {
                "Apart from pictures and initials, you can also use icons to make the avatars more universal."
            }
            Example {
                centered: true,
                Avatar {
                    Icon {
                        icon: "user",
                    }
                }
                Avatar {
                    Icon {
                        icon: "plus",
                    }
                }
                Avatar {
                    Icon {
                        icon: "user-plus",
                    }
                }
            }
            ExampleCode {
                "Avatar {{
    Icon {{ icon: \"user\", }}
}}
Avatar {{
    Icon {{ icon: \"plus\", }}
}}
Avatar {{
    Icon {{ icon: \"user-plus\", }}
}}"
            }
            h2 {
                id: "avatar-initials-color",
                "Avatar initials color"
            }
            p {
                "Customize the color of the avatars’ background. You can click"
                a {
                    href: "./colors.html","here"
                }
                "to see the list of available colors."
            }
            Example {
                centered: true,
                Avatar {
                    color: Color::light(Hue::Green),
                    "AB"
                }
                Avatar {
                    color: Color::light(Hue::Red),
                    "CD"
                }
                Avatar {
                    color: Color::light(Hue::Yellow),
                    "EF"
                }
                Avatar {
                    color: Color::light(Hue::Blue),
                    "GH"
                }
                Avatar {
                    color: Color::light(Hue::Purple),
                    "IJ"
                }
            }
            ExampleCode {
                "Avatar {{ color: Color::light(Hue::Green), \"AB\" }}
Avatar {{ color: Color::light(Hue::Red), \"CD\" }}
Avatar {{ color: Color::light(Hue::Yellow), \"EF\" }}
Avatar {{ color: Color::light(Hue::Blue), \"GH\" }}
Avatar {{ color: Color::light(Hue::Purple), \"IJ\" }}"
            }
            h2 {
                id: "avatar-size",
                "Avatar size"
            }
            p {
                "Using Bootstrap’s typical naming structure, you can create a standard avatar or scale it up or down to different sizes based on what you need."
            }
            Example {
                centered: true,
                Avatar {
                    size: Size::Xl,
                    "HS"
                }
                Avatar {
                    size: Size::Lg,
                    image_url: "/img/avatars/003f.jpg",
                }
                Avatar {
                    size: Size::Md,
                    image_url: "/img/avatars/002f.jpg",
                }
                Avatar {
                    "EP"
                }
                Avatar {
                    size: Size::Sm,
                    image_url: "/img/avatars/001f.jpg",
                }
                Avatar {
                    size: Size::Xs,
                    image_url: "/img/avatars/000f.jpg",
                }
            }
            ExampleCode {
                "Avatar {{ size: Size::Xl, \"HS\" }}
Avatar {{ size: Size::Lg, image_url: \"/img/avatars/003f.jpg\", }}
Avatar {{ size: Size::Md, image_url: \"/img/avatars/002f.jpg\", }}
Avatar {{ \"EP\" }}
Avatar {{ size: Size::Sm, image_url: \"/img/avatars/001f.jpg\", }}
Avatar {{ size: Size::Xs, image_url: \"/img/avatars/000f.jpg\", }}"
            }
            h2 {
                id: "avatar-status",
                "Avatar status"
            }
            p {
                "Add a status indicator to your avatar to show, for instance, if a users is online or offline or indicate the number of messages they have received."
            }
            Example {
                centered: true,
                Avatar { image_url: "/img/avatars/006f.jpg", }
                Avatar { image_url: "/img/avatars/004f.jpg",
                    Badge { color: Color::base(Hue::Danger), "" }
                }
                Avatar { image_url: "/img/avatars/007m.jpg",
                    Badge { color: Color::base(Hue::Success), "" }
                }
                Avatar { "SA"
                    Badge { color: Color::base(Hue::Warning), "" }
                }
                Avatar { image_url: "/img/avatars/004f.jpg",
                    Badge { color: Color::base(Hue::Info), "" }
                }
                Avatar { image_url: "/img/avatars/004f.jpg",
                    Badge { color: Color::base(Hue::Gray), "5" }
                }
            }
            ExampleCode {
                "Avatar {{ image_url: \"/img/avatars/006mf.jpg\", }}
Avatar {{ image_url: \"/img/avatars/004f.jpg\",
    Badge {{ color: Color::base(Hue::Danger), \"\" }}
}}
Avatar {{ image_url: \"/img/avatars/007m.jpg\",
    Badge {{ color: Color::base(Hue::Success), \"\" }}
}}
Avatar {{ \"SA\"
    Badge {{ color: Color::base(Hue::Warning), \"\" }}
}}
Avatar {{ image_url: \"/img/avatars/004f.jpg\",
    Badge {{ color: Color::base(Hue::Info), \"\" }}
}}
Avatar {{ image_url: \"/img/avatars/004f.jpg\",
    Badge {{ color: Color::base(Hue::Gray), \"5\" }}
}}"
            }
            h2 {
                id: "avatar-shape",
                "Avatar shape"
            }
            p {
                "Change the shape of an avatar with the default Bootstrap image classes. You can make them round or square and change their border radius."
            }
            Example {
                centered: true,
                Avatar { image_url: "/img/avatars/005f.jpg", }
                Avatar { image_url: "/img/avatars/006f.jpg", shape: Shape::Rounded }
                Avatar { shape: Shape::RoundedCircle, "AA" }
                Avatar { image_url: "/img/avatars/008f.jpg", shape: Shape::Rounded0 }
                Avatar { image_url: "/img/avatars/009f.jpg", shape: Shape::Rounded3 }
            }
            ExampleCode {
                "Avatar {{ image_url: \"/img/avatars/005f.jpg\", }}
Avatar {{ image_url: \"/img/avatars/006f.jpg\", shape: Shape::Rounded }}
Avatar {{ shape: Shape::RoundedCircle, \"AA\" }}
Avatar {{ image_url: \"/img/avatars/008f.jpg\", shape: Shape::Rounded0 }}
Avatar {{ image_url: \"/img/avatars/009f.jpg\", shape: Shape::Rounded3 }}"
            }
            h2 {
                id: "avatars-list",
                "Avatars list"
            }
            p {
                "Create a list of avatars within one parent container."
            }
            Example {
                centered: true,
                AvatarList {
                    Avatar {
                        shape: Shape::RoundedCircle,
                        size: Size::Sm,
                        image_url: "/img/avatars/000m.jpg",
                    }
                    Avatar {
                        shape: Shape::RoundedCircle,
                        size: Size::Sm,
                        "JL"
                    }
                    Avatar {
                        shape: Shape::RoundedCircle,
                        size: Size::Sm,
                        image_url: "/img/avatars/002m.jpg",
                    }
                    Avatar {
                        shape: Shape::RoundedCircle,
                        size: Size::Sm,
                        image_url: "/img/avatars/003m.jpg",
                    }
                    Avatar {
                        shape: Shape::RoundedCircle,
                        size: Size::Sm,
                        image_url: "/img/avatars/000f.jpg",
                    }
                }
            }
            ExampleCode {
                "AvatarList {{
    Avatar {{
        shape: Shape::RoundedCircle,
        size: Size::Sm,
        image_url: \"/img/avatars/000m.jpg\",
    }}
    Avatar {{
        shape: Shape::RoundedCircle,
        size: Size::Sm,
        \"JL\"
    }}
    Avatar {{
        shape: Shape::RoundedCircle,
        size: Size::Sm,
        image_url: \"/img/avatars/002m.jpg\",
    }}
    Avatar {{
        shape: Shape::RoundedCircle,
        size: Size::Sm,
        image_url: \"/img/avatars/003m.jpg\",
    }}
    Avatar {{
        shape: Shape::RoundedCircle,
        size: Size::Sm,
        image_url: \"/img/avatars/000f.jpg\",
    }}
}}"
            }
            h2 {
                id: "stacked-list",
                "Stacked list"
            }
            p {
                "Make the list stack once a certain number of avatars is reached to make it look clear and display well regardless of the screen size."
            }
            Example {
                centered: true,
                AvatarList {
                    stacked: true,
                    Avatar { "EB" }
                    Avatar { image_url: "/img/avatars/016f.jpg", }
                    Avatar { image_url: "/img/avatars/015m.jpg", }
                    Avatar { image_url: "/img/avatars/017f.jpg", }
                    Avatar { image_url: "/img/avatars/018f.jpg", }
                    Avatar { "+8" }
                }
            }
            ExampleCode {
                "AvatarList {{
    stacked: true,
    Avatar {{ \"EB\" }}
    Avatar {{ image_url: \"/img/avatars/016f.jpg\", }}
    Avatar {{ image_url: \"/img/avatars/015m.jpg\", }}
    Avatar {{ image_url: \"/img/avatars/017f.jpg\", }}
    Avatar {{ image_url: \"/img/avatars/018f.jpg\", }}
    Avatar {{ \"+8\" }}
}}"
            }
        }
})
}