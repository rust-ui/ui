


```rust
use dioxus::prelude::*;
use icons::Search;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input::{Input, InputType};

const COMPONENT_LINKS: &[(&str, &str)] = &[
    ("/docs/components/accordion", "Accordion"),
    ("/docs/components/button", "Button"),
    ("/docs/components/card", "Card"),
    ("/docs/components/chips", "Chips"),
    ("/docs/components/dialog", "Dialog"),
    ("/docs/components/input", "Input"),
];
const HOOK_LINKS: &[(&str, &str)] = &[
    ("/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/docs/hooks/use-random", "Use Random"),
];
const BLOCK_LINKS: &[(&str, &str)] = &[
    ("/blocks/login", "Login"),
    ("/blocks/sidenav", "Sidenav"),
    ("/blocks/footers", "Footers"),
    ("/blocks/headers", "Headers"),
    ("/blocks/faq", "FAQ"),
];

#[component]
pub fn Sidenav02() -> Element {
    rsx! {
        div { class: "min-h-[760px] bg-muted/40",
            div { class: "mx-auto grid max-w-7xl gap-6 px-6 py-8 lg:grid-cols-[18rem_minmax(0,1fr)]",
                aside { class: "flex flex-col rounded-xl border bg-sidenav p-3 text-sidenav-foreground shadow-sm",
                    div { class: "space-y-2 border-b border-sidenav-border/70 pb-3",
                        div { class: "text-lg font-semibold", "Collapsible Menus" }
                        p { class: "text-xs text-sidenav-foreground/70", "A sidebar layout tuned for nested sections and quick filtering." }
                        div { class: "relative",
                            Input { r#type: InputType::Search, placeholder: "Filter links...", class: "h-8 pl-9 bg-background" }
                            Search { class: "pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" }
                        }
                    }
                    div { class: "flex flex-1 flex-col gap-4 py-3", /* same sections */
                        section { class: "space-y-2",
                            div { class: "px-2 text-xs font-medium uppercase tracking-wide text-sidenav-foreground/70", "Components" }
                            div { class: "space-y-1", for (href, label) in COMPONENT_LINKS { a { href: *href, class: "flex items-center gap-2 rounded-md px-2 py-2 text-sm hover:bg-sidenav-accent hover:text-sidenav-accent-foreground", span { class: "truncate", "{label}" } } } }
                        }
                        section { class: "space-y-2",
                            div { class: "px-2 text-xs font-medium uppercase tracking-wide text-sidenav-foreground/70", "Hooks" }
                            div { class: "space-y-1", for (href, label) in HOOK_LINKS { a { href: *href, class: "flex items-center gap-2 rounded-md px-2 py-2 text-sm hover:bg-sidenav-accent hover:text-sidenav-accent-foreground", span { class: "truncate", "{label}" } } } }
                        }
                        section { class: "space-y-2",
                            div { class: "px-2 text-xs font-medium uppercase tracking-wide text-sidenav-foreground/70", "Blocks" }
                            div { class: "space-y-1", for (href, label) in BLOCK_LINKS { a { href: *href, class: "flex items-center gap-2 rounded-md px-2 py-2 text-sm hover:bg-sidenav-accent hover:text-sidenav-accent-foreground", span { class: "truncate", "{label}" } } } }
                        }
                    }
                    div { class: "mt-auto border-t border-sidenav-border/70 pt-3 text-xs text-sidenav-foreground/70", "Menu state synced with the current section" }
                }
                main { class: "rounded-xl border bg-background p-6 shadow-sm",
                    div { class: "space-y-4",
                        div { class: "inline-flex items-center gap-2 rounded-full border px-3 py-1 text-xs text-muted-foreground", Search { class: "size-3.5" } "Demo" }
                        h1 { class: "text-3xl font-semibold tracking-tight", "Collapsible Menus" }
                        p { class: "max-w-prose text-sm text-muted-foreground", "A sidebar layout tuned for nested sections and quick filtering." }
                        div { class: "grid gap-4 md:grid-cols-2",
                            div { class: "rounded-lg border bg-card p-4", div { class: "text-sm font-medium", "Navigation" } p { class: "mt-2 text-sm text-muted-foreground", "This block shows the shell layout, sidebar links, and content framing used by the Leptos demo." } }
                            div { class: "rounded-lg border bg-card p-4", div { class: "text-sm font-medium", "Actions" } div { class: "mt-3 flex flex-wrap gap-2", Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Copy link" } Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Open docs" } } }
                        }
                    }
                }
            }
        }
    }
}
```