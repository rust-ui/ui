---
title: "Use Media Query"
name: "use_media_query"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_media_query.rs"
---

# Use Media Query

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_media_query
```

## Component Code

```rust
use leptos::prelude::*;
use leptos::wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

/// Reactive hook that tracks whether a CSS media query matches.
///
/// Sets up a `MediaQueryList` change listener so the returned signal
/// updates automatically when the viewport changes.
///
/// # Example
/// ```ignore
/// let is_wide = use_media_query("(min-width: 1024px)");
///
/// view! {
///     {move || if is_wide.get() { "Wide layout" } else { "Narrow layout" }}
/// }
/// ```
pub fn use_media_query(query: &str) -> Signal<bool> {
    let is_match = RwSignal::new(false);
    let query = query.to_string();

    Effect::new(move |_| {
        let Some(mql) = window().match_media(&query).ok().flatten() else {
            return;
        };

        is_match.set(mql.matches());

        let mql_clone = mql.clone();
        let closure = Closure::<dyn Fn(JsValue)>::new(move |_: JsValue| {
            is_match.set(mql_clone.matches());
        });

        let _ = mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());

        closure.forget();
    });

    is_match.into()
}
```
