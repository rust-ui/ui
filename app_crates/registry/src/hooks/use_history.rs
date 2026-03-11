use leptos::prelude::*;
use leptos::wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
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
///
/// view! {
///     <button on:click=move |_| history.go_back()>"Undo"</button>
///     <button on:click=move |_| history.go_forward()>"Redo"</button>
/// }
/// ```
#[derive(Clone, Copy)]
pub struct UseHistory {
    history: RwSignal<Vec<String>>,
    index: RwSignal<usize>,
    is_navigating: RwSignal<bool>,
}

impl UseHistory {
    /// Initialize the history stack and provide it as context.
    /// Sets up `⌘Z` / `⌘⇧Z` / `⌃Y` keyboard shortcuts on the document.
    #[must_use]
    pub fn init() -> Self {
        let hook =
            Self { history: RwSignal::new(Vec::new()), index: RwSignal::new(0), is_navigating: RwSignal::new(false) };

        provide_context(hook);

        // Seed the stack with the current URL
        Effect::new(move |_| {
            let href = window().location().href().unwrap_or_default();
            hook.history.update(|h| h.push(href));
        });

        // Register ⌘Z / ⌘⇧Z / ⌃Y shortcuts
        Effect::new(move |_| {
            let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
                let key = e.key();
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

            if let Some(document) = window().document() {
                let _ = document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }

            closure.forget();
        });

        hook
    }

    /// Push a new URL onto the stack (truncates any forward history).
    pub fn push(&self, url: String) {
        if self.is_navigating.get_untracked() {
            return;
        }

        let idx = self.index.get_untracked();
        self.history.update(|h| {
            h.truncate(idx + 1);
            h.push(url.clone());
        });
        self.index.update(|i| *i += 1);

        Self::replace_state(&url);
    }

    /// Navigate one step back in the stack.
    pub fn go_back(&self) {
        let idx = self.index.get_untracked();
        if idx == 0 {
            return;
        }

        self.is_navigating.set(true);
        let new_idx = idx - 1;
        self.index.set(new_idx);

        let url = self.history.with_untracked(|h| h.get(new_idx).cloned()).unwrap_or_default();
        Self::replace_state(&url);

        self.is_navigating.set(false);
    }

    /// Navigate one step forward in the stack.
    pub fn go_forward(&self) {
        let idx = self.index.get_untracked();
        let len = self.history.with_untracked(|h| h.len());
        if idx + 1 >= len {
            return;
        }

        self.is_navigating.set(true);
        let new_idx = idx + 1;
        self.index.set(new_idx);

        let url = self.history.with_untracked(|h| h.get(new_idx).cloned()).unwrap_or_default();
        Self::replace_state(&url);

        self.is_navigating.set(false);
    }

    /// `true` when there is a previous state to undo to.
    pub fn can_go_back(&self) -> Signal<bool> {
        let index = self.index;
        Signal::derive(move || index.get() > 0)
    }

    /// `true` when there is a future state to redo to.
    pub fn can_go_forward(&self) -> Signal<bool> {
        let history = self.history;
        let index = self.index;
        Signal::derive(move || index.get() + 1 < history.with(|h| h.len()))
    }

    /// Current position in the stack (1-based for display).
    pub fn position(&self) -> Signal<usize> {
        let index = self.index;
        Signal::derive(move || index.get() + 1)
    }

    /// Total number of states in the stack.
    pub fn total(&self) -> Signal<usize> {
        let history = self.history;
        Signal::derive(move || history.with(|h| h.len()))
    }

    /* ========================================================== */
    /*                     ✨ FUNCTIONS ✨                        */
    /* ========================================================== */

    fn replace_state(url: &str) {
        let Ok(history) = window().history() else { return };
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(url));
    }
}

/// Access the `UseHistory` context initialized by `UseHistory::init()`.
pub fn use_history() -> UseHistory {
    expect_context::<UseHistory>()
}
