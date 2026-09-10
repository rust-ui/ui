use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use tw_merge::tw_merge;

static SHIMMER_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_shimmer_id() -> String {
    use_hook(|| {
        let id = SHIMMER_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("shimmer_{id}")
    })
}

#[component]
pub fn Shimmer(
    loading: Signal<bool>,
    #[props(into, optional)] shimmer_color: Option<String>,
    #[props(into, optional)] background_color: Option<String>,
    #[props(optional)] duration: Option<f64>,
    #[props(optional)] fallback_border_radius: Option<f64>,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let shimmer_id = use_shimmer_id();
    let c = tw_merge!("relative", class.as_deref().unwrap_or(""));
    let shimmer_color = shimmer_color.unwrap_or_default();
    let background_color = background_color.unwrap_or_default();
    let duration_str = duration.map(|d| d.to_string()).unwrap_or_default();
    let radius_str = fallback_border_radius.map(|r| r.to_string()).unwrap_or_default();
    let loading_str = use_memo(move || loading().to_string());

    rsx! {
        div {
            id: "{shimmer_id}",
            class: "{c}",
            "data-name": "Shimmer",
            "data-shimmer-loading": "{loading_str}",
            "data-shimmer-color": "{shimmer_color}",
            "data-shimmer-bg-color": "{background_color}",
            "data-shimmer-duration": "{duration_str}",
            "data-shimmer-fallback-radius": "{radius_str}",
            {children}
        }
    }
}
