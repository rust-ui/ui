---
title: "Use Horizontal Scroll"
name: "use_horizontal_scroll"
cargo_dependencies: ["strum", "wasm_bindgen"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_horizontal_scroll.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
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
use dioxus::prelude::*;
use strum::Display;

const DEFAULT_SCROLL_PERCENTAGE: f64 = 0.5;
const DEFAULT_UPDATE_DELAY_MS: i32 = 300;

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
    pub scroll_state: Signal<HorizontalScrollState>,
    pub scroll_by: Callback<i32>,
    pub on_scroll: Callback<Event<ScrollData>>,
}

pub fn use_horizontal_scroll(
    _element_signal: ReadSignal<Option<web_sys::Element>>,
    scroll_percentage: Option<f64>,
    update_delay_ms: Option<i32>,
) -> HorizontalScrollContext {
    let scroll_state = use_signal(HorizontalScrollState::default);
    let _scroll_pct = scroll_percentage.unwrap_or(DEFAULT_SCROLL_PERCENTAGE);
    let _delay_ms = update_delay_ms.unwrap_or(DEFAULT_UPDATE_DELAY_MS);

    let update_state = move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(el) = _element_signal.read().as_ref() {
                let scroll_left = el.scroll_left();
                let scroll_width = el.scroll_width();
                let client_width = el.client_width();

                let state = if scroll_left <= 0 {
                    HorizontalScrollState::Start
                } else if scroll_left >= scroll_width - client_width - 1 {
                    HorizontalScrollState::End
                } else {
                    HorizontalScrollState::Middle
                };
                *scroll_state.write_unchecked() = state;
            }
        }
    };

    let scroll_by = Callback::new(move |_direction: i32| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(el) = _element_signal.read().as_ref() {
                if let Ok(html_el) = el.clone().dyn_into::<web_sys::HtmlElement>() {
                    let container_width = html_el.client_width();
                    let scroll_amount = (container_width as f64 * _scroll_pct) as i32;
                    html_el.set_scroll_left(html_el.scroll_left() + (scroll_amount * _direction));

                    let closure = wasm_bindgen::closure::Closure::once_into_js(move || {
                        update_state();
                    });
                    let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        _delay_ms,
                    );
                }
            }
        }
    });

    let on_scroll = Callback::new(move |_ev: Event<ScrollData>| {
        update_state();
    });

    HorizontalScrollContext { scroll_state, scroll_by, on_scroll }
}
```
