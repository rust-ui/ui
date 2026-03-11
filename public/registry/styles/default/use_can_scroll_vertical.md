---
title: "Use Can Scroll Vertical"
name: "use_can_scroll_vertical"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_can_scroll_vertical.rs"
---

# Use Can Scroll Vertical

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_can_scroll_vertical
```

## Component Code

```rust
use leptos::prelude::*;

/// Hook for detecting vertical scroll state of a scrollable element
///
/// Returns a tuple of (on_scroll_handler, can_scroll_up_signal, can_scroll_down_signal) where:
/// - `on_scroll_handler`: Event handler to attach to the scrollable element's `on:scroll`
/// - `can_scroll_up_signal`: RwSignal<bool> indicating if content is scrolled down (can scroll up)
/// - `can_scroll_down_signal`: RwSignal<bool> indicating if more content is below (can scroll down)
pub fn use_can_scroll_vertical() -> (impl Fn(web_sys::Event) + Clone, RwSignal<bool>, RwSignal<bool>) {
    let can_scroll_up_signal = RwSignal::new(false);
    let can_scroll_down_signal = RwSignal::new(false);

    let on_scroll = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlElement>(&ev);
        let scroll_top = target.scroll_top();
        let scroll_height = target.scroll_height();
        let client_height = target.client_height();

        can_scroll_up_signal.set(scroll_top > 0);
        can_scroll_down_signal.set(scroll_top < scroll_height - client_height - 1);
    };

    (on_scroll, can_scroll_up_signal, can_scroll_down_signal)
}
```
