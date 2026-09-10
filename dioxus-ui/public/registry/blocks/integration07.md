


```rust
use dioxus::prelude::*;
use icons::{Circle, LifeBuoy, Sparkles, Square, Star, Triangle, Wind};

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
fn Integration07IconCircle(children: Element) -> Element {
    rsx! {
        div { class: "flex relative z-20 m-auto rounded-full border border-transparent ring-1 shadow-md bg-background ring-foreground/10 size-full",
            div { class: "m-auto size-fit *:size-5 *:text-muted-foreground", {children} }
        }
    }
}

#[component]
pub fn Integration07() -> Element {
    rsx! {
        section { class: "bg-muted",
            div { class: "py-24 bg-muted/50",
                div { class: "px-6 mx-auto max-w-5xl",
                    div { class: "relative",
                        div { class: "inset-0 pb-20 m-auto space-y-5 w-full max-w-xl text-center md:absolute !aspect-auto h-fit",
                            Badge { variant: BadgeVariant::Outline, "Consistent style across all icons" }
                            h2 { class: "text-2xl font-semibold md:text-4xl text-balance",
                                "Thousands of Icons"
                            }
                            p { class: "text-muted-foreground text-balance",
                                "Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect harmony in your interface."
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Sm,
                                href: "#",
                                "Browse Icons"
                            }
                        }
                        div { class: "grid gap-1 md:grid-rows-6 *:aspect-square max-md:mt-12 md:grid-cols-18",
                            div {
                                aria_hidden: "true",
                                class: "flex col-start-3 md:row-start-2 max-md:hidden",
                                div { class: "mt-auto ml-auto rounded-lg border translate-x-full bg-muted/50 size-1/2 translate-y-[125%]" }
                            }
                            div {
                                aria_hidden: "true",
                                class: "flex md:row-start-5 max-md:hidden",
                                div { class: "ml-auto rounded-lg border md:translate-x-full bg-muted/50 size-1/2 md:-translate-y-[125%]" }
                            }
                            div {
                                aria_hidden: "true",
                                class: "flex md:row-start-3 col-start-16 max-md:hidden",
                                div { class: "rounded-lg border md:-translate-x-full bg-muted size-1/2 md:-translate-y-[125%]" }
                            }
                            div {
                                aria_hidden: "true",
                                class: "flex md:row-start-5 col-start-18 max-md:hidden",
                                div { class: "rounded-lg border md:-translate-x-full bg-muted size-1/2 md:-translate-y-[125%]" }
                            }
                            div { class: "col-start-3",
                                Integration07IconCircle { Sparkles {} }
                            }
                            div { class: "col-start-9 md:translate-x-1/2 md:row-start-8",
                                Integration07IconCircle { Square {} }
                            }
                            div { class: "md:row-start-3",
                                Integration07IconCircle { Star {} }
                            }
                            div { class: "col-start-3 md:row-start-5",
                                Integration07IconCircle { Triangle {} }
                            }
                            div { class: "col-start-16",
                                Integration07IconCircle { Circle {} }
                            }
                            div { class: "md:row-start-3 col-start-18",
                                Integration07IconCircle { LifeBuoy {} }
                            }
                            div { class: "md:row-start-5 col-start-16",
                                Integration07IconCircle { Wind {} }
                            }
                        }
                    }
                }
            }
        }
    }
}
```