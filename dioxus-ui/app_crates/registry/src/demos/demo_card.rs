use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> Element {
    rsx! {
        div { class: "space-y-6 w-full",
            Card { class: "max-w-lg",
                CardHeader {
                    CardTitle { "Card Title" }
                }
                CardContent {
                    CardDescription {
                        "Hello, this is a more detailed description of the card content. You can add more text here to provide additional information about the card's purpose, features, or any other relevant details that might interest the viewer."
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
