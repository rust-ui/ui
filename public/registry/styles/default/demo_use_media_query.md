---
title: "Demo Use Media Query"
name: "demo_use_media_query"
cargo_dependencies: []
registry_dependencies: ["badge"]
type: "components:demos"
path: "demos/demo_use_media_query.rs"
---

# Demo Use Media Query

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_media_query
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::hooks::use_media_query::use_media_query;
use crate::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoUseMediaQuery() -> impl IntoView {
    let is_md = use_media_query("(min-width: 768px)");
    let is_lg = use_media_query("(min-width: 1024px)");

    let md_variant = Signal::derive(move || if is_md.get() { BadgeVariant::Default } else { BadgeVariant::Secondary });
    let lg_variant = Signal::derive(move || if is_lg.get() { BadgeVariant::Default } else { BadgeVariant::Secondary });

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div class="flex gap-2 items-center">
                <span class="text-sm text-muted-foreground">"≥ 768px (md)"</span>
                <Badge variant=md_variant>{move || if is_md.get() { "matches" } else { "no match" }}</Badge>
            </div>
            <div class="flex gap-2 items-center">
                <span class="text-sm text-muted-foreground">"≥ 1024px (lg)"</span>
                <Badge variant=lg_variant>{move || if is_lg.get() { "matches" } else { "no match" }}</Badge>
            </div>
            <p class="text-xs text-muted-foreground">"Resize the window to see the signals update."</p>
        </div>
    }
}
```
