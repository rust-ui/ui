use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    pub fn class(self) -> &'static str {
        match self {
            ItemVariant::Default => "bg-transparent",
            ItemVariant::Outline => "border-border",
            ItemVariant::Muted => "bg-muted/50",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl ItemSize {
    pub fn class(self) -> &'static str {
        match self {
            ItemSize::Default => "p-4 gap-4",
            ItemSize::Sm => "py-3 px-4 gap-2.5",
            ItemSize::Xs => "py-2 px-3 gap-2",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn class(self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "bg-transparent",
            ItemMediaVariant::Icon => "size-8 border rounded-sm bg-muted [&_svg:not([class*='size-'])]:size-4",
            ItemMediaVariant::Image => "size-10 rounded-sm overflow-hidden [&_img]:size-full [&_img]:object-cover",
        }
    }
}

const ITEM_BASE: &str = "group/item flex items-center border border-transparent text-sm rounded-md transition-colors [a]:hover:bg-accent/50 [a]:transition-colors duration-100 flex-wrap outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

const ITEM_MEDIA_BASE: &str = "flex shrink-0 items-center justify-center gap-2 group-has-[[data-slot=item-description]]/item:self-start [&_svg]:pointer-events-none group-has-[[data-slot=item-description]]/item:translate-y-0.5";

#[component]
pub fn Item(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ItemVariant::Default)] variant: ItemVariant,
    #[props(default = ItemSize::Default)] size: ItemSize,
    #[props(into, optional)] href: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(ITEM_BASE, variant.class(), size.class(), class.as_deref().unwrap_or(""));

    if let Some(href) = href {
        rsx! { a { "data-name": "Item", class: "{c}", href: "{href}", {children} } }
    } else {
        rsx! { div { "data-name": "Item", class: "{c}", {children} } }
    }
}

#[component]
pub fn ItemMedia(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ItemMediaVariant::Default)] variant: ItemMediaVariant,
    children: Element,
) -> Element {
    let c = tw_merge!(ITEM_MEDIA_BASE, variant.class(), class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemMedia", "data-slot": "item-media", class: "{c}", {children} } }
}

#[component]
pub fn ItemGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("group/item-group flex flex-col", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemGroup", class: "{c}", {children} } }
}

#[component]
pub fn ItemContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c =
        tw_merge!("flex flex-1 flex-col gap-1 [&+[data-slot=item-content]]:flex-none", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemContent", "data-slot": "item-content", class: "{c}", {children} } }
}

#[component]
pub fn ItemTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex w-fit items-center gap-2 text-sm leading-snug font-medium", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemTitle", class: "{c}", {children} } }
}

#[component]
pub fn ItemDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "text-muted-foreground line-clamp-2 text-sm leading-normal font-normal text-balance [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4",
        class.as_deref().unwrap_or("")
    );
    rsx! { p { "data-name": "ItemDescription", "data-slot": "item-description", class: "{c}", {children} } }
}

#[component]
pub fn ItemActions(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex items-center gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemActions", class: "{c}", {children} } }
}

#[component]
pub fn ItemHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex basis-full items-center justify-between gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemHeader", class: "{c}", {children} } }
}

#[component]
pub fn ItemFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex basis-full items-center justify-between gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemFooter", class: "{c}", {children} } }
}

#[component]
pub fn ItemSeparator(#[props(into, optional)] class: Option<String>) -> Element {
    // Mirrors leptos `<Separator attr:data-name="ItemSeparator" class=tw_merge!("my-0", class) />`.
    // Separator base string inlined so `data-name` and `role` stay on the rendered node.
    let c = tw_merge!("shrink-0 bg-border w-full h-[1px] my-0", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ItemSeparator", class: "{c}", role: "separator" } }
}
