use tablerust::dioxus::prelude::*;
use tablerust::components::{Alert, AlertTitle, AlertType, Avatar, AlertLink};
use tablerust::enums::*;
use tablerust::icon::*;
use tablerust::color::*;
use super::component_doc_page::*;

pub fn Alerts(cx: Scope) -> Element {
    let success_alert_visible = use_state(&cx, || true);
    let info_alert_visible = use_state(&cx, || true);
    let warning_alert_visible = use_state(&cx, || true);
    let danger_alert_visible = use_state(&cx, || true);

    cx.render(rsx! {
        ComponentDocPage {
            title: "Alerts",
            href: "https://preview.tabler.io/docs/alerts.html",
            description: "Alert messages are used to inform users of the status of their action and help them solve any problems that might have occurred. Good design of alert modals is very important for the overall user experience of a website or app.",
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
            Example {
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

            ExampleCode {
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
            h2 {
                id: "alert-links",
                "Alert links"
            }
            p {
                "Add a link to your alert message to redirect users to the details they need to complete or additional information they should read."
            }
            Example {
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
            ExampleCode {
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
            h2 {
                id: "dismissible-alerts",
                "Dismissible alerts with icons"
            }
            p {
                "Add the "
                code {
                    class: "language-plaintext highlighter-rouge",
                    "x"
                }
                " close button to make an alert modal dismissible. Thanks to that, your alert modal will disappear only once the user closes it."
            }
            Example {
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
            ExampleCode {
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
            h2 {
                id: "alert-with-avatar",
                "Alert with avatar"
            }
            p {
                "Add an avatar to your alert modal to make it more personalized."
            }
            Example {
                Alert {
                    alert_type: AlertType::Success,
                    icon_or_avatar: cx.render(rsx!{
                        Avatar { class: "me-3", image_url: "/img/avatars/000m.jpg" }
                    }),
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit."
                }
                Alert {
                    alert_type: AlertType::Info,
                    icon_or_avatar: cx.render(rsx!{
                        Avatar { class: "me-3", "JL" }
                    }),
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit."
                }
                Alert {
                    alert_type: AlertType::Warning,
                    icon_or_avatar: cx.render(rsx!{
                        Avatar { class: "me-3", image_url: "/img/avatars/002m.jpg" }
                    }),
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit."
                }
                Alert {
                    alert_type: AlertType::Danger,
                    icon_or_avatar: cx.render(rsx!{
                        Avatar { class: "me-3", image_url: "/img/avatars/003m.jpg" }
                    }),
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit."
                }
            }
            ExampleCode {
                "
Alert {{
    alert_type: AlertType::Success,
    icon_or_avatar: cx.render(rsx!{{
        Avatar {{ class: \"me-3\", image_url: \"/img/avatars/000m.jpg\" }}
    }}),
    \"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit.\"
}}
Alert {{
    alert_type: AlertType::Info,
    icon_or_avatar: cx.render(rsx!{{
        Avatar {{ class: \"me-3\", \"JL\" }}
    }}),
    \"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit.\"
}}
Alert {{
    alert_type: AlertType::Warning,
    icon_or_avatar: cx.render(rsx!{{
        Avatar {{ class: \"me-3\", image_url: \"/img/avatars/002m.jpg\" }}
    }}),
    \"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit.\"
}}
Alert {{
    alert_type: AlertType::Danger,
    icon_or_avatar: cx.render(rsx!{{
        Avatar {{ class: \"me-3\", image_url: \"/img/avatars/003m.jpg\" }}
    }}),
    \"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit.\"
}}"
            } // TODO : buttons
//             h2 {
//                 id: "alert-with-buttons",
//                 "Alert with buttons"
//             }
//             p {
//                 "Add primary and secondary buttons to your alert modals if you want users to take a particular action based on the information included in the modal message."
//             }
//             Example {
//                 success_alert_visible.then(|| rsx! {
//                     Alert {
//                         alert_type: AlertType::Success,
//                         dismissible: true,
//                         ondismiss: move |_| {
//                             success_alert_visible.set(false);
//                         },
//                         h3 {
//                             "Some Title"
//                         }
//                         div {
//                             "Lorem ipsum Minim ad pariatur eiusmod ea ut nulla aliqua est quis id dolore minim voluptate."
//                         }
//                     }
//                 })
//                 info_alert_visible.then(|| rsx! {
//                     Alert {
//                         alert_type: AlertType::Info,
//                         dismissible: true,
//                         ondismiss: move |_| {
//                             info_alert_visible.set(false);
//                         },
//                         h3 {
//                             "Some Title"
//                         }
//                         div {
//                             "Lorem ipsum Minim ad pariatur eiusmod ea ut nulla aliqua est quis id dolore minim voluptate."
//                         }
//                     }
//                 })
//                 warning_alert_visible.then(|| rsx! {
//                     Alert {
//                         alert_type: AlertType::Warning,
//                         dismissible: true,
//                         ondismiss: move |_| {
//                             warning_alert_visible.set(false);
//                         },
//                         h3 {
//                             "Some Title"
//                         }
//                         div {
//                             "Lorem ipsum Minim ad pariatur eiusmod ea ut nulla aliqua est quis id dolore minim voluptate."
//                         }
//                     }
//                 })
//                 danger_alert_visible.then(|| rsx! {
//                     Alert {
//                         alert_type: AlertType::Danger,
//                         dismissible: true,
//                         ondismiss: move |_| {
//                             danger_alert_visible.set(false);
//                         },
//                         h3 {
//                             "Some Title"
//                         }
//                         div {
//                             "Lorem ipsum Minim ad pariatur eiusmod ea ut nulla aliqua est quis id dolore minim voluptate."
//                         }
//                     }
//                 })
//             }
//             ExampleCode {
//                 "
// let success_alert_visible = use_state(&cx, || true);
// let info_alert_visible = use_state(&cx, || true);
// let warning_alert_visible = use_state(&cx, || true);
// let danger_alert_visible = use_state(&cx, || true);
//
// cx.render(rsx! {{
//     success_alert_visible.then(|| rsx! {{
//         Alert {{
//             alert_type: AlertType::Success,
//             icon_or_avatar: cx.render(rsx!{{
//                 Icon {{icon: \"check\",}}
//             }}),
//             dismissible: true,
//             ondismiss: move |_| {{
//                 success_alert_visible.set(false);
//             }},
//             AlertTitle {{
//                 \"Wow! Everything worked!\"
//             }}
//             div {{
//                 class: \"text-muted\",
//                 \"Your account has been saved!\"
//             }}
//         }}
//     }})
//     info_alert_visible.then(|| rsx! {{
//         Alert {{
//             alert_type: AlertType::Info,
//             icon_or_avatar: cx.render(rsx!{{
//                 Icon {{icon: \"info-circle\",}}
//             }}),
//             dismissible: true,
//             ondismiss: move |_| {{
//                 info_alert_visible.set(false);
//             }},
//             AlertTitle {{
//                 \"Did you know?\"
//             }}
//             div {{
//                 class: \"text-muted\",
//                 \"Here is something that you might like to know.\"
//             }}
//         }}
//     }})
//     warning_alert_visible.then(|| rsx! {{
//         Alert {{
//             alert_type: AlertType::Warning,
//             icon_or_avatar: cx.render(rsx!{{
//                 Icon {{icon: \"alert-triangle\",}}
//             }}),
//             dismissible: true,
//             ondismiss: move |_| {{
//                 warning_alert_visible.set(false);
//             }},
//             AlertTitle {{
//                 \"Uh oh, something went wrong\"
//             }}
//             div {{
//                 class: \"text-muted\",
//                 \"Sorry! There was a problem with your request.\"
//             }}
//         }}
//     }})
//     danger_alert_visible.then(|| rsx! {{
//         Alert {{
//             alert_type: AlertType::Danger,
//             icon_or_avatar: cx.render(rsx!{{
//                 Icon {{icon: \"alert-circle\",}}
//             }}),
//             dismissible: true,
//             ondismiss: move |_| {{
//                 danger_alert_visible.set(false);
//             }},
//             AlertTitle {{
//                 \"I'm so sorry…\"
//             }}
//             div {{
//                 class: \"text-muted\",
//                 \"Your account has been deleted and can't be restored.\"
//             }}
//         }}
//     }})
// }})"
//             }
            h2 {
                id: "important-alerts",
                "Important alerts"
            }
            p {
                "If you want your alert to be really eye-catching, you can add the 'important' property"
            }
            Example {
                success_alert_visible.then(|| rsx! {
                    Alert {
                        alert_type: AlertType::Success,
                        icon_or_avatar: cx.render(rsx!{
                            Icon { icon: "check" }
                        }),
                        important: true,
                        dismissible: true,
                        ondismiss: move |_| {
                            success_alert_visible.set(false);
                        },
                        "Your account has been saved!"
                    }
                })
                info_alert_visible.then(|| rsx! {
                    Alert {
                        alert_type: AlertType::Info,
                        icon_or_avatar: cx.render(rsx!{
                            Icon { icon: "info-circle" }
                        }),
                        important: true,
                        dismissible: true,
                        ondismiss: move |_| {
                            info_alert_visible.set(false);
                        },
                        "Here is something that you might like to know."
                    }
                })
                warning_alert_visible.then(|| rsx! {
                    Alert {
                        alert_type: AlertType::Warning,
                        icon_or_avatar: cx.render(rsx!{
                            Icon { icon: "alert-triangle" }
                        }),
                        important: true,
                        dismissible: true,
                        ondismiss: move |_| {
                            warning_alert_visible.set(false);
                        },
                        "Sorry! There was a problem with your request."
                    }
                })
                danger_alert_visible.then(|| rsx! {
                    Alert {
                        alert_type: AlertType::Danger,
                        icon_or_avatar: cx.render(rsx!{
                            Icon { icon: "alert-circle" }
                        }),
                        important: true,
                        dismissible: true,
                        ondismiss: move |_| {
                            danger_alert_visible.set(false);
                        },
                        "Your account has been deleted and can't be restored."
                    }
                })
            }
            ExampleCode {
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
                Icon {{ icon: \"check\" }}
            }}),
            important: true,
            dismissible: true,
            ondismiss: move |_| {{
                success_alert_visible.set(false);
            }},
            \"Your account has been saved!\"
        }}
    }})
    info_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Info,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{ icon: \"info-circle\" }}
            }}),
            important: true,
            dismissible: true,
            ondismiss: move |_| {{
                info_alert_visible.set(false);
            }},
            \"Here is something that you might like to know.\"
        }}
    }})
    warning_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Warning,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{ icon: \"alert-triangle\" }}
            }}),
            important: true,
            dismissible: true,
            ondismiss: move |_| {{
                warning_alert_visible.set(false);
            }},
            \"Sorry! There was a problem with your request.\"
        }}
    }})
    danger_alert_visible.then(|| rsx! {{
        Alert {{
            alert_type: AlertType::Danger,
            icon_or_avatar: cx.render(rsx!{{
                Icon {{ icon: \"alert-circle\" }}
            }}),
            important: true,
            dismissible: true,
            ondismiss: move |_| {{
                danger_alert_visible.set(false);
            }},
            \"Your account has been deleted and can't be restored.\"
        }}
    }})
}})"
            }
        }
    })
}