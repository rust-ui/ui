use dioxus::prelude::*;
use icons::{Eye, EyeOff, Facebook};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent};
use crate::ui::input::{Input, InputType};
use crate::ui::label::Label;

/*
 * title: Split Login with Social Buttons
*/

#[component]
fn LogoGoogle() -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
            path {
                d: "M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z",
                fill: "currentColor",
            }
        }
    }
}

#[component]
fn LogoApple() -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
            path {
                d: "M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z",
                fill: "currentColor",
            }
        }
    }
}

#[component]
pub fn Login04() -> Element {
    let mut show_password = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col justify-center items-center p-6 md:p-10 bg-muted min-h-svh",
            div { class: "w-full max-w-sm md:max-w-3xl",
                div { class: "flex flex-col gap-6",
                    Card { class: "overflow-hidden p-0",
                        CardContent { class: "grid p-0 md:grid-cols-2",
                            form { class: "p-6 md:p-8",
                                div { class: "flex flex-col gap-6",
                                    div { class: "flex flex-col items-center text-center",
                                        h1 { class: "text-2xl font-bold", "Welcome back" }
                                        p { class: "text-muted-foreground text-balance",
                                            "Login to your Acme Inc account"
                                        }
                                    }
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
                                                class: "ml-auto text-sm hover:underline underline-offset-2",
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
                                        span { class: "relative z-10 px-2 bg-card text-muted-foreground",
                                            "Or continue with"
                                        }
                                    }
                                    div { class: "grid grid-cols-3 gap-4",
                                        Button { variant: ButtonVariant::Outline,
                                            LogoApple {}
                                            span { class: "hidden", "Login with Apple" }
                                        }
                                        Button { variant: ButtonVariant::Outline,
                                            LogoGoogle {}
                                            span { class: "hidden", "Login with Google" }
                                        }
                                        Button { variant: ButtonVariant::Outline,
                                            Facebook {}
                                            span { class: "hidden", "Login with Facebook" }
                                        }
                                    }
                                    div { class: "text-sm text-center",
                                        "Don't have an account? "
                                        a { href: "#", class: "underline underline-offset-4", "Sign up" }
                                    }
                                }
                            }
                            div { class: "hidden relative md:block bg-muted",
                                img {
                                    src: "/placeholder.svg",
                                    alt: "Image",
                                    class: "object-cover absolute inset-0 w-full h-full dark:brightness-[0.2] dark:grayscale",
                                }
                            }
                        }
                    }
                    div { class: "text-xs text-center text-muted-foreground text-balance",
                        "By clicking continue, you agree to our "
                        a { href: "#", class: "underline underline-offset-4", "Terms of Service" }
                        " and "
                        a { href: "#", class: "underline underline-offset-4", "Privacy Policy" }
                        "."
                    }
                }
            }
        }
    }
}
