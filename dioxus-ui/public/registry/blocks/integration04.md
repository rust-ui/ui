


```rust
use dioxus::prelude::*;
use icons::{CircleDot, LifeBuoy, Search, Sparkles, Square, Wind, Zap};

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
fn Integration04IconBubble(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let cls = format!(
        "flex bg-white rounded-full border shadow-sm size-12 shadow-black/5 dark:bg-white/5 dark:backdrop-blur-md {}",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { class: "{cls}",
            div { class: "m-auto size-fit *:size-5 *:text-muted-foreground", {children} }
        }
    }
}

#[component]
pub fn Integration04() -> Element {
    rsx! {
        section {
            div { class: "py-24 md:py-32",
                div { class: "px-6 mx-auto max-w-5xl",
                    div { class: "flex relative justify-between items-center mx-auto sm:max-w-sm aspect-16/10 group max-w-[22rem]",
                        div {
                            role: "presentation",
                            class: "absolute inset-0 z-10 justify-center items-center to-transparent rounded-full border-t opacity-0 animate-spin group-hover:opacity-100 bg-linear-to-b border-foreground/5 aspect-square from-lime-500/15 to-25% duration-[3.5s] dark:from-white/5",
                        }
                        div {
                            role: "presentation",
                            class: "absolute inset-16 z-10 justify-center items-center to-transparent rounded-full border-t opacity-0 animate-spin scale-90 group-hover:opacity-100 bg-linear-to-b border-foreground/5 aspect-square from-blue-500/15 to-25% duration-[3.5s]",
                        }
                        div { class: "flex absolute inset-0 justify-center items-center to-transparent rounded-full border-t bg-linear-to-b from-muted-foreground/15 aspect-square to-25%",
                            Integration04IconBubble { class: "absolute left-0 top-1/4 z-30 -translate-y-1/4 -translate-x-1/6", Sparkles {} }
                            Integration04IconBubble { class: "absolute top-0 z-30 -translate-y-1/2", Square {} }
                            Integration04IconBubble { class: "absolute right-0 top-1/4 z-30 -translate-y-1/4 translate-x-1/6", Wind {} }
                        }
                        div { class: "flex absolute inset-16 justify-center items-center to-transparent rounded-full border-t scale-90 bg-linear-to-b from-muted-foreground/15 aspect-square to-25%",
                            Integration04IconBubble { class: "absolute top-0 z-30 -translate-y-1/2", CircleDot {} }
                            Integration04IconBubble { class: "absolute left-0 top-1/4 z-30 -translate-x-1/4 -translate-y-1/4", LifeBuoy {} }
                            Integration04IconBubble { class: "absolute right-0 top-1/4 z-30 translate-x-1/4 -translate-y-1/4", Zap {} }
                        }
                        div { class: "flex absolute inset-x-0 bottom-0 gap-2 justify-center my-2 mx-auto w-fit",
                            div { class: "relative z-20 p-1 rounded-full border bg-muted",
                                div { class: "flex relative z-30 bg-white rounded-full border shadow-xl shadow-black-950/10 size-16 border-black/20 dark:backdrop-blur-md dark:bg-background dark:border-white/25 dark:shadow-white/15",
                                    div { class: "m-auto size-fit *:size-8",
                                        Search { class: "text-muted-foreground" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "relative z-20 mx-auto mt-12 space-y-6 max-w-lg text-center bg-linear-to-t from-background from-55%",
                        h2 { class: "text-3xl font-semibold md:text-4xl text-balance",
                            "Thousands of Icons"
                        }
                        p { class: "mt-3 text-center sm:text-center lg:mt-3 [word-spacing:0.02em] font-book text-muted-foreground",
                            "Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect harmony in your interface."
                        }
                        Button { variant: ButtonVariant::Outline, href: "#", "Get Started" }
                    }
                }
            }
        }
    }
}
```