use dioxus::prelude::*;
use icons::{
    Atom, Boxes, CircleDot, Code, Cpu, Database, Globe, Heart, Layers, Package, Rocket, Search, Shield, Sparkles,
    Square, Star, Wind, Zap,
};

use crate::ui::button::{Button, ButtonVariant};

/*
 * title: Icon Library with Scrolling Rows
*/

#[component]
fn Integration06IconBubble(children: Element) -> Element {
    rsx! {
        div { class: "flex relative z-20 rounded-full border bg-background size-12",
            div { class: "m-auto size-fit *:size-5", {children} }
        }
    }
}

#[component]
pub fn Integration06() -> Element {
    rsx! {
        section {
            div { class: "py-24 md:py-32 bg-muted dark:bg-background",
                div { class: "px-6 mx-auto max-w-5xl",
                    div { class: "relative justify-between items-center mx-auto space-y-6 sm:max-w-md bg-muted/25 group max-w-[22rem] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]",
                        div {
                            role: "presentation",
                            class: "absolute inset-0 opacity-50 -z-10 bg-[linear-gradient(to_right,var(--color-border)_1px,transparent_1px),linear-gradient(to_bottom,#4f4f4f2e_1px,transparent_1px)] bg-[size:32px_32px]",
                        }
                        div {
                            div { class: "overflow-hidden",
                                div {
                                    class: "flex w-max",
                                    style: "gap: 24px; flex-direction: row; transform: translateX(-119.64px);",
                                    Integration06IconBubble { CircleDot { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Atom { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Sparkles { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Star { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Square { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Wind { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Zap { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Cpu { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Boxes { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Layers { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Package { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Code { class: "text-muted-foreground" } }
                                }
                            }
                        }
                        div {
                            div { class: "overflow-hidden",
                                div {
                                    class: "flex w-max",
                                    style: "gap: 24px; flex-direction: row; transform: translateX(-311.08px);",
                                    Integration06IconBubble { Sparkles { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Square { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Atom { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Wind { class: "text-muted-foreground" } }
                                    Integration06IconBubble { CircleDot { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Database { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Globe { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Shield { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Rocket { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Heart { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Star { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Zap { class: "text-muted-foreground" } }
                                }
                            }
                        }
                        div {
                            div { class: "overflow-hidden",
                                div {
                                    class: "flex w-max",
                                    style: "gap: 24px; flex-direction: row; transform: translateX(-119.64px);",
                                    Integration06IconBubble { Square { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Wind { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Sparkles { class: "text-muted-foreground" } }
                                    Integration06IconBubble { CircleDot { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Atom { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Cpu { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Boxes { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Layers { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Package { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Code { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Database { class: "text-muted-foreground" } }
                                    Integration06IconBubble { Globe { class: "text-muted-foreground" } }
                                }
                            }
                        }
                        div { class: "flex absolute inset-0 gap-2 justify-center m-auto size-fit",
                            div { class: "flex relative z-20 rounded-full border shadow-xl shadow-black-950/10 size-16 bg-white/25 backdrop-blur-md backdrop-grayscale dark:border-white/10 dark:shadow-white/15",
                                div { class: "m-auto size-fit *:size-8",
                                    Search { class: "text-muted-foreground" }
                                }
                            }
                        }
                    }
                    div { class: "mx-auto mt-12 space-y-6 max-w-lg text-center",
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
