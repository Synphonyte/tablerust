use tablerust::dioxus::prelude::*;
use tablerust::components::{Alert, AlertTitle, AlertType, Avatar, AlertLink};
use tablerust::enums::*;
use tablerust::icon::*;
use tablerust::color::*;

pub fn Alerts(cx: Scope) -> Element {
    let success_alert_visible = use_state(&cx, || true);
    let info_alert_visible = use_state(&cx, || true);
    let warning_alert_visible = use_state(&cx, || true);
    let danger_alert_visible = use_state(&cx, || true);

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
            h2 {
                id: "alert-links",
                "Alert links"
            }
            p {
                "Add a link to your alert message to redirect users to the details they need to complete or additional information they should read."
            }
            div {
                class: "example no_toc_section",
                div {
                    class: "example-content",
                    Alert {
                        alert_type: AlertType::Success,
                        "This is a success alert — "
                        AlertLink {"check it out"}
                        "!"
                    }
                    Alert {
                        alert_type: AlertType::Info,
                        "This is a info alert — "
                        AlertLink {"check it out"}
                        "!"
                    }
                    Alert {
                        alert_type: AlertType::Warning,
                        "This is a warning alert — "
                        AlertLink {"check it out"}
                        "!"
                    }
                    Alert {
                        alert_type: AlertType::Danger,
                        "This is a danger alert — "
                        AlertLink {"check it out"}
                        "!"
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
    \"This is a success alert — \"
    AlertLink {{\"check it out\"}}
    \"!\"
}}
Alert {{
    alert_type: AlertType::Info,
    \"This is a info alert — \"
    AlertLink {{\"check it out\"}}
    \"!\"
}}
Alert {{
    alert_type: AlertType::Warning,
    \"This is a warning alert — \"
    AlertLink {{\"check it out\"}}
    \"!\"
}}
Alert {{
    alert_type: AlertType::Danger,
    \"This is a danger alert — \"
    AlertLink {{\"check it out\"}}
    \"!\"
}}"
                        }
                    }
                }
            }
            h2 {
                id: "dismissible-alerts",
                "Dismissible alerts with icons"
            }
            p {
                "Add the"
                code {
                    class: "language-plaintext highlighter-rouge",
                    "x"
                }
                "close button to make an alert modal dismissible. Thanks to that, your alert modal will disappear only once the user closes it."
            }
            div {
                class: "example no_toc_section",
                div {
                    class: "example-content",
                    success_alert_visible.then(|| rsx! {
                        Alert {
                            alert_type: AlertType::Success,
                            icon_or_avatar: cx.render(rsx!{
                                Icon {icon: "check",}
                            }),
                            dismissible: true,
                            ondismiss: move |_| {
                                success_alert_visible.set(false);
                            },
                            AlertTitle {
                                "Wow! Everything worked!"
                            }
                            div {
                                class: "text-muted",
                                "Your account has been saved!"
                            }
                        }
                    })
                    info_alert_visible.then(|| rsx! {
                        Alert {
                            alert_type: AlertType::Info,
                            icon_or_avatar: cx.render(rsx!{
                                Icon {icon: "info-circle",}
                            }),
                            dismissible: true,
                            ondismiss: move |_| {
                                info_alert_visible.set(false);
                            },
                            AlertTitle {
                                "Did you know?"
                            }
                            div {
                                class: "text-muted",
                                "Here is something that you might like to know."
                            }
                        }
                    })
                    warning_alert_visible.then(|| rsx! {
                        Alert {
                            alert_type: AlertType::Warning,
                            icon_or_avatar: cx.render(rsx!{
                                Icon {icon: "alert-triangle",}
                            }),
                            dismissible: true,
                            ondismiss: move |_| {
                                warning_alert_visible.set(false);
                            },
                            AlertTitle {
                                "Uh oh, something went wrong"
                            }
                            div {
                                class: "text-muted",
                                "Sorry! There was a problem with your request."
                            }
                        }
                    })
                    danger_alert_visible.then(|| rsx! {
                        Alert {
                            alert_type: AlertType::Danger,
                            icon_or_avatar: cx.render(rsx!{
                                Icon {icon: "alert-circle",}
                            }),
                            dismissible: true,
                            ondismiss: move |_| {
                                danger_alert_visible.set(false);
                            },
                            AlertTitle {
                                "I'm so sorry…"
                            }
                            div {
                                class: "text-muted",
                                "Your account has been deleted and can't be restored."
                            }
                        }
                    })
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
                            "
let success_alert_visible = use_state(&cx, || true);
let info_alert_visible = use_state(&cx, || true);
let warning_alert_visible = use_state(&cx, || true);
let danger_alert_visible = use_state(&cx, || true);

cx.render(rsx! {{
    success_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Success,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{icon: \"check\",}}
            }}),
            dismissible: true,
            ondismiss: move |_| {{
                success_alert_visible.set(false);
            }},
            AlertTitle {{
                \"Wow! Everything worked!\"
            }}
            div {{
                class: \"text-muted\",
                \"Your account has been saved!\"
            }}
        }}
    }})
    info_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Info,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{icon: \"info-circle\",}}
            }}),
            dismissible: true,
            ondismiss: move |_| {{
                info_alert_visible.set(false);
            }},
            AlertTitle {{
                \"Did you know?\"
            }}
            div {{
                class: \"text-muted\",
                \"Here is something that you might like to know.\"
            }}
        }}
    }})
    warning_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Warning,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{icon: \"alert-triangle\",}}
            }}),
            dismissible: true,
            ondismiss: move |_| {{
                warning_alert_visible.set(false);
            }},
            AlertTitle {{
                \"Uh oh, something went wrong\"
            }}
            div {{
                class: \"text-muted\",
                \"Sorry! There was a problem with your request.\"
            }}
        }}
    }})
    danger_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Danger,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{icon: \"alert-circle\",}}
            }}),
            dismissible: true,
            ondismiss: move |_| {{
                danger_alert_visible.set(false);
            }},
            AlertTitle {{
                \"I'm so sorry…\"
            }}
            div {{
                class: \"text-muted\",
                \"Your account has been deleted and can't be restored.\"
            }}
        }}
    }})
}})"
                        }
                    }
                }
            }
        }
    })
}