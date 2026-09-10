use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PathMatchType {
    #[default]
    StartsWith,
    Exact,
    Contains,
    Custom(String),
    StartsWithExcept(String, Vec<String>),
    MatchAny(Vec<String>),
}

#[component]
pub fn Link(
    children: Element,
    #[props(into)] href: String,
    #[props(default = true)] scroll: bool,
    #[props(default = PathMatchType::default())] match_type: PathMatchType,
    #[props(default = false)] force_reload: bool,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    // In Dioxus, we don't have a built-in use_location yet — active detection is simplified.
    // Pass `active: true` via class or extend later when dioxus-router location hook is available.

    #[cfg(target_arch = "wasm32")]
    let href_clone = href.clone();
    let class_str = class.as_deref().unwrap_or("").to_string();

    if force_reload {
        rsx! {
            button {
                r#type: "button",
                class: "cursor-pointer",
                onclick: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href(&href_clone);
                    }
                },
                span { class: "{class_str}", {children} }
            }
        }
    } else {
        rsx! {
            a { href: "{href}", class: "{class_str}", {children} }
        }
    }
}
