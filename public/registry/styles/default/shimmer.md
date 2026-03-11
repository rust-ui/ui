---
title: "Shimmer"
name: "shimmer"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/shimmer.rs"
description: "Auto-adapting skeleton loader that mirrors your DOM structure."
tags: []
---

# Shimmer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add shimmer
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id_for;

#[component]
pub fn Shimmer(
    /// Controls shimmer visibility (works with any bool signal)
    #[prop(into)]
    loading: Signal<bool>,

    /// Color of the shimmer wave (default: "rgba(255,255,255,0.15)")
    #[prop(into, optional)]
    shimmer_color: Option<String>,

    /// Background color of shimmer blocks (default: "rgba(255,255,255,0.08)")
    #[prop(into, optional)]
    background_color: Option<String>,

    /// Animation duration in seconds (default: 1.5)
    #[prop(optional)]
    duration: Option<f64>,

    /// Fallback border-radius for text elements in px (default: 4)
    #[prop(optional)]
    fallback_border_radius: Option<f64>,

    /// Additional classes
    #[prop(into, optional)]
    class: String,

    /// Children to wrap
    children: Children,
) -> impl IntoView {
    let shimmer_id = use_random_id_for("Shimmer");
    let merged_class = tw_merge!("relative", class);

    view! {
        <div
            id=shimmer_id
            class=merged_class
            data-name="Shimmer"
            data-shimmer-loading=move || loading.get().to_string()
            data-shimmer-color=shimmer_color
            data-shimmer-bg-color=background_color
            data-shimmer-duration=duration.map(|d| d.to_string())
            data-shimmer-fallback-radius=fallback_border_radius.map(|r| r.to_string())
        >
            {children()}
        </div>
    }
}
```
