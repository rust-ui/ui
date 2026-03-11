---
title: "Tabs"
name: "tabs"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/tabs.rs"
description: "Rust/UI component that displays a set of layered sections of content, known as tab pages, that are displayed one at a time."
tags: ["utils"]
---

# Tabs

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add tabs
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Context                              */
/* ========================================================== */

#[derive(Clone)]
struct TabsCtx {
    selected: RwSignal<String>,
}

#[derive(Clone, Copy)]
struct TabsListCtx {
    variant: TabsVariant,
}

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsVariant {
    #[default]
    Default,
    Line,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Tabs(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] default_value: String,
    #[prop(default = TabsOrientation::Horizontal)] orientation: TabsOrientation,
    children: Children,
) -> impl IntoView {
    let selected = RwSignal::new(default_value);
    provide_context(TabsCtx { selected });

    let is_horizontal = orientation == TabsOrientation::Horizontal;
    let class = tw_merge!("group/tabs flex gap-2", if is_horizontal { "flex-col" } else { "flex-row" }, class);

    view! {
        <div class=class data-name="Tabs" data-orientation=if is_horizontal { "Horizontal" } else { "Vertical" }>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsList(
    #[prop(into, optional)] class: String,
    #[prop(default = TabsVariant::Default)] variant: TabsVariant,
    children: Children,
) -> impl IntoView {
    provide_context(TabsListCtx { variant });

    let is_line = variant == TabsVariant::Line;
    let class = tw_merge!(
        "group/tabs-list inline-flex w-fit items-center justify-center rounded-lg p-[3px] text-muted-foreground",
        // Orientation via group-data on parent Tabs element (works — Tailwind scans static strings)
        "group-data-[orientation=Horizontal]/tabs:h-8",
        "group-data-[orientation=Vertical]/tabs:h-fit group-data-[orientation=Vertical]/tabs:flex-col",
        if is_line { "gap-1 bg-transparent rounded-none p-0" } else { "bg-muted" },
        class
    );

    view! {
        <div class=class data-name="TabsList" data-variant=if is_line { "Line" } else { "Default" }>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<TabsCtx>();
    let variant = expect_context::<TabsListCtx>().variant;
    let is_line = variant == TabsVariant::Line;

    let val = value.clone();
    let is_active = Memo::new(move |_| ctx.selected.get() == val);

    // Static classes that Tailwind can scan (group-data selectors reference parent data attrs)
    let base = tw_merge!(
        "relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium whitespace-nowrap transition-all cursor-pointer select-none",
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-none",
        "disabled:pointer-events-none disabled:opacity-50",
        // Vertical layout via group-data on parent
        "group-data-[orientation=Vertical]/tabs:w-full group-data-[orientation=Vertical]/tabs:justify-start",
        // After pseudo-element (underline indicator)
        "after:absolute after:bg-foreground after:transition-opacity",
        "group-data-[orientation=Horizontal]/tabs:after:inset-x-0 group-data-[orientation=Horizontal]/tabs:after:bottom-[-5px] group-data-[orientation=Horizontal]/tabs:after:h-0.5",
        "group-data-[orientation=Vertical]/tabs:after:inset-y-0 group-data-[orientation=Vertical]/tabs:after:-right-1 group-data-[orientation=Vertical]/tabs:after:w-0.5",
        class
    );

    view! {
        <button
            class=move || {
                let active = is_active.get();
                tw_merge!(
                    &base,
                    // Active state applied reactively (CSS attr selectors not generated by Tailwind)
                    if active { "text-foreground" } else { "text-foreground/60 hover:text-foreground" },
                    // Default variant: active gets white pill + shadow
                    if !is_line && active { "bg-background shadow-sm dark:border-input dark:bg-input/30" } else { "" },
                    // Line variant: show underline when active
                    if is_line && active { "after:opacity-100" } else { "after:opacity-0" },
                )
            }
            data-name="TabsTrigger"
            data-state=move || if is_active.get() { "Active" } else { "Inactive" }
            on:click=move |_| ctx.selected.set(value.clone())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<TabsCtx>();
    let val = value;
    let is_active = Memo::new(move |_| ctx.selected.get() == val);

    view! {
        <div
            data-name="TabsContent"
            class=move || {
                tw_merge!(
                    "flex-1 text-sm outline-none",
                // Orientation via group-data on parent
                "group-data-[orientation=Horizontal]/tabs:mt-2 group-data-[orientation=Vertical]/tabs:ml-4",
                &class,
                if is_active.get() { "" } else { "hidden" }
                )
            }
        >
            {children()}
        </div>
    }
}
```
