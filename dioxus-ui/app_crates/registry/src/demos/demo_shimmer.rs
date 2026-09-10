use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::shimmer::Shimmer;

#[component]
pub fn DemoShimmer() -> Element {
    let mut loading = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex gap-2",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| loading.set(!loading()),
                    "Toggle Loading"
                }
            }
            Shimmer { loading,
                Card { class: "max-w-lg lg:max-w-2xl",
                    CardHeader {
                        CardTitle { "Card Title" }
                    }
                    CardContent {
                        CardDescription {
                            "Click 'Toggle Loading' to see the shimmer effect overlay."
                        }
                    }
                    CardFooter { class: "justify-end",
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                        Button { "Confirm" }
                    }
                }
            }
        }
    }
}
