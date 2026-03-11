use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use tw_merge::*;

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
    children: Children,
    #[prop(into)] href: String,
    #[prop(into, default = true)] scroll: bool, // Default to true in Leptos.
    #[prop(into, default = PathMatchType::default())] match_type: PathMatchType,
    #[prop(into, default = false)] force_reload: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let location = use_location();

    let current_path = Memo::new(move |_| location.pathname.get());

    let is_active = move |path_segment: &str| -> bool {
        let current = current_path();
        match &match_type {
            PathMatchType::StartsWith => current.starts_with(path_segment),
            PathMatchType::Exact => current == path_segment,
            PathMatchType::Contains => current.contains(path_segment),
            PathMatchType::Custom(custom_path) => current.starts_with(custom_path),
            PathMatchType::StartsWithExcept(base_path, excludes) => {
                current.starts_with(base_path) && !excludes.contains(&current)
            }
            PathMatchType::MatchAny(paths) => paths.contains(&current),
        }
    };

    let class_with_active = move |path_segment: &str| -> String {
        let active_styles = if is_active(path_segment) { "font-semibold" } else { "" };
        tw_merge!(&class, active_styles)
    };

    if force_reload {
        let href_clone = href.clone();
        view! {
            <button
                type="button"
                class="cursor-pointer"
                on:click=move |_| {
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href(&href_clone);
                    }
                }
            >
                <span class=move || class_with_active(&href)>{children()}</span>
            </button>
        }
        .into_any()
    } else {
        view! {
            <A href=href.clone() scroll=scroll>
                <span class=move || class_with_active(&href)>{children()}</span>
            </A>
        }
        .into_any()
    }
}
