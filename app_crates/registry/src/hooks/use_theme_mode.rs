#![cfg_attr(not(target_arch = "wasm32"), allow(clippy::missing_const_for_fn))]

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ThemeMode {
    state: Signal<bool>,
}

#[cfg(target_arch = "wasm32")]
const LOCALSTORAGE_KEY: &str = "darkmode";

#[must_use]
pub fn use_theme_mode() -> ThemeMode {
    consume_context::<ThemeMode>()
}

impl ThemeMode {
    #[must_use]
    pub fn init() -> Self {
        let state = use_signal(|| false);
        let theme_mode = Self { state };

        use_context_provider(|| theme_mode);

        use_effect(move || {
            let initial = Self::get_storage_state().unwrap_or_else(Self::prefers_dark_mode);
            let mut s = state;
            s.set(initial);
        });

        theme_mode
    }

    pub fn toggle(&mut self) {
        let new = !*self.state.read();
        self.state.set(new);
        Self::set_storage_state(new);
    }

    pub fn set_dark(&mut self) {
        self.set(true);
    }

    pub fn set_light(&mut self) {
        self.set(false);
    }

    pub fn set(&mut self, dark: bool) {
        self.state.set(dark);
        Self::set_storage_state(dark);
    }

    #[must_use]
    pub fn get(&self) -> bool {
        *self.state.read()
    }

    #[must_use]
    pub fn is_dark(&self) -> bool {
        *self.state.read()
    }

    #[must_use]
    pub fn is_light(&self) -> bool {
        !*self.state.read()
    }

    fn get_storage_state() -> Option<bool> {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
                .and_then(|s| s.get(LOCALSTORAGE_KEY).ok().flatten())
                .and_then(|v| v.parse::<bool>().ok())
        }
        #[cfg(not(target_arch = "wasm32"))]
        None
    }

    fn prefers_dark_mode() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|mql| mql.matches())
                .unwrap_or(false)
        }
        #[cfg(not(target_arch = "wasm32"))]
        false
    }

    fn set_storage_state(_state: bool) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
                let _ = storage.set(LOCALSTORAGE_KEY, &_state.to_string());
            }
        }
    }
}
