---
title: "Use Scroll Lock"
name: "use_scroll_lock"
cargo_dependencies: ["wasm_bindgen"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_scroll_lock.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Scroll Lock

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_scroll_lock
```

## Component Code

```rust
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
const EXCLUDED_DATA_NAMES: &[&str] =
    &["ScrollArea", "CommandList", "SelectContent", "MultiSelectContent", "DropdownMenuContent", "ContextMenuContent"];

#[cfg(target_arch = "wasm32")]
const FIXED_EXCLUDED: &[&str] = &["header", "nav", "aside"];

#[allow(dead_code)]
const SCROLLABLE_SELECTOR: &str =
    r#"body, [data-name="ScrollArea"], [data-radix-scroll-area-viewport], [data-scroll-area-viewport]"#;

#[cfg(target_arch = "wasm32")]
struct BodyStyles {
    position: String,
    top: String,
    width: String,
    overflow: String,
    padding_right: String,
}

#[cfg(target_arch = "wasm32")]
struct ScrollableEntry {
    element: web_sys::HtmlElement,
    scroll_top: i32,
    overflow: String,
    overflow_y: String,
    padding_right: String,
}

#[cfg(target_arch = "wasm32")]
struct FixedEntry {
    element: web_sys::HtmlElement,
    padding_right: String,
}

#[cfg(target_arch = "wasm32")]
struct State {
    locked: bool,
    window_scroll_y: f64,
    body_styles: Option<BodyStyles>,
    scrollable: Vec<ScrollableEntry>,
    fixed: Vec<FixedEntry>,
}

#[cfg(target_arch = "wasm32")]
impl State {
    const fn new() -> Self {
        Self { locked: false, window_scroll_y: 0.0, body_styles: None, scrollable: vec![], fixed: vec![] }
    }

    fn clear(&mut self) {
        self.locked = false;
        self.body_styles = None;
        self.scrollable.clear();
        self.fixed.clear();
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static STATE: RefCell<State> = const { RefCell::new(State::new()) };
}

#[cfg(target_arch = "wasm32")]
fn set_style(style: &web_sys::CssStyleDeclaration, prop: &str, val: &str) {
    if val.is_empty() {
        let _ = style.remove_property(prop);
    } else {
        let _ = style.set_property(prop, val);
    }
}

#[cfg(target_arch = "wasm32")]
fn parse_px(s: &str) -> f64 {
    s.trim_end_matches("px").parse::<f64>().unwrap_or(0.0)
}

/// Register `window.ScrollLock` for JS interop.
///
/// Call once at app startup. Subsequent calls are no-ops.
pub fn init() {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = web_sys::window() else { return };
        let window_js: &JsValue = window.as_ref();

        if js_sys::Reflect::get(window_js, &"ScrollLock".into())
            .ok()
            .filter(|v| !v.is_undefined() && !v.is_null())
            .is_some()
        {
            return;
        }

        let obj = js_sys::Object::new();
        let obj_js: &JsValue = obj.as_ref();

        let lock_closure = Closure::wrap(Box::new(lock) as Box<dyn Fn()>);
        let _ = js_sys::Reflect::set(obj_js, &"lock".into(), lock_closure.as_ref());
        lock_closure.forget();

        let unlock_closure = Closure::wrap(Box::new(|delay: JsValue| {
            unlock(delay.as_f64().unwrap_or(0.0) as u32);
        }) as Box<dyn Fn(JsValue)>);
        let _ = js_sys::Reflect::set(obj_js, &"unlock".into(), unlock_closure.as_ref());
        unlock_closure.forget();

        let is_locked_closure = Closure::wrap(Box::new(is_locked) as Box<dyn Fn() -> bool>);
        let _ = js_sys::Reflect::set(obj_js, &"isLocked".into(), is_locked_closure.as_ref());
        is_locked_closure.forget();

        let _ = js_sys::Reflect::set(window_js, &"ScrollLock".into(), obj_js);
    }
}

#[cfg(target_arch = "wasm32")]
fn is_excluded(el: &web_sys::Element) -> bool {
    if let Some(name) = el.get_attribute("data-name")
        && EXCLUDED_DATA_NAMES.iter().any(|&n| n == name)
    {
        return true;
    }
    false
}

#[cfg(target_arch = "wasm32")]
fn is_fixed_excluded(el: &web_sys::Element) -> bool {
    for &name in FIXED_EXCLUDED {
        let sel = format!(r#"[data-name="{name}"]"#);
        if el.closest(&sel).ok().flatten().is_some() {
            return true;
        }
    }
    false
}

/// Lock scrolling on body and all scrollable containers.
pub fn lock() {
    #[cfg(target_arch = "wasm32")]
    {
        let proceed = STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.locked {
                return false;
            }
            s.locked = true;
            true
        });
        if !proceed {
            return;
        }

        let Some(window) = web_sys::window() else { return };
        let Some(document) = window.document() else { return };
        let Some(body) = document.body() else { return };

        let window_scroll_y = window.scroll_y().unwrap_or(0.0);
        let inner_width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
        let scrollbar_width = inner_width - body.client_width() as f64;

        let body_style = body.style();
        let original_body = BodyStyles {
            position: body_style.get_property_value("position").unwrap_or_default(),
            top: body_style.get_property_value("top").unwrap_or_default(),
            width: body_style.get_property_value("width").unwrap_or_default(),
            overflow: body_style.get_property_value("overflow").unwrap_or_default(),
            padding_right: body_style.get_property_value("padding-right").unwrap_or_default(),
        };

        let _ = body_style.set_property("position", "fixed");
        let _ = body_style.set_property("top", &format!("-{window_scroll_y}px"));
        let _ = body_style.set_property("width", "100%");
        let _ = body_style.set_property("overflow", "hidden");

        if scrollbar_width > 0.0 {
            let _ = body_style.set_property("padding-right", &format!("{scrollbar_width}px"));
        }

        let mut s_entries: Vec<ScrollableEntry> = Vec::new();

        if let Ok(nodes) = document.query_selector_all(SCROLLABLE_SELECTOR) {
            for i in 0..nodes.length() {
                let Some(node) = nodes.get(i) else { continue };
                let Ok(element) = node.dyn_into::<web_sys::Element>() else { continue };
                if is_excluded(&element) {
                    continue;
                }
                let Ok(el) = element.clone().dyn_into::<web_sys::HtmlElement>() else { continue };
                let st = el.style();
                s_entries.push(ScrollableEntry {
                    scroll_top: el.scroll_top(),
                    overflow: st.get_property_value("overflow").unwrap_or_default(),
                    overflow_y: st.get_property_value("overflow-y").unwrap_or_default(),
                    padding_right: st.get_property_value("padding-right").unwrap_or_default(),
                    element: el,
                });
            }
        }

        let mut f_entries: Vec<FixedEntry> = Vec::new();
        if let Ok(nodes) = document.query_selector_all("header,nav,aside,[role=\"dialog\"],[role=\"alertdialog\"]") {
            for i in 0..nodes.length() {
                let Some(node) = nodes.get(i) else { continue };
                let Ok(element) = node.dyn_into::<web_sys::Element>() else { continue };
                if is_fixed_excluded(&element) {
                    continue;
                }
                let Ok(el) = element.clone().dyn_into::<web_sys::HtmlElement>() else { continue };
                let st = el.style();
                let padding_right = st.get_property_value("padding-right").unwrap_or_default();
                let cp = document
                    .default_view()
                    .and_then(|w| w.get_computed_style(&element).ok().flatten())
                    .and_then(|cs| cs.get_property_value("padding-right").ok())
                    .map(|p| parse_px(&p))
                    .unwrap_or(0.0);
                if scrollbar_width > 0.0 {
                    let np = cp + scrollbar_width;
                    let _ = el.style().set_property("padding-right", &format!("{np}px"));
                }
                f_entries.push(FixedEntry { element: el, padding_right });
            }
        }

        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.window_scroll_y = window_scroll_y;
            s.body_styles = Some(original_body);
            s.scrollable = s_entries;
            s.fixed = f_entries;
        });
    }
}

