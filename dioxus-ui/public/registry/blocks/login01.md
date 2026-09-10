


```rust
use dioxus::prelude::*;
use icons::{Eye, EyeOff};

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::{Input, InputType};
use crate::components::ui::label::Label;

#[component]
pub fn Login01() -> Element {
    let mut show_password = use_signal(|| false);

    rsx! {
        div { class: "flex justify-center items-center p-6 w-full md:p-10 min-h-svh",
            div { class: "w-full max-w-sm",
                div { class: "flex flex-col gap-6",
                    Card {
                        CardHeader {
                            CardTitle { "Login to your account" }
                            CardDescription { "Enter your email below to login to your account" }
                        }
                        CardContent {
                            form {
                                div { class: "flex flex-col gap-6",
                                    div { class: "grid gap-3",
                                        Label { html_for: "email", "Email" }
                                        Input {
                                            r#type: InputType::Email,
                                            id: "email",
                                            placeholder: "m@example.com",
                                            required: true,
                                        }
                                    }
                                    div { class: "grid gap-3",
                                        div { class: "flex items-center",
                                            Label { html_for: "password", "Password" }
                                            a {
                                                href: "#",
                                                class: "inline-block ml-auto text-sm hover:underline underline-offset-4",
                                                "Forgot your password?"
                                            }
                                        }
                                        div { class: "relative",
                                            Input {
                                                r#type: if show_password() { InputType::Text } else { InputType::Password },
                                                id: "password",
                                                required: true,
                                                class: "pr-10",
                                            }
                                            button {
                                                r#type: "button",
                                                class: "absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground",
                                                aria_label: if show_password() { "Hide password" } else { "Show password" },
                                                onclick: move |_| show_password.set(!show_password()),
                                                if show_password() {
                                                    EyeOff { class: "size-4" }
                                                    span { class: "sr-only", "Hide password" }
                                                } else {
                                                    Eye { class: "size-4" }
                                                    span { class: "sr-only", "Show password" }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-col gap-3",
                                        Button { class: "w-full", "Login" }
                                        Button { variant: ButtonVariant::Outline, class: "w-full",
                                            "Login with Google"
                                        }
                                    }
                                }
                                div { class: "mt-4 text-sm text-center",
                                    "Don't have an account? "
                                    a { href: "#", class: "underline underline-offset-4", "Sign up" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```