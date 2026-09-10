


```rust
use dioxus::prelude::*;
use icons::Search;

use crate::components::ui::input::{Input, InputType};

const LINKS: &[(&str, &str)] = &[
    ("/docs/components/accordion", "Accordion"),
    ("/docs/components/button", "Button"),
    ("/docs/components/card", "Card"),
    ("/docs/components/chips", "Chips"),
    ("/docs/components/dialog", "Dialog"),
    ("/docs/components/input", "Input"),
];

#[component]
pub fn Sidenav03() -> Element {
    rsx! {
        div { class: "min-h-[760px] bg-muted/40",
            div { class: "mx-auto grid max-w-7xl gap-6 px-6 py-8 lg:grid-cols-[18rem_minmax(0,1fr)]",
                aside { class: "flex flex-col rounded-xl border bg-sidenav p-3 text-sidenav-foreground shadow-sm",
                    div { class: "space-y-2 border-b border-sidenav-border/70 pb-3",
                        div { class: "text-lg font-semibold", "Submenus" }
                        p { class: "text-xs text-sidenav-foreground/70", "A longer navigation rail with deeper section groupings." }
                        div { class: "relative",
                            Input { r#type: InputType::Search, placeholder: "Search submenus...", class: "h-8 pl-9 bg-background" }
                            Search { class: "pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" }
                        }
                    }
                    div { class: "flex flex-1 flex-col gap-4 py-3",
                        section { class: "space-y-2",
                            div { class: "px-2 text-xs font-medium uppercase tracking-wide text-sidenav-foreground/70", "Components" }
                            div { class: "space-y-1", for (href, label) in LINKS { a { href: *href, class: "flex items-center gap-2 rounded-md px-2 py-2 text-sm hover:bg-sidenav-accent hover:text-sidenav-accent-foreground", span { class: "truncate", "{label}" } } } }
                        }
                    }
                    div { class: "mt-auto border-t border-sidenav-border/70 pt-3 text-xs text-sidenav-foreground/70", "Submenus stay collapsed until needed" }
                }
                main { class: "rounded-xl border bg-background p-6 shadow-sm",
                    h1 { class: "text-3xl font-semibold tracking-tight", "Submenus" }
                }
            }
        }
    }
}
```