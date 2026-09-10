use dioxus::prelude::*;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use registry::ui::switch::Switch;

#[component]
pub fn CardCookieSettings() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Cookie Settings" }
                CardDescription { "Manage your cookie settings here." }
            }
            CardContent { class: "grid gap-6",
                div { class: "flex gap-4 justify-between items-start",
                    div { class: "flex flex-col gap-1.5",
                        span { class: "text-base font-medium", "Strictly Necessary" }
                        span { class: "text-sm font-normal leading-snug text-muted-foreground",
                            "These cookies are essential in order to use the website and use its features."
                        }
                    }
                    Switch { checked: true }
                }
                div { class: "flex gap-4 justify-between items-start",
                    div { class: "flex flex-col gap-1.5",
                        span { class: "text-base font-medium", "Functional Cookies" }
                        span { class: "text-sm font-normal leading-snug text-muted-foreground",
                            "These cookies allow the website to provide personalized functionality."
                        }
                    }
                    Switch {}
                }
            }
            CardFooter {
                Button { variant: ButtonVariant::Outline, class: "w-full",
                    "Save preferences"
                }
            }
        }
    }
}
