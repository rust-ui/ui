---
title: "Demo Use Horizontal Scroll"
name: "demo_use_horizontal_scroll"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_use_horizontal_scroll.rs"
---

# Demo Use Horizontal Scroll

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_horizontal_scroll
```

## Component Code

```rust
use icons::{ChevronLeft, ChevronRight};
use leptos::html::Div;
use leptos::prelude::*;

use crate::components::hooks::use_horizontal_scroll::{HorizontalScrollState, use_horizontal_scroll};

#[component]
pub fn DemoUseHorizontalScroll() -> impl IntoView {
    let scroll_container_ref = NodeRef::<Div>::new();
    let scroll_ctx = use_horizontal_scroll(scroll_container_ref, None, None);

    view! {
        <div class="w-full">
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-semibold">"Horizontal Scroll Demo"</h3>
                <div class="flex gap-2">
                    <button
                        class="flex justify-center items-center rounded-full border disabled:opacity-50 disabled:cursor-not-allowed size-8 bg-secondary"
                        disabled=move || scroll_ctx.scroll_state.get() == HorizontalScrollState::Start
                        on:click=move |_| scroll_ctx.scroll_by.run(-1)
                    >
                        <ChevronLeft class="size-4" />
                    </button>
                    <button
                        class="flex justify-center items-center rounded-full border disabled:opacity-50 disabled:cursor-not-allowed size-8 bg-secondary"
                        disabled=move || scroll_ctx.scroll_state.get() == HorizontalScrollState::End
                        on:click=move |_| scroll_ctx.scroll_by.run(1)
                    >
                        <ChevronRight class="size-4" />
                    </button>
                </div>
            </div>

            <div
                node_ref=scroll_container_ref
                on:scroll=move |e| scroll_ctx.on_scroll.run(e)
                class="flex overflow-x-scroll gap-4 snap-x snap-mandatory scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
            >
                {(0..6)
                    .map(|i| {
                        view! {
                            <div class="flex justify-center items-center w-64 h-40 bg-gray-300 rounded-lg shrink-0 snap-start">
                                <span class="text-2xl font-bold text-gray-600">{format!("Card {}", i + 1)}</span>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <div class="mt-4 text-sm text-muted-foreground">
                "Scroll state: " <span class="font-semibold">{move || scroll_ctx.scroll_state.get().to_string()}</span>
            </div>
        </div>
    }
}
```
