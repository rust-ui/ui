---
title: "Separator"
name: "separator"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/separator.rs"
description: "Rust/UI component that displays a separator line."
tags: []
---

# Separator

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add separator
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: Signal<SeparatorOrientation>,
    #[prop(into, optional)] class: String,
    // children: Children,
) -> impl IntoView {
    let merged_class = Memo::new(move |_| {
        let orientation = orientation.get();
        let separator = SeparatorClass { orientation };
        separator.with_class(class.clone())
    });

    view! { <div class=merged_class role="separator" /> }
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

#[derive(TwClass, Default)]
#[tw(class = "shrink-0 bg-border")]
pub struct SeparatorClass {
    orientation: SeparatorOrientation,
}

#[derive(TwVariant)]
pub enum SeparatorOrientation {
    #[tw(default, class = "w-full h-[1px]")]
    Default,
    #[tw(class = "h-full w-[1px]")]
    Vertical,
}
```
