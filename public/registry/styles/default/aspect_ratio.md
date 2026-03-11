---
title: "Aspect Ratio"
name: "aspect_ratio"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/aspect_ratio.rs"
description: "Displays content within a desired ratio."
tags: []
---

# Aspect Ratio

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add aspect_ratio
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn AspectRatio(
    #[prop(default = 1.7777777777777777)] ratio: f64,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let style = format!("aspect-ratio: {ratio}");

    view! {
        <div data-name="AspectRatio" class=tw_merge!("relative w-full overflow-hidden", class) style=style>
            {children()}
        </div>
    }
}
```
