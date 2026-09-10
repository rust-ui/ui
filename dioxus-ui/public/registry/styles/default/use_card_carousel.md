---
title: "Use Card Carousel"
name: "use_card_carousel"
cargo_dependencies: ["wasm_bindgen", "web_sys"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_card_carousel.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Card Carousel

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_card_carousel
```

## Component Code

```rust
use std::cell::RefCell;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Event, EventTarget};

const CAROUSEL_ROOT: &str = r#"[data-name="CardCarousel"]"#;
const CAROUSEL_TRACK: &str = r#"[data-name="CardCarouselTrack"]"#;
const CAROUSEL_NAV_BUTTON: &str = r#"[data-name="CardCarouselNavButton"]"#;
const CAROUSEL_INDICATOR: &str = r#"[data-name="CardCarouselIndicator"]"#;

thread_local! {
    static LISTENERS: RefCell<Option<Listeners>> = const { RefCell::new(None) };
}

struct Listeners {
    _click: Closure<dyn FnMut(Event)>,
    _scroll: Closure<dyn FnMut(Event)>,
}

/// Register delegated event listeners on `document` for all `CardCarousel`
/// instances on the page. Safe to call multiple times — subsequent calls are no-ops.
pub fn init() {
    LISTENERS.with(|cell| {
        if cell.borrow().is_some() {
            return;
        }
        if let Some(listeners) = setup_listeners() {
            *cell.borrow_mut() = Some(listeners);
        }
    });
}

fn document() -> Option<web_sys::Document> {
    web_sys::window().and_then(|w| w.document())
}

// ── Setup ─────────────────────────────────────────────────────────────────────

fn setup_listeners() -> Option<Listeners> {
    let target: EventTarget = document()?.unchecked_into();

    let click_cb = Closure::new(handle_click);
    let scroll_cb = Closure::new(handle_scroll);

    let _ = target.add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref());
    // Capture phase: scroll events on overflow-scroll track don't bubble.
    let _ = target.add_event_listener_with_callback_and_bool("scroll", scroll_cb.as_ref().unchecked_ref(), true);

    Some(Listeners { _click: click_cb, _scroll: scroll_cb })
}

// ── Click handler ─────────────────────────────────────────────────────────────

fn handle_click(event: Event) {
    let Some(target) = event.target() else { return };
    let Ok(el) = target.dyn_into::<Element>() else { return };
    let Some(btn) = el.closest(CAROUSEL_NAV_BUTTON).ok().flatten() else { return };

    // Prevent navigation when NavButton is inside an <a> tag.
    event.stop_propagation();
    event.prevent_default();

    let Some(root) = btn.closest(CAROUSEL_ROOT).ok().flatten() else { return };
    let Some(track) = root.query_selector(CAROUSEL_TRACK).ok().flatten() else { return };
    let Ok(buttons) = root.query_selector_all(CAROUSEL_NAV_BUTTON) else { return };

    let is_prev = buttons.item(0).and_then(|n| n.dyn_into::<Element>().ok()).is_some_and(|first| first == btn);

    let delta = f64::from(track.client_width()) * if is_prev { -1.0 } else { 1.0 };
    // No explicit behavior — CSS scroll-smooth on the track handles the animation,
    // and avoids a known WebKit bug where behavior:'smooth' breaks snap-mandatory.
    track.scroll_by_with_x_and_y(delta, 0.0);
}

// ── Scroll handler ────────────────────────────────────────────────────────────

fn handle_scroll(event: Event) {
    let Some(target) = event.target() else { return };
    let Ok(el) = target.dyn_into::<Element>() else { return };
    let Some(track) = el.closest(CAROUSEL_TRACK).ok().flatten() else { return };
    let Some(root) = track.closest(CAROUSEL_ROOT).ok().flatten() else { return };

    let Ok(indicators) = root.query_selector_all(CAROUSEL_INDICATOR) else { return };
    let Ok(buttons) = root.query_selector_all(CAROUSEL_NAV_BUTTON) else { return };

    let client_width = track.client_width();
    let index =
        if client_width > 0 { (f64::from(track.scroll_left()) / f64::from(client_width)).round() as u32 } else { 0 };

    let count = indicators.length();

    for i in 0..count {
        let Some(node) = indicators.item(i) else { continue };
        let Ok(dot) = node.dyn_into::<Element>() else { continue };
        if i == index {
            let _ = dot.set_attribute("aria-current", "true");
        } else {
            let _ = dot.remove_attribute("aria-current");
        }
    }

    set_aria_disabled(buttons.item(0), index == 0);
    set_aria_disabled(buttons.item(1), count > 0 && index >= count - 1);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn set_aria_disabled(node: Option<web_sys::Node>, disabled: bool) {
    let Some(node) = node else { return };
    let Ok(el) = node.dyn_into::<Element>() else { return };
    if disabled {
        let _ = el.set_attribute("aria-disabled", "true");
    } else {
        let _ = el.remove_attribute("aria-disabled");
    }
}
```
