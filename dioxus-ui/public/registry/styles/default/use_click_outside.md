---
title: "Use Click Outside"
name: "use_click_outside"
cargo_dependencies: ["wasm_bindgen"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_click_outside.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Click Outside

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_click_outside
```

## Component Code

```rust
use dioxus::prelude::*;

/// Registers a document-level mousedown listener that fires the callback
/// when a click occurs outside the element given by `element_signal`.
/// Set the signal from an `onmounted` handler.
pub fn use_click_outside<F>(_element_signal: ReadSignal<Option<web_sys::Element>>, _on_click_outside: F)
where
    F: Fn() + Clone + 'static,
{
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let callback = _on_click_outside.clone();
            let handler =
                wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
                    let Some(ref element) = *_element_signal.read() else { return };
                    let Some(target) = ev.target() else { return };
                    let Ok(target_node) = target.dyn_into::<web_sys::Node>() else { return };
                    if !element.contains(Some(&target_node)) {
                        callback();
                    }
                });

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ = document.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref());
            }

            handler.forget();
        }
    });
}
```