/// Unlock scrolling, optionally after a delay in milliseconds.
pub fn unlock(_delay_ms: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let locked = STATE.with(|s| s.borrow().locked);
        if !locked {
            return;
        }

        if _delay_ms > 0 {
            let closure = Closure::once_into_js(move || {
                perform_unlock();
            });
            let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                _delay_ms as i32,
            );
        } else {
            perform_unlock();
        }
    }
}

pub fn is_locked() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        STATE.with(|s| s.borrow().locked)
    }
    #[cfg(not(target_arch = "wasm32"))]
    false
}

#[cfg(target_arch = "wasm32")]
fn perform_unlock() {
    let Some(window) = web_sys::window() else { return };

    STATE.with(|state| {
        let mut s = state.borrow_mut();

        if let Some(body) = window.document().and_then(|d| d.body()) {
            if let Some(ref orig) = s.body_styles {
                let st = body.style();
                set_style(&st, "position", &orig.position);
                set_style(&st, "top", &orig.top);
                set_style(&st, "width", &orig.width);
                set_style(&st, "overflow", &orig.overflow);
                set_style(&st, "padding-right", &orig.padding_right);
            }
        }

        window.scroll_to_with_x_and_y(0.0, s.window_scroll_y);

        for entry in &s.scrollable {
            let st = entry.element.style();
            set_style(&st, "overflow", &entry.overflow);
            set_style(&st, "overflow-y", &entry.overflow_y);
            set_style(&st, "padding-right", &entry.padding_right);
            entry.element.set_scroll_top(entry.scroll_top);
        }

        for entry in &s.fixed {
            set_style(&entry.element.style(), "padding-right", &entry.padding_right);
        }

        s.clear();
    });
}
```
