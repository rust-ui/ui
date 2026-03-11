---
title: "Pressable"
name: "pressable"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/pressable.rs"
description: "Wrapper component that adds press feedback (scale effect) to any children."
tags: []
---

# Pressable

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add pressable
```

## Component Code

```rust
use leptos::prelude::*;

/// Wrapper component that adds press feedback (scale effect) to any children.
/// Works on mobile where `:active` pseudo-class doesn't work on non-button elements.
#[component]
pub fn Pressable(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let is_pressed_signal = RwSignal::new(false);

    let wrapper_class = move || {
        let base = format!("transition-transform {class}");
        if is_pressed_signal.get() { format!("{base} scale-[0.98]") } else { base }
    };

    view! {
        <div
            class=wrapper_class
            on:pointerdown=move |_| is_pressed_signal.set(true)
            on:pointerup=move |_| is_pressed_signal.set(false)
            on:pointerleave=move |_| is_pressed_signal.set(false)
            on:pointercancel=move |_| is_pressed_signal.set(false)
        >
            {children()}
        </div>
    }
}
```
