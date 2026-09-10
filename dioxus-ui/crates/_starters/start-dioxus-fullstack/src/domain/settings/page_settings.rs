use dioxus::prelude::*;

use crate::components::hooks::use_theme_mode::use_theme_mode;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::separator::Separator;

#[component]
pub fn SettingsPage() -> Element {
    let mut is_dark = use_theme_mode();

    rsx! {
        div { class: "flex flex-col gap-6 p-6 pt-[calc(env(safe-area-inset-top)+4rem)] sm:pt-6",
            h1 { class: "text-2xl font-bold tracking-tight", "Settings" }

            Card {
                CardHeader {
                    CardTitle { "Appearance" }
                    CardDescription { "Customize how the app looks." }
                }
                CardContent { class: "flex flex-col gap-4",
                    div { class: "flex items-center justify-between",
                        div { class: "flex flex-col gap-0.5",
                            p { class: "text-sm font-medium", "Dark mode" }
                            p { class: "text-xs text-muted-foreground", "Toggle between light and dark theme." }
                        }
                        button {
                            class: "relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
                            class: if is_dark() { "bg-primary" } else { "bg-input" },
                            onclick: move |_| is_dark.toggle(),
                            span {
                                class: "inline-block h-4 w-4 rounded-full bg-background shadow-sm transition-transform",
                                class: if is_dark() { "translate-x-6" } else { "translate-x-1" },
                            }
                        }
                    }

                    Separator {}

                    div { class: "flex items-center justify-between",
                        div { class: "flex flex-col gap-0.5",
                            p { class: "text-sm font-medium", "Version" }
                            p { class: "text-xs text-muted-foreground", "0.1.0" }
                        }
                    }
                }
            }
        }
    }
}
