---
title: "Use Can Scroll"
name: "use_can_scroll"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_can_scroll.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Can Scroll

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_can_scroll
```

## Component Code

```rust
use dioxus::prelude::*;

/// Returns (update_fn, show_left, show_right).
/// Call update_fn on scroll events of the target element.
pub fn use_can_scroll() -> (impl Fn(Event<ScrollData>) + Clone, ReadSignal<bool>, ReadSignal<bool>) {
    let show_left = use_signal(|| false);
    let show_right = use_signal(|| true);

    let on_scroll = move |_ev: Event<ScrollData>| {
        let scroll_left = _ev.scroll_left();
        let scroll_width = _ev.scroll_width() as f64;
        let client_width = _ev.client_width() as f64;
        *show_left.write_unchecked() = scroll_left > 0.0;
        *show_right.write_unchecked() = scroll_left < scroll_width - client_width - 1.0;
    };

    (on_scroll, show_left.into(), show_right.into())
}
```
