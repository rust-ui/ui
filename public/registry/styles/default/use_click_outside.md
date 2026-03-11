---
title: "Use Click Outside"
name: "use_click_outside"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_click_outside.rs"
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
use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Hook to detect clicks outside a referenced element.
///
/// Registers a document-level mousedown listener that calls the callback
/// when a click occurs outside the element referenced by `node_ref`.
///
/// # Example
/// ```ignore
/// let container_ref = NodeRef::<Div>::new();
/// use_click_outside(container_ref, move || {
///     // Called when clicking outside the container
///     my_signal.set(None);
/// });
/// ```
pub fn use_click_outside<F>(node_ref: NodeRef<Div>, on_click_outside: F)
where
    F: Fn() + Clone + 'static,
{
    Effect::new(move |_| {
        let callback = on_click_outside.clone();
        let handler =
            wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
                // Use try_get_untracked() since we're in an event handler, not a reactive context
                if let Some(Some(element)) = node_ref.try_get_untracked()
                    && let Some(target) = ev.target()
                    && let Ok(target_node) = target.dyn_into::<web_sys::Node>()
                    && !element.contains(Some(&target_node))
                {
                    callback();
                }
            });

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = document.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref());
        }

        // Keep the closure alive for the lifetime of the component
        handler.forget();
    });
}
```
