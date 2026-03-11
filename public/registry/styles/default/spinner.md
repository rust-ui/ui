---
title: "Spinner"
name: "spinner"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/spinner.rs"
description: "A loading spinner component with animation for indicating processing states."
tags: ["animation", "utils"]
---

# Spinner

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add spinner
```

## Component Code

```rust
use icons::{Loader, LoaderCircle};
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Spinner(#[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("size-4 animate-spin", class);

    view! { <Loader class=merged_class attr:role="status" attr:aria-label="Loading" /> }
}

#[component]
pub fn SpinnerCircle(#[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("size-4 animate-spin", class);

    view! { <LoaderCircle class=merged_class attr:role="status" attr:aria-label="Loading" /> }
}
```
