---
title: "Use Horizontal Scroll"
name: "use_horizontal_scroll"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_horizontal_scroll.rs"
---

# Use Horizontal Scroll

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_horizontal_scroll
```

## Component Code

```rust
use leptos::html::Div;
use leptos::prelude::*;
use strum::Display;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, Event};

const DEFAULT_SCROLL_PERCENTAGE: f64 = 0.5;
const DEFAULT_UPDATE_DELAY_MS: u64 = 300;

#[derive(Default, Clone, Copy, Display, PartialEq, Debug)]
#[strum(serialize_all = "PascalCase")]
pub enum HorizontalScrollState {
    #[default]
    Start,
    Middle,
    End,
}

#[derive(Clone)]
pub struct HorizontalScrollContext {
    pub scroll_state: RwSignal<HorizontalScrollState>,
    pub scroll_by: Callback<i32>,
    pub on_scroll: Callback<Event>,
}

pub fn use_horizontal_scroll(
    node_ref: NodeRef<Div>,
    scroll_percentage: Option<f64>,
    update_delay_ms: Option<u64>,
) -> HorizontalScrollContext {
    let scroll_state_signal = RwSignal::new(HorizontalScrollState::default());
    let scroll_pct = scroll_percentage.unwrap_or(DEFAULT_SCROLL_PERCENTAGE);
    let delay_ms = update_delay_ms.unwrap_or(DEFAULT_UPDATE_DELAY_MS);

    let update_scroll_state = move || {
        if let Some(element) = node_ref.get() {
            let element: Element = element.unchecked_into();
            let scroll_left = element.scroll_left();
            let scroll_width = element.scroll_width();
            let client_width = element.client_width();

            let state = if scroll_left <= 0 {
                HorizontalScrollState::Start
            } else if scroll_left >= scroll_width - client_width - 1 {
                HorizontalScrollState::End
            } else {
                HorizontalScrollState::Middle
            };

            // Use try_set to avoid panic if component is unmounted before timeout fires
            let _ = scroll_state_signal.try_set(state);
        }
    };

    let scroll_by = Callback::new(move |direction: i32| {
        if let Some(element) = node_ref.get() {
            let element: Element = element.unchecked_into();
            let container_width = element.client_width();
            let scroll_amount = (container_width as f64 * scroll_pct) as i32;
            element.set_scroll_left(element.scroll_left() + (scroll_amount * direction));
            set_timeout(
                move || {
                    update_scroll_state();
                },
                std::time::Duration::from_millis(delay_ms),
            );
        }
    });

    let on_scroll = Callback::new(move |_: Event| {
        update_scroll_state();
    });

    HorizontalScrollContext { scroll_state: scroll_state_signal, scroll_by, on_scroll }
}
```
