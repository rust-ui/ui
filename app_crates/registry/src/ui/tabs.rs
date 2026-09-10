use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    #[default]
    Default,
    Line,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
struct TabsCtx {
    active: Signal<String>,
}

#[derive(Clone, Copy)]
struct TabsListCtx {
    variant: TabsVariant,
}

#[component]
pub fn Tabs(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] default_value: Option<String>,
    #[props(default = TabsOrientation::Horizontal)] orientation: TabsOrientation,
    children: Element,
) -> Element {
    let active = use_signal(|| default_value.unwrap_or_default());
    provide_context(TabsCtx { active });

    let is_horizontal = orientation == TabsOrientation::Horizontal;
    let class = tw_merge!(
        "group/tabs flex gap-2",
        if is_horizontal { "flex-col" } else { "flex-row" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: "{class}",
            "data-name": "Tabs",
            "data-orientation": if is_horizontal { "Horizontal" } else { "Vertical" },
            {children}
        }
    }
}

#[component]
pub fn TabsList(
    #[props(into, optional)] class: Option<String>,
    #[props(default = TabsVariant::Default)] variant: TabsVariant,
    children: Element,
) -> Element {
    provide_context(TabsListCtx { variant });

    let is_line = variant == TabsVariant::Line;
    let class = tw_merge!(
        "group/tabs-list inline-flex w-fit items-center justify-center rounded-lg p-[3px] text-muted-foreground",
        "group-data-[orientation=Horizontal]/tabs:h-8",
        "group-data-[orientation=Vertical]/tabs:h-fit group-data-[orientation=Vertical]/tabs:flex-col",
        if is_line { "gap-1 bg-transparent rounded-none p-0" } else { "bg-muted" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: "{class}",
            "data-name": "TabsList",
            "data-variant": if is_line { "Line" } else { "Default" },
            {children}
        }
    }
}

#[component]
pub fn TabsTrigger(
    #[props(into)] value: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let TabsCtx { mut active } = use_context::<TabsCtx>();
    let list_ctx = use_context::<TabsListCtx>();
    let is_line = list_ctx.variant == TabsVariant::Line;
    let val = value.clone();
    let is_active = active() == val;

    let merged = tw_merge!(
        "relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium whitespace-nowrap transition-all cursor-pointer select-none",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-none",
        "disabled:pointer-events-none disabled:opacity-50",
        "group-data-[orientation=Vertical]/tabs:w-full group-data-[orientation=Vertical]/tabs:justify-start",
        "after:absolute after:bg-foreground after:transition-opacity",
        "group-data-[orientation=Horizontal]/tabs:after:inset-x-0 group-data-[orientation=Horizontal]/tabs:after:bottom-[-5px] group-data-[orientation=Horizontal]/tabs:after:h-0.5",
        "group-data-[orientation=Vertical]/tabs:after:inset-y-0 group-data-[orientation=Vertical]/tabs:after:-right-1 group-data-[orientation=Vertical]/tabs:after:w-0.5",
        if is_active { "text-foreground" } else { "text-foreground/60 hover:text-foreground" },
        if !is_line && is_active { "bg-background shadow-sm dark:border-input dark:bg-input/30" } else { "" },
        if is_line && is_active { "after:opacity-100" } else { "after:opacity-0" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            class: "{merged}",
            "data-name": "TabsTrigger",
            "data-state": if is_active { "Active" } else { "Inactive" },
            onclick: move |_| active.set(value.clone()),
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    #[props(into)] value: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let TabsCtx { active } = use_context::<TabsCtx>();
    let is_active = active() == value;

    let class = tw_merge!(
        "flex-1 text-sm outline-none",
        "group-data-[orientation=Horizontal]/tabs:mt-2 group-data-[orientation=Vertical]/tabs:ml-4",
        if is_active { "" } else { "hidden" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "{class}", "data-name": "TabsContent", {children} }
    }
}
