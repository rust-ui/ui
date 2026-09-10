


```rust
use dioxus::prelude::*;
use icons::Search;

use crate::components::ui::input::{Input, InputType};

#[component]
pub fn Sidenav08() -> Element {
    rsx! {
        div { class: "min-h-[760px] bg-muted/40",
            div { class: "mx-auto grid max-w-7xl gap-6 px-6 py-8 lg:grid-cols-[18rem_minmax(0,1fr)]",
                aside { class: "flex flex-col rounded-xl border bg-sidenav p-3 text-sidenav-foreground shadow-sm",
                    div { class: "space-y-2 border-b border-sidenav-border/70 pb-3",
                        div { class: "text-lg font-semibold", "Inset Secondary Navigation" }
                        p { class: "text-xs text-sidenav-foreground/70", "An inset layout that keeps the sidebar attached to the docs surface." }
                        div { class: "relative",
                            Input { r#type: InputType::Search, placeholder: "Search inset items...", class: "h-8 pl-9 bg-background" }
                            Search { class: "pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" }
                        }
                    }
                    div { class: "mt-auto border-t border-sidenav-border/70 pt-3 text-xs text-sidenav-foreground/70", "Inset secondary navigation" }
                }
                main { class: "rounded-xl border bg-background p-6 shadow-sm",
                    h1 { class: "text-3xl font-semibold tracking-tight", "Inset Secondary Navigation" }
                }
            }
        }
    }
}
```