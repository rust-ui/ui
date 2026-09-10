use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Kbd(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let class = tw_merge!(
        "bg-muted text-muted-foreground pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm px-1 font-sans text-xs font-medium select-none [&_svg:not([class*='size-'])]:size-3 [[data-slot=tooltip-content]_&]:bg-background/20 [[data-slot=tooltip-content]_&]:text-background dark:[[data-slot=tooltip-content]_&]:bg-background/10",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        kbd { class: "{class}", {children} }
    }
}

#[component]
pub fn KbdGroup(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let class = tw_merge!("inline-flex items-center gap-1", class.as_deref().unwrap_or(""));
    rsx! {
        kbd { class: "{class}", {children} }
    }
}
