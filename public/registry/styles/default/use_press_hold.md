---
title: "Use Press Hold"
name: "use_press_hold"
cargo_dependencies: ["wasm_bindgen"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_press_hold.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Press Hold

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_press_hold
```

## Component Code

```rust
use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

#[derive(Clone)]
#[allow(dead_code)]
pub struct UsePressHold {
    pub progress_signal: Signal<f64>,
    pub is_holding_signal: Signal<bool>,
    interval_id: Rc<Cell<Option<i32>>>,
    last_update: Rc<Cell<f64>>,
    duration: f64,
    on_complete: Callback<()>,
    disabled: bool,
}

impl UsePressHold {
    fn clear_interval(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(id) = self.interval_id.get() {
            web_sys::window().unwrap().clear_interval_with_handle(id);
            self.interval_id.set(None);
        }
    }

    pub fn on_pointer_down(&self) {
        if self.disabled {
            return;
        }

        self.clear_interval();
        let mut is_holding = self.is_holding_signal;
        is_holding.set(true);
        #[cfg(target_arch = "wasm32")]
        self.last_update.set(js_sys::Date::now());

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let progress = self.progress_signal;
            let is_holding = self.is_holding_signal;
            let interval_id = Rc::clone(&self.interval_id);
            let last_update = Rc::clone(&self.last_update);
            let duration = self.duration;
            let on_complete = self.on_complete;

            let callback = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                let now = js_sys::Date::now();
                let delta = now - last_update.get();
                last_update.set(now);

                let progress_delta = delta / duration;
                let new_progress = (progress.read().clone() + progress_delta).min(1.0);
                *progress.write_unchecked() = new_progress;

                if new_progress >= 1.0 {
                    on_complete.call(());
                    if let Some(id) = interval_id.get() {
                        web_sys::window().unwrap().clear_interval_with_handle(id);
                    }
                    interval_id.set(None);
                    *is_holding.write_unchecked() = false;
                    *progress.write_unchecked() = 0.0;
                }
            });

            if let Ok(id) = web_sys::window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 16)
            {
                self.interval_id.set(Some(id));
            }

            callback.forget();
        }
    }

    pub fn on_pointer_up(&self) {
        self.clear_interval();
        let mut is_holding = self.is_holding_signal;
        is_holding.set(false);

        if *self.progress_signal.read() <= 0.0 {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        self.last_update.set(js_sys::Date::now());

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let progress = self.progress_signal;
            let is_holding = self.is_holding_signal;
            let interval_id = Rc::clone(&self.interval_id);
            let last_update = Rc::clone(&self.last_update);
            let duration = self.duration;

            let callback = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                if *is_holding.read() {
                    return;
                }

                let now = js_sys::Date::now();
                let delta = now - last_update.get();
                last_update.set(now);

                let progress_delta = delta / duration;
                let new_progress = (*progress.read() - progress_delta).max(0.0);
                *progress.write_unchecked() = new_progress;

                if new_progress <= 0.0 {
                    if let Some(id) = interval_id.get() {
                        web_sys::window().unwrap().clear_interval_with_handle(id);
                    }
                    interval_id.set(None);
                }
            });

            if let Ok(id) = web_sys::window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 16)
            {
                self.interval_id.set(Some(id));
            }

            callback.forget();
        }
    }
}

pub fn use_press_hold(duration_ms: u32, on_complete: Callback<()>, disabled: bool) -> UsePressHold {
    UsePressHold {
        progress_signal: use_signal(|| 0.0),
        is_holding_signal: use_signal(|| false),
        interval_id: Rc::new(Cell::new(None)),
        last_update: Rc::new(Cell::new(0.0)),
        duration: duration_ms as f64,
        on_complete,
        disabled,
    }
}
```
