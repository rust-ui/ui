---
title: "Use Data Scrolled"
name: "use_data_scrolled"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_data_scrolled.rs"
---

# Use Data Scrolled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_data_scrolled
```

## Component Code

```rust
use leptos::prelude::*;
use leptos::wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub const DATA_SCROLL_TARGET: &str = "data-scroll-target";

pub fn use_data_scrolled(threshold_px: u32) -> RwSignal<bool> {
    let is_data_scrolled_signal = RwSignal::new(false);

    Effect::new(move |_| {
        let threshold = f64::from(threshold_px);

        // Try to find the scroll target, fallback to window
        let scroll_container = window().document().and_then(|d| d.get_element_by_id(DATA_SCROLL_TARGET));

        let get_scroll_pos = {
            let container = scroll_container.clone();
            move || -> f64 { if let Some(ref el) = container { el.scroll_top() as f64 } else { get_scroll_position() } }
        };

        // Set initial value
        is_data_scrolled_signal.set(get_scroll_pos() > threshold);

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            is_data_scrolled_signal.set(get_scroll_pos() > threshold);
        }) as Box<dyn Fn(web_sys::Event)>);

        if let Some(el) = scroll_container {
            let _ = el.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        } else {
            let _ = window().add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }

        closure.forget();
    });

    is_data_scrolled_signal
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Synchronize body's scrollbar padding compensation to Header's fixed element.
/// When `use_lock_body_scroll_popover` applies padding-right to body, fixed elements need the same padding to prevent shifting.
fn sync_header_padding_with_body(padding: &str) {
    let _ = (|| -> Option<()> {
        let element = window().document()?.query_selector("[data-name='NavMenuFixed']").ok()??;
        let header_el = element.dyn_ref::<web_sys::HtmlElement>()?;

        if !padding.is_empty() && padding != "0px" {
            header_el.style().set_property("padding-right", padding).ok()?;
        } else {
            header_el.style().remove_property("padding-right").ok()?;
        }
        Some(())
    })();
}

/// Get scroll position, accounting for when `use_lock_body_scroll_popover` fixes the body with negative top offset.
fn get_scroll_position() -> f64 {
    let Some(body) = window().document().and_then(|d| d.body()) else {
        return window().scroll_y().unwrap_or(0.0);
    };

    let style = body.style();
    let is_fixed = style.get_property_value("position").ok() == Some("fixed".to_string());
    let padding = style.get_property_value("padding-right").unwrap_or_default();

    sync_header_padding_with_body(&padding);

    if is_fixed {
        style
            .get_property_value("top")
            .ok()
            .and_then(|top| top.strip_suffix("px")?.strip_prefix("-")?.parse().ok())
            .unwrap_or(0.0)
    } else {
        window().scroll_y().unwrap_or(0.0)
    }
}
```
