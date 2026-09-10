use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn ScrollArea(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("relative overflow-hidden", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-name": "ScrollArea", class: "{merged}",
            ScrollAreaViewport { {children} }
            ScrollBar {}
            ScrollAreaCorner {}
        }
    }
}

#[component]
pub fn ScrollAreaViewport(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "focus-visible:ring-ring/50 size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:outline-1 overflow-auto",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { "data-name": "ScrollAreaViewport", class: "{merged}", {children} }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ScrollBarOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[component]
pub fn ScrollBar(
    #[props(default = ScrollBarOrientation::default())] orientation: ScrollBarOrientation,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let orientation_class = match orientation {
        ScrollBarOrientation::Vertical => "h-full w-2.5 border-l border-l-transparent",
        ScrollBarOrientation::Horizontal => "h-2.5 flex-col border-t border-t-transparent",
    };
    let merged = tw_merge!(
        "flex touch-none p-px transition-colors select-none",
        orientation_class,
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { "data-name": "ScrollBar", class: "{merged}",
            ScrollAreaThumb {}
        }
    }
}

#[component]
pub fn ScrollAreaThumb(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!("bg-border relative flex-1 rounded-full", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}" } }
}

#[component]
pub fn ScrollAreaCorner(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!("bg-border", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}" } }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SnapAreaVariant {
    #[default]
    Center,
}

#[component]
pub fn SnapScrollArea(
    #[props(default = SnapAreaVariant::Center)] variant: SnapAreaVariant,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        SnapAreaVariant::Center => "overflow-x-auto snap-x",
    };
    let merged = tw_merge!(variant_class, class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SnapScrollArea", class: "{merged}", {children} } }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SnapVariant {
    #[default]
    Center,
}

#[component]
pub fn SnapItem(
    #[props(default = SnapVariant::Center)] variant: SnapVariant,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        SnapVariant::Center => "snap-center",
    };
    let merged = tw_merge!("shrink-0", variant_class, class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SnapItem", class: "{merged}", {children} } }
}
