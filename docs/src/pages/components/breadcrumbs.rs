#![allow(non_snake_case)]

use tablerust::dioxus::prelude::*;
use tablerust::components::{Breadcrumb, BreadcrumbSeparator, BreadcrumbItem};
use crate::Code::Code;
use super::component_doc_page::*;

pub fn Breadcrumbs(cx: Scope) -> Element {
    cx.render( rsx!{
        ComponentDocPage {
            title: "Breadcrumbs",
            href: "https://preview.tabler.io/docs/Breadcrumb.html",
            description: "Breadcrumbs are used to show the current website or app location and reduce the number of actions users have to take. Thanks to breadcrumbs, they can easily navigate within the website hierarchy and better understand its structure.",
            h2 {
                id: "default-markup",
                "Default markup"
            }
            p {
                "The active property in a BreadcrumpItem will help you indicate the current page location and facilitate navigation within your website or app."
            }
        }
        Example {
            centered: true,
            Breadcrumb {
                BreadcrumbItem { a { href: "#", "Home" } }
                BreadcrumbItem { a { href: "#", "Library" } }
                BreadcrumbItem { active: true, a { href: "#", "Data" } }
            }
        }
        ExampleCode {
            "Breadcrumb {{
    BreadcrumbItem {{ a {{ href: \"/\", \"Home\" }} }}
    BreadcrumbItem {{ a {{ href: \"/library\", \"Library\" }} }}
    BreadcrumbItem {{ active: true, a {{ href: \"/library/data\", \"Data\" }} }}
}}"
        }
        h2 {
            id: "router-breadcrumb",
            "Router Breadcrumbs"
        }
        p {
            "The easiest way to use router "
            Code {"Link"}
            " components inside breadcrumbs is to use "
            Code {"BreadcrumbRouterItem"}
            "."
        }
        Example {
            centered: true,
            Breadcrumb {
                BreadcrumbItem { a { href: "#", "Home" } }
                BreadcrumbItem { a { href: "#", "Library" } }
                BreadcrumbItem { active: true, a { href: "#", "Data" } }
            }
        }
        ExampleCode {
            "Router {{
    ...
    BreadcrumbRouterItem {{
        to: \"/\",
        \"Home\"
    }}
    BreadcrumbRouterItem {{
        to: \"/library\",
        \"Library\"
    }}
    BreadcrumbRouterItem {{
        to: \"/library/data\",
        \"Data\"
    }}
}}"
        }
        h2 {
            id: "breadcrumb-variations",
            "Breadcrumb variations"
        }
        p {
            "Depending on the aesthetics of your design, you can choose "
            Code {"Slash"}
            " (default), "
            Code {"Dot"}
            ", "
            Code {"Bullet"}
            " or "
            Code {"Arrow"}
            "."
        }
        Example {
            centered: true,
            Breadcrumb {
                separator: BreadcrumbSeparator::Dot,
                BreadcrumbItem { a { href: "#", "Home" } }
                BreadcrumbItem { a { href: "#", "Library" } }
                BreadcrumbItem { active: true, a { href: "#", "Data" } }
            }
        }
        ExampleCode {
            "Breadcrumb {{
    separator: BreadcrumbSeparator::Dot,
    BreadcrumbItem {{ a {{ href: \"/\", \"Home\" }} }}
    BreadcrumbItem {{ a {{ href: \"/library\", \"Library\" }} }}
    BreadcrumbItem {{ active: true, a {{ href: \"/library/data\", \"Data\" }} }}
}}"
        }
        Example {
            centered: true,
            Breadcrumb {
                separator: BreadcrumbSeparator::Arrow,
                BreadcrumbItem { a { href: "#", "Home" } }
                BreadcrumbItem { a { href: "#", "Library" } }
                BreadcrumbItem { active: true, a { href: "#", "Data" } }
            }
        }
        ExampleCode {
            "Breadcrumb {{
    separator: BreadcrumbSeparator::Arrow,
    BreadcrumbItem {{ a {{ href: \"/\", \"Home\" }} }}
    BreadcrumbItem {{ a {{ href: \"/library\", \"Library\" }} }}
    BreadcrumbItem {{ active: true, a {{ href: \"/library/data\", \"Data\" }} }}
}}"
        }
        Example {
            centered: true,
            Breadcrumb {
                separator: BreadcrumbSeparator::Bullet,
                BreadcrumbItem { a { href: "#", "Home" } }
                BreadcrumbItem { a { href: "#", "Library" } }
                BreadcrumbItem { active: true, a { href: "#", "Data" } }
            }
        }
        ExampleCode {
            "Breadcrumb {{
    separator: BreadcrumbSeparator::Bullet,
    BreadcrumbItem {{ a {{ href: \"/\", \"Home\" }} }}
    BreadcrumbItem {{ a {{ href: \"/library\", \"Library\" }} }}
    BreadcrumbItem {{ active: true, a {{ href: \"/library/data\", \"Data\" }} }}
}}"
        }
        h2 {
            id: "breadcrumb-in-headers",
            "Breadcrumb in headers"
        }
        p {
            "If you wish to use breadcrumbs in headers, place them above the headers."
        }
        Example {
            // Todo: Buttons
            class: "example-bg",
            Breadcrumb {
                BreadcrumbItem { a { href: "#", "Home" } }
                BreadcrumbItem { a { href: "#", "Library" } }
                BreadcrumbItem { active: true, a { href: "#", "Data" } }
            }
            h2 {
                class: "page-title",
                span {
                    class: "text-truncate",
                    "Knights of Ni, we are but simple travelers who seek the enchanter who lives beyond these woods."
                }
            }
        }
        ExampleCode {
            "Breadcrumb {{
    BreadcrumbItem {{ a {{ href: \"/\", \"Home\" }} }}
    BreadcrumbItem {{ a {{ href: \"/library\", \"Library\" }} }}
    BreadcrumbItem {{ active: true, a {{ href: \"/library/data\", \"Data\" }} }}
}}
h2 {{
    class: \"page-title\",
    span {{
        class: \"text-truncate\",
        \"Knights of Ni, we are but simple travelers who seek the enchanter who lives...\"
    }}
}}"
        }
    })
}