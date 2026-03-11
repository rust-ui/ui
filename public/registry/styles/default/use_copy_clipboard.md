---
title: "Use Copy Clipboard"
name: "use_copy_clipboard"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_copy_clipboard.rs"
---

# Use Copy Clipboard

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_copy_clipboard
```

## Component Code

```rust
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

const DEFAULT_TIMEOUT_MS: i32 = 2000;

/// Hook for copying text to clipboard with optional delay
///
/// Returns a tuple of (copy_fn, copied_signal) where:
/// - `copy_fn`: Function that takes text to copy
/// - `copied_signal`: ReadSignal<bool> indicating if text was recently copied
pub fn use_copy_clipboard(timeout_ms: Option<i32>) -> (impl Fn(&str) + Clone, ReadSignal<bool>) {
    let copied_signal = RwSignal::new(false);
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);

    let copy_to_clipboard = {
        move |text: &str| {
            if let Some(window) = window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let _ = clipboard.write_text(text);

                // Set copied state to true
                copied_signal.set(true);

                // Reset to false after timeout
                // Use try_set to avoid panic if component is unmounted before timeout fires
                let copied_clone = copied_signal;
                let closure = Closure::once_into_js(move || {
                    let _ = copied_clone.try_set(false);
                });
                let _ = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), timeout);
            }
        }
    };

    (copy_to_clipboard, copied_signal.read_only())
}
```
