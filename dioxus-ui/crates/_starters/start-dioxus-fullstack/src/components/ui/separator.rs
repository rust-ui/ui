use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn Separator(
    #[props(default)] orientation: SeparatorOrientation,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let orientation_class = match orientation {
        SeparatorOrientation::Horizontal => "w-full h-[1px]",
        SeparatorOrientation::Vertical => "h-full w-[1px]",
    };
    let merged = tw_merge!("shrink-0 bg-border", orientation_class, class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", role: "separator" } }
}
