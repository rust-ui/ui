---
title: "Slider"
name: "slider"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/slider.rs"
description: "Rust/UI component that allows users to select a value from a range."
tags: []
---

# Slider

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add slider
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, strum::AsRefStr, strum::Display)]
pub enum SliderVariant {
    #[default]
    Round,
    Flat,
}

#[component]
pub fn Slider(
    #[prop(optional, into)] class: String,
    #[prop(default = SliderVariant::default())] variant: SliderVariant,
) -> impl IntoView {
    let variant_attr = variant.to_string();

    let merged_class = tw_merge!(
        "overflow-hidden relative bg-transparent transition-all duration-100 ease-in-out appearance-none disabled:opacity-30 disabled:cursor-not-allowed text-[1.5rem] w-[12.5em] text-primary active:cursor-grabbing disabled:grayscale",
        class
    );

    view! { <input data-name="Slider" data-variant=variant_attr type="range" class=merged_class /> }
}
```
