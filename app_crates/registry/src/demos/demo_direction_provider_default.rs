use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::{Input, InputType};
use crate::ui::label::Label;

#[component]
pub fn DemoDirectionProviderDefault() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Ltr, class: "w-full max-w-sm",
            Card {
                CardHeader {
                    CardTitle { "Create an account" }
                    CardDescription { "Enter your details below to create your account" }
                }
                CardContent {
                    div { class: "flex flex-col gap-6",
                        div { class: "grid gap-2",
                            Label { html_for: "name-ltr-default", "Full Name" }
                            Input { id: "name-ltr-default", placeholder: "John Doe" }
                        }
                        div { class: "grid gap-2",
                            Label { html_for: "email-ltr-default", "Email" }
                            Input { id: "email-ltr-default", r#type: InputType::Email, placeholder: "m@example.com" }
                        }
                    }
                }
                CardFooter { class: "flex-col gap-2",
                    Button { class: "w-full", "Create Account" }
                    Button { variant: ButtonVariant::Outline, class: "w-full", "Sign up with Google" }
                }
            }
        }
    }
}
