use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::{Input, InputType};
use crate::ui::label::Label;

#[component]
pub fn DemoDirectionProvider() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Ltr, class: "w-full max-w-sm",
            Card {
                CardHeader {
                    CardTitle { "Login to your account" }
                    CardDescription { "Enter your email below to login to your account" }
                }
                CardContent {
                    div { class: "flex flex-col gap-6",
                        div { class: "grid gap-2",
                            Label { html_for: "email-ltr", "Email" }
                            Input { id: "email-ltr", r#type: InputType::Email, placeholder: "m@example.com" }
                        }
                        div { class: "grid gap-2",
                            div { class: "flex items-center",
                                Label { html_for: "password-ltr", "Password" }
                                a { href: "#", class: "text-sm hover:underline ms-auto underline-offset-4",
                                    "Forgot your password?"
                                }
                            }
                            Input { id: "password-ltr", r#type: InputType::Password }
                        }
                    }
                }
                CardFooter { class: "flex-col gap-2",
                    Button { class: "w-full", "Login" }
                    Button { variant: ButtonVariant::Outline, class: "w-full", "Login with Google" }
                }
            }
        }
    }
}
