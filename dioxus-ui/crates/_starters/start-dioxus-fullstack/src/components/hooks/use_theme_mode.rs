use dioxus::prelude::*;

const STORAGE_KEY: &str = "darkmode";

/// Call once at App root. Reads from localStorage, applies `dark` class to `<html>`.
pub fn use_theme_provider() {
    let mut is_dark = use_context_provider(|| Signal::new(false));

    // Init from localStorage on mount
    use_effect(move || {
        spawn(async move {
            let result = dioxus::document::eval(&format!(
                "localStorage.getItem('{STORAGE_KEY}') === 'true' || \
                 (localStorage.getItem('{STORAGE_KEY}') === null && \
                  window.matchMedia('(prefers-color-scheme: dark)').matches)"
            ))
            .await;

            if let Ok(serde_json::Value::Bool(dark)) = result {
                is_dark.set(dark);
            }
        });
    });

    // Persist + apply class whenever value changes
    use_effect(move || {
        let dark = is_dark();
        spawn(async move {
            let script = if dark {
                format!(
                    "document.documentElement.classList.add('dark'); \
                     localStorage.setItem('{STORAGE_KEY}', 'true');"
                )
            } else {
                format!(
                    "document.documentElement.classList.remove('dark'); \
                     localStorage.setItem('{STORAGE_KEY}', 'false');"
                )
            };
            let _ = dioxus::document::eval(&script).await;
        });
    });
}

/// Access dark-mode signal from any child.
pub fn use_theme_mode() -> Signal<bool> {
    use_context::<Signal<bool>>()
}
