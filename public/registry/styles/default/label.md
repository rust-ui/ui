---
title: "Label"
name: "label"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/label.rs"
description: "Rust/UI component that displays a label for an input field."
tags: []
---

# Label

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add label
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] html_for: String,
    children: Children,
) -> impl IntoView {
    let peer_classes = if !html_for.is_empty() {
        format!("peer-disabled/{}:cursor-not-allowed peer-disabled/{}:opacity-50", html_for, html_for)
    } else {
        "peer-disabled:cursor-not-allowed peer-disabled:opacity-50".to_string()
    };

    let class = tw_merge!(
        "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50",
        &peer_classes,
        class
    );

    view! {
        <label class=class r#for=html_for>
            {children()}
        </label>
    }
}
```
