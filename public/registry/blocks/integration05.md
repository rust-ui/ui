


```rust
use dioxus::prelude::*;
use icons::{
    Atom, Boxes, CircleDot, Code, Cpu, Database, Globe, Layers, Package, Search, Shield, Sparkles, Square, Star,
    Triangle, Wind, Zap,
};

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
fn Integration05IconCard(children: Element) -> Element {
    rsx! {
        div {
            aria_hidden: "true",
            class: "flex relative z-20 rounded-xl border bg-card size-20",
            div { class: "m-auto size-fit *:size-8 *:text-muted-foreground", {children} }
        }
    }
}

#[component]
pub fn Integration05() -> Element {
    rsx! {
        main { role: "main",
            div { "data-theme": "quartz", class: "scheme-light bg-background",
                section { class: "bg-muted",
                    div { class: "py-24 bg-muted/50",
                        div { class: "px-6 mx-auto max-w-5xl perspective-dramatic group",
                            div { class: "relative justify-between items-center pb-1 mx-auto space-y-6 max-w-2xl from-transparent transition-transform duration-1000 scale-y-90 hover:scale-y-100 rotate-x-6 mask-radial-from-70% mask-radial-[50%_90%] group hover:rotate-x-0",
                                div { class: "absolute inset-0 opacity-25 mask-radial-to-55% bg-[radial-gradient(var(--color-foreground)_1px,transparent_1px)] [background-size:16px_16px]" }
                                div {
                                    div { class: "overflow-hidden",
                                        div {
                                            class: "flex w-max",
                                            style: "gap: 56px; flex-direction: row; transform: translateX(-73.98px);",
                                            Integration05IconCard { Square {} }
                                            Integration05IconCard { Atom {} }
                                            Integration05IconCard { Wind {} }
                                            Integration05IconCard { CircleDot {} }
                                            Integration05IconCard { Star {} }
                                            Integration05IconCard { Sparkles {} }
                                            Integration05IconCard { Zap {} }
                                            Integration05IconCard { Cpu {} }
                                            Integration05IconCard { Boxes {} }
                                            Integration05IconCard { Layers {} }
                                            Integration05IconCard { Package {} }
                                            Integration05IconCard { Code {} }
                                        }
                                    }
                                }
                                div {
                                    div { class: "overflow-hidden",
                                        div {
                                            class: "flex w-max",
                                            style: "gap: 56px; flex-direction: row; transform: translateX(-115.903px);",
                                            Integration05IconCard { Database {} }
                                            Integration05IconCard { Globe {} }
                                            Integration05IconCard { Triangle {} }
                                            Integration05IconCard { Shield {} }
                                            Integration05IconCard { Star {} }
                                            Integration05IconCard { Zap {} }
                                            Integration05IconCard { Cpu {} }
                                            Integration05IconCard { Boxes {} }
                                            Integration05IconCard { Layers {} }
                                            Integration05IconCard { Package {} }
                                            Integration05IconCard { Code {} }
                                            Integration05IconCard { Atom {} }
                                        }
                                    }
                                }
                                div {
                                    div { class: "overflow-hidden",
                                        div {
                                            class: "flex w-max",
                                            style: "gap: 56px; flex-direction: row; transform: translateX(-382.935px);",
                                            Integration05IconCard { Wind {} }
                                            Integration05IconCard { Sparkles {} }
                                            Integration05IconCard { Database {} }
                                            Integration05IconCard { Globe {} }
                                            Integration05IconCard { Shield {} }
                                            Integration05IconCard { Star {} }
                                            Integration05IconCard { Zap {} }
                                            Integration05IconCard { Cpu {} }
                                            Integration05IconCard { Boxes {} }
                                            Integration05IconCard { Layers {} }
                                            Integration05IconCard { Package {} }
                                            Integration05IconCard { Code {} }
                                        }
                                    }
                                }
                                div { class: "flex absolute inset-0 gap-2 justify-center m-auto -translate-y-3.5 size-fit",
                                    div { class: "relative z-20 p-1 rounded-2xl border bg-muted",
                                        div { class: "flex relative rounded-xl border shadow-xl bg-background shadow-black-950/10 size-24 border-black/25 dark:bg-background dark:border-white/25 dark:shadow-white/10",
                                            div { class: "m-auto size-fit *:size-8",
                                                Search { class: "text-muted-foreground" }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "mx-auto mt-12 max-w-xl text-center",
                                h2 { class: "text-3xl font-semibold md:text-4xl text-balance",
                                    "Thousands of Icons with your favorite Tools"
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
    }
}
```