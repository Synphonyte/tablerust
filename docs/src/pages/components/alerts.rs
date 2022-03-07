use tablerust::dioxus::prelude::*;
use tablerust::components::{Alert, AlertTitle, AlertType};
use tablerust::icon::*;
use tablerust::color::*;

pub fn Alerts(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "d-flex mb-3",
                h1 {
                    class: "m-0",
                    "Alerts"
                }
                p {
                    class: "ms-auto",
                    a {
                        class: "d-flex align-items-center",
                        href: "https://preview.tabler.io/docs/alerts.html",
                        target: "_blank",
                        Icon {
                            icon: "external-link",
                            color: Color::base(Hue::Blue),
                        }
                        "Tabler documentation"
                    }
                }
            }
            p {
                class: "mb-4 text-muted",
                "Alert messages are used to inform users of the status of their action and help them solve any problems that might have occurred. Good design of alert modals is very important for the overall user experience of a website or app."
            }
            h2 {
                id: "default-markup",
                "Default markup"
            }
            p {
                "Depending on the information you need to convey, you can use one of the following types of alert messages – "
                strong { "success" }
                ", "
                strong { "info" }
                ", "
                strong { "warning" }
                " or "
                strong { "danger" }
                ". Using the right type of alert modal will help draw users’ attention to the message and prompt them to take action."
            }
            div {
                class: "example no_toc_section",
                div {
                    class: "example-content",
                    Alert {
                        alert_type: AlertType::Success,
                        AlertTitle {"Wow! Everything worked!"}
                        div {
                            class: "text-muted",
                            "Your account has been saved!"
                        }
                    }
                    Alert {
                        alert_type: AlertType::Info,
                        AlertTitle {"Did you know?"}
                        div {
                            class: "text-muted",
                            "Here is something that you might like to know."
                        }
                    }
                    Alert {
                        alert_type: AlertType::Warning,
                        AlertTitle {"Uh oh, something went wrong"}
                        div {
                            class: "text-muted",
                            "Sorry! There was a problem with your request."
                        }
                    }
                    Alert {
                        alert_type: AlertType::Danger,
                        AlertTitle {"I'm so sorry…"}
                        div {
                            class: "text-muted",
                            "Your account has been deleted and can't be restored."
                        }
                    }
                }
            }
            div {
                class: "example-code",
                figure {
                    class: "highlight",
                    pre {
                        code {
                            class: "language-html",
                            "data-lang": "html",
                            // TODO : syntax highlighting
                            "Alert {{
    alert_type: AlertType::Success,
    AlertTitle {{ \"Wow! Everything worked!\" }}
    div {{
        class: \"text-muted\",
        \"Your account has been saved!\"
    }}
}}
Alert {{
    alert_type: AlertType::Info,
    AlertTitle {{ \"Did you know?\" }}
    div {{
        class: \"text-muted\",
        \"Here is something that you might like to know.\"
    }}
}}
Alert {{
    alert_type: AlertType::Warning,
    AlertTitle {{ \"Uh oh, something went wrong\" }}
    div {{
        class: \"text-muted\",
        \"Sorry! There was a problem with your request.\"
    }}
}}
Alert {{
    alert_type: AlertType::Danger,
    AlertTitle {{ \"I'm so sorry…\" }}
    div {{
        class: \"text-muted\",
        \"Your account has been deleted and can't be restored.\"
    }}
}}"
                        }
                    }
                }
            }
        }
    })
}