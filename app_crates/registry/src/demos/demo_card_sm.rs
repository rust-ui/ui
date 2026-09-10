use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCardSm() -> Element {
    rsx! {
        Card { class: "max-w-sm py-3",
            CardHeader {
                CardTitle { "Card Title" }
            }
            CardContent {
                CardDescription {
                    "A compact card with reduced padding, ideal for dense UI panels like customizers or sidebars."
                }
            }
            CardFooter { class: "justify-end",
                Button { variant: ButtonVariant::Outline, "Cancel" }
                Button { "Confirm" }
            }
        }
    }
}
