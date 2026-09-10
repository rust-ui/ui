use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Moon, Sun};

#[component]
pub fn ThemeToggle() -> Element {
    let mut dark = use_signal(|| false);

    // On mount: read preference from localStorage, fallback to system preference.
    // Uses eval() to access browser APIs not available in WASM directly.
    use_effect(move || {
        spawn(async move {
            let mut ev = eval(
                "return localStorage.getItem('darkmode') === 'true' || \
                 (localStorage.getItem('darkmode') === null && \
                  window.matchMedia('(prefers-color-scheme: dark)').matches);",
            );
            if let Ok(is_dark) = ev.recv::<bool>().await {
                dark.set(is_dark);
                // Apply immediately to avoid flash
                if is_dark {
                    eval("document.documentElement.classList.add('dark')");
                }
            }
        });
    });

    // Whenever dark changes: toggle .dark on <html> and persist to localStorage.
    use_effect(move || {
        let is_dark = dark();
        eval(&format!(
            "document.documentElement.classList.toggle('dark', {is_dark}); \
             localStorage.setItem('darkmode', '{is_dark}');"
        ));
    });

    let toggle = move |_| {
        dark.set(!dark());
    };

    rsx! {
        button {
            r#type: "button",
            "aria-label": "Toggle theme",
            class: "inline-flex items-center justify-center size-8 rounded-md hover:bg-accent transition-colors cursor-pointer",
            onclick: toggle,
            if dark() {
                Sun { class: "size-4" }
            } else {
                Moon { class: "size-4" }
            }
        }
    }
}
