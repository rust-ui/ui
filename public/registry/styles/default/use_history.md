---
title: "Use History"
name: "use_history"
cargo_dependencies: ["wasm_bindgen", "web_sys"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_history.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use History

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_history
```

## Component Code

```rust
use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::KeyboardEvent;

/// Undo/redo history stack for URL-based state.
///
/// Tracks a list of URL strings and navigates between them using
/// `history.replaceState` — no new browser history entries are created.
///
/// # Usage
/// Call `UseHistory::init()` once at the top of your page component,
/// then access it anywhere in the tree via `use_history()`.
///
/// ```ignore
/// // In page component:
/// UseHistory::init();
///
/// // In child component:
/// let history = use_history();
/// history.push("?color=red".to_string());
/// ```
#[derive(Clone, Copy)]
pub struct UseHistory {
    history: Signal<Vec<String>>,
    index: Signal<usize>,
    is_navigating: Signal<bool>,
}

impl UseHistory {
    /// Initialize the history stack and provide it as context.
    /// Sets up `⌘Z` / `⌘⇧Z` / `⌃Y` keyboard shortcuts on the document.
    #[must_use]
    pub fn init() -> Self {
        let hook = Self { history: use_signal(Vec::new), index: use_signal(|| 0), is_navigating: use_signal(|| false) };

        provide_context(hook);

        // Seed the stack with the current query string
        use_effect(move || {
            #[cfg(target_arch = "wasm32")]
            {
                let search = web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default();
                let mut history = hook.history;
                history.with_mut(|h| h.push(search));
            }
        });

        // Register ⌘Z / ⌘⇧Z / ⌃Y shortcuts
        use_effect(move || {
            #[cfg(target_arch = "wasm32")]
            {
                let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
                    let key = e.key().to_lowercase();
                    let meta = e.meta_key() || e.ctrl_key();
                    let shift = e.shift_key();

                    // Skip if focus is in an input / textarea / select
                    if let Some(target) = e.target()
                        && let Some(el) = target.dyn_ref::<web_sys::HtmlElement>()
                    {
                        let tag = el.tag_name().to_lowercase();
                        if matches!(tag.as_str(), "input" | "textarea" | "select") {
                            return;
                        }
                    }

                    if meta && key == "z" && !shift {
                        e.prevent_default();
                        hook.go_back();
                    } else if meta && ((key == "z" && shift) || key == "y") {
                        e.prevent_default();
                        hook.go_forward();
                    }
                });

                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let _ = document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }

                closure.forget();
            }
        });

        hook
    }

    /// Push a new URL onto the stack (truncates any forward history).
    pub fn push(&self, url: String) {
        if *self.is_navigating.peek() {
            return;
        }

        let idx = *self.index.peek();

        // Truncate forward history
        let mut history = self.history;
        history.with_mut(|h| {
            h.truncate(idx + 1);
            h.push(url.clone());
        });
        let mut index = self.index;
        index.set(idx + 1);

        Self::replace_state(&url);
    }

    /// Navigate backwards (undo).
    pub fn go_back(&self) {
        let idx = *self.index.peek();
        if idx == 0 {
            return;
        }

        let mut is_navigating = self.is_navigating;
        is_navigating.set(true);
        let new_idx = idx - 1;
        let mut index = self.index;
        index.set(new_idx);

        let url = self.history.peek().get(new_idx).cloned().unwrap_or_default();
        Self::replace_state(&url);

        is_navigating.set(false);
    }

    /// Navigate forwards (redo).
    pub fn go_forward(&self) {
        let idx = *self.index.peek();
        let len = self.history.peek().len();
        if idx + 1 >= len {
            return;
        }

        let mut is_navigating = self.is_navigating;
        is_navigating.set(true);
        let new_idx = idx + 1;
        let mut index = self.index;
        index.set(new_idx);

        let url = self.history.peek().get(new_idx).cloned().unwrap_or_default();
        Self::replace_state(&url);

        is_navigating.set(false);
    }

    /// `true` when there is a previous state to undo to.
    pub fn can_go_back(&self) -> bool {
        self.index() > 0
    }

    /// `true` when there is a future state to redo to.
    pub fn can_go_forward(&self) -> bool {
        self.index() + 1 < (self.history)().len()
    }

    /// Current position in the stack (1-based for display).
    pub fn position(&self) -> usize {
        self.index() + 1
    }

    /// Total number of states in the stack.
    pub fn total(&self) -> usize {
        (self.history)().len()
    }

    /// The current URL in the history stack (reactive).
    pub fn current(&self) -> String {
        let history = (self.history)();
        let idx = self.index();
        history.get(idx).cloned().unwrap_or_default()
    }

    fn index(&self) -> usize {
        (self.index)()
    }

    /* ========================================================== */
    /*                     ✨ FUNCTIONS ✨                        */
    /* ========================================================== */

    fn replace_state(url: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let Ok(history) = web_sys::window().and_then(|w| w.history().ok()).ok_or(()) else { return };
            let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(url));
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = url;
    }
}

/// Access the `UseHistory` context initialized by `UseHistory::init()`.
pub fn use_history() -> UseHistory {
    consume_context::<UseHistory>()
}
```
