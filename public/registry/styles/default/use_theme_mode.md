---
title: "Use Theme Mode"
name: "use_theme_mode"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_theme_mode.rs"
---

# Use Theme Mode

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_theme_mode
```

## Component Code

```rust
use leptos::prelude::*;
use web_sys::Storage;

#[derive(Debug, Clone, Copy)]
pub struct ThemeMode {
    state: RwSignal<bool>,
}

const LOCALSTORAGE_KEY: &str = "darkmode";

/// Hook to access the dark mode context
///
/// Returns the ThemeMode instance from context for easy access
pub fn use_theme_mode() -> ThemeMode {
    expect_context::<ThemeMode>()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl ThemeMode {
    #[must_use]
    /// Initializes a new ThemeMode instance.
    pub fn init() -> Self {
        let theme_mode = Self { state: RwSignal::new(false) };

        provide_context(theme_mode);

        // Use Effect to handle browser-only initialization
        Effect::new(move |_| {
            let initial = Self::get_storage_state().unwrap_or(Self::prefers_dark_mode());
            theme_mode.state.set(initial);
        });

        theme_mode
    }

    pub fn toggle(&self) {
        self.state.update(|state| {
            *state = !*state;
            Self::set_storage_state(*state);
        });
    }

    pub fn set_dark(&self) {
        self.set(true);
    }

    pub fn set_light(&self) {
        self.set(false);
    }

    /// - `dark`: Set to `true` for dark mode, and `false` for light mode.
    pub fn set(&self, dark: bool) {
        self.state.set(dark);
        Self::set_storage_state(dark);
    }

    #[must_use]
    pub fn get(&self) -> bool {
        self.state.get()
    }

    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.state.get()
    }

    #[must_use]
    pub fn is_light(&self) -> bool {
        !self.state.get()
    }

    /* ========================================================== */
    /*                     ✨ FUNCTIONS ✨                        */
    /* ========================================================== */

    /// Retrieves the local storage object, if available.
    fn get_storage() -> Option<Storage> {
        window().local_storage().ok().flatten()
    }

    /// Retrieves the dark mode state from local storage, if available.
    fn get_storage_state() -> Option<bool> {
        Self::get_storage()
            .and_then(|storage| storage.get(LOCALSTORAGE_KEY).ok())
            .flatten()
            .and_then(|entry| entry.parse::<bool>().ok())
    }

    /// Checks whether the user's system prefers dark mode based on media queries.
    fn prefers_dark_mode() -> bool {
        window()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map(|media| media.matches())
            .unwrap_or_default()
    }

    /// Stores the dark mode state in local storage.
    fn set_storage_state(state: bool) {
        if let Some(storage) = Self::get_storage() {
            storage.set(LOCALSTORAGE_KEY, state.to_string().as_str()).ok();
        }
    }
}
```
