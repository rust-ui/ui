---
title: "Demo Card Group"
name: "demo_card_group"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_card_group.rs"
---

# Demo Card Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card_group
```

## Component Code

```rust
use icons::{CircleAlert, Cloud, Search};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoCardGroup() -> impl IntoView {
    clx! {IconWrapper, div, "grid place-items-center bg-white rounded-xl ring-1 transition duration-500 group-hover:duration-200 size-12 ring-black/[0.08] group-hover:-translate-y-0.5 shadow-lg"}
    clx! {CardGroup, div, "group", "p-14 w-full text-center rounded-xl border transition duration-500 hover:duration-200 bg-accent hover:bg-secondary"}

    view! {
        <CardGroup class="max-w-[620px]">
            <div class="flex justify-center isolate">
                <IconWrapper class="relative top-1.5 left-2.5 -rotate-6 group-hover:-rotate-12 group-hover:-translate-x-5">
                    <Cloud class="size-4" />
                </IconWrapper>
                <IconWrapper class="z-10">
                    <Search class="size-4" />
                </IconWrapper>
                <IconWrapper class="relative top-1.5 right-2.5 rotate-6 group-hover:rotate-12 group-hover:translate-x-5">
                    <CircleAlert class="size-4" />
                </IconWrapper>
            </div>

            <h2 class="mt-6 text-base font-medium">"No Icons Found"</h2>
            <p class="mx-auto mt-1 text-sm text-muted-foreground max-w-[300px]">
                "You were searching for Icons in Rust/UI but none of them was found. Sorry!"
            </p>

            <Button variant=ButtonVariant::Outline class="mt-4" attr:href="/icons">
                "Go Back to Icons Page"
            </Button>
        </CardGroup>
    }
}
```
