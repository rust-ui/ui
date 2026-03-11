---
title: "Use Lock Body Scroll Popover"
name: "use_lock_body_scroll_popover"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_lock_body_scroll_popover.rs"
---

# Use Lock Body Scroll Popover

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_lock_body_scroll_popover
```

## Component Code

```rust
use leptos::prelude::*;

/// Hook to lock/unlock body scroll while preserving scroll position.
///
/// This prevents the page from scrolling when selects or modals are open,
/// maintaining the current scroll position by using fixed positioning with
/// a negative top offset.
///
/// When unlocking, it delays the restoration by 100ms to allow select
/// closing animations to complete, preventing visual flashing during the
/// transition where the page would briefly jump to the top.
///
/// # Arguments
/// * `initial_locked` - Whether the body scroll should be initially locked
///
/// # Returns
/// A reactive signal that controls the lock state - set to `true` to lock,
/// `false` to unlock with delayed restoration
pub fn use_lock_body_scroll_popover(initial_locked: bool) -> RwSignal<bool> {
    let locked_signal = RwSignal::new(initial_locked);
    let scroll_position_signal = RwSignal::new(0.0);

    Effect::new(move |_| {
        let Some(body) = window().document().and_then(|d| d.body()) else {
            return;
        };

        if locked_signal.get() {
            let current_scroll = window().scroll_y().unwrap_or(0.0);
            scroll_position_signal.set(current_scroll);

            let Some(inner_width) = window().inner_width().ok().and_then(|w| w.as_f64()) else {
                return;
            };
            let scrollbar_width = inner_width - body.client_width() as f64;
            let style = body.style();

            let _ = style.set_property("position", "fixed");
            let _ = style.set_property("top", &format!("-{current_scroll}px"));
            let _ = style.set_property("width", "100%");
            let _ = style.set_property("overflow", "hidden");

            if scrollbar_width > 0.0 {
                let _ = style.set_property("padding-right", &format!("{scrollbar_width}px"));
            }
        } else {
            let stored_position = scroll_position_signal.get();
            set_timeout(
                move || {
                    let style = body.style();
                    for prop in ["position", "top", "width", "overflow", "padding-right"] {
                        let _ = style.remove_property(prop);
                    }
                    window().scroll_to_with_x_and_y(0.0, stored_position);
                },
                std::time::Duration::from_millis(100),
            );
        }
    });

    locked_signal
}
```
