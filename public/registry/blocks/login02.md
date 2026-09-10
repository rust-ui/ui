


```rust
use dioxus::prelude::*;
use icons::{Eye, EyeOff, GalleryVerticalEnd, Github};

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::input::{Input, InputType};
use crate::components::ui::label::Label;

#[component]
pub fn Login02() -> Element {
    let mut show_password = use_signal(|| false);

    rsx! {
        div { class: "grid lg:grid-cols-2 min-h-svh",
            div { class: "flex flex-col gap-4 p-6 md:p-10",
                div { class: "flex gap-2 justify-center md:justify-start",
                    a { href: "#", class: "flex gap-2 items-center font-medium",
                        div { class: "flex justify-center items-center rounded-md bg-primary text-primary-foreground size-6",
                            GalleryVerticalEnd { class: "size-4" }
                        }
                        "Acme Inc."
                    }
                }
                div { class: "flex flex-1 justify-center items-center",
                    div { class: "w-full max-w-xs",
                        form { class: "flex flex-col gap-6",
                            div { class: "flex flex-col gap-2 items-center text-center",
                                h1 { class: "text-2xl font-bold", "Login to your account" }
                                p { class: "text-sm text-muted-foreground text-balance",
                                    "Enter your email below to login to your account"
                                }
                            }
                            div { class: "grid gap-6",
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
                                            class: "ml-auto text-sm hover:underline underline-offset-4",
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
                                Button { class: "w-full", "Login" }
                                div { class: "relative text-sm text-center after:border-border after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t",
                                    span { class: "relative z-10 px-2 bg-background text-muted-foreground",
                                        "Or continue with"
                                    }
                                }
                                Button { variant: ButtonVariant::Outline, class: "w-full",
                                    Github {}
                                    span { "Login with GitHub" }
                                }
                            }
                            div { class: "text-sm text-center",
                                "Don't have an account? "
                                a { href: "#", class: "underline underline-offset-4", "Sign up" }
                            }
                        }
                    }
                }
            }
            div { class: "hidden relative lg:block bg-muted",
                img {
                    src: "/placeholder.svg",
                    alt: "Image",
                    class: "object-cover absolute inset-0 w-full h-full dark:brightness-[0.2] dark:grayscale",
                }
            }
        }
    }
}
```