use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "border-t bg-background mt-16",
            div { class: "container mx-auto px-8 py-12 flex flex-col md:flex-row items-start justify-between gap-8",
                // Brand
                div { class: "flex flex-col gap-2 max-w-xs",
                    span { class: "font-semibold text-sm", "Dioxus UI" }
                    p { class: "text-sm text-muted-foreground leading-relaxed",
                        "A registry of reusable Dioxus components. Copy, paste, and customize."
                    }
                }
                // Links grid
                div { class: "grid grid-cols-2 gap-8 text-sm",
                    div { class: "flex flex-col gap-2",
                        p { class: "font-medium", "Components" }
                        a { class: "text-muted-foreground hover:text-foreground transition-colors", href: "/docs/components/button", "Button" }
                        a { class: "text-muted-foreground hover:text-foreground transition-colors", href: "/docs/components/card", "Card" }
                        a { class: "text-muted-foreground hover:text-foreground transition-colors", href: "/docs/components/input", "Input" }
                        a { class: "text-muted-foreground hover:text-foreground transition-colors", href: "/docs/components/badge", "Badge" }
                    }
                    div { class: "flex flex-col gap-2",
                        p { class: "font-medium", "Links" }
                        a {
                            class: "text-muted-foreground hover:text-foreground transition-colors",
                            href: "https://dioxuslabs.com",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Dioxus"
                        }
                        a {
                            class: "text-muted-foreground hover:text-foreground transition-colors",
                            href: "https://rust-ui.com",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Rust UI"
                        }
                        a {
                            class: "text-muted-foreground hover:text-foreground transition-colors",
                            href: "https://github.com/rust-ui/dioxus-ui",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "GitHub"
                        }
                    }
                }
            }
            div { class: "border-t",
                div { class: "container mx-auto px-8 py-4 flex items-center justify-between text-xs text-muted-foreground",
                    span { "© 2026 Dioxus UI. Built with Rust." }
                    a {
                        class: "hover:text-foreground transition-colors",
                        href: "https://rustify.rs",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Rustify"
                    }
                }
            }
        }
    }
}
