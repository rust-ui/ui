---
title: "Use Lock Body Scroll"
name: "use_lock_body_scroll"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_lock_body_scroll.rs"
---

# Use Lock Body Scroll

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_lock_body_scroll
```

## Component Code

```rust
use leptos::prelude::*;

pub fn use_lock_body_scroll(initial_locked: bool) -> RwSignal<bool> {
    let locked_signal = RwSignal::new(initial_locked);

    Effect::new(move |_| {
        if let Some(body) = window().document().and_then(|d| d.body()) {
            let overflow = if locked_signal.get() { "hidden" } else { "" };
            let _ = body.style().set_property("overflow", overflow);
        }
    });

    locked_signal
}
```
