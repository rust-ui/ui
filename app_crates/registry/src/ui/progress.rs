use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Progress(
    #[props(default = 0.0)] value: f64,
    #[props(default = 100.0)] max: f64,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let pct = (value / max * 100.0).clamp(0.0, 100.0);
    let style = format!("transform: translateX(-{}%)", 100.0 - pct);

    let merged = tw_merge!(
        "relative h-2 w-full overflow-hidden rounded-full bg-secondary",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "Progress",
            role: "progressbar",
            aria_valuemin: "0",
            aria_valuemax: "{max}",
            aria_valuenow: "{value}",
            class: "{merged}",
            div {
                class: "flex-1 w-full h-full transition-all duration-300 ease-in-out bg-primary",
                style: "{style}",
            }
        }
    }
}
