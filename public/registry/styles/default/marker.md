---
title: "Marker"
name: "marker"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/marker.rs"
description: "Displays an inline status, system note, bordered row, or labeled separator in a conversation."
tags: []
---

# Marker

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add marker
```

## Component Code

```rust
use leptos::ev;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum MarkerVariant {
    #[default]
    Default,
    Separator,
    Border,
}

/* ========================================================== */
/*                     Components (clx!)                      */
/* ========================================================== */

mod components {
    use super::*;

    // data-name="MarkerContent" auto-set by clx!
    clx! {
        MarkerContent,
        span,
        "min-w-0 wrap-break-word group-data-[variant=Separator]/marker:flex-none group-data-[variant=Separator]/marker:text-center *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground"
    }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn MarkerIcon(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("size-4 shrink-0 [&_svg:not([class*='size-'])]:size-4", class);

    view! {
        <span class=merged_class data-name="MarkerIcon" aria-hidden="true">
            {children()}
        </span>
    }
}

#[component]
pub fn Marker(
    #[prop(optional)] variant: MarkerVariant,
    #[prop(optional, into)] class: String,
    // TODO PORT: shadcn uses a `render` prop (e.g. `render={<a href="..." />}`) to swap
    // the root element polymorphically (Radix asChild pattern). Leptos has no asChild, so
    // we split into two explicit props: `href` (→ <a>) and `on_click` (→ <button>).
    // If neither is set, renders a plain <div>. Matches shadcn behavior for all 3 cases.
    /// Renders the marker as an <a> element
    #[prop(optional, into)]
    href: Option<String>,
    /// Renders the marker as a <button> element
    #[prop(optional)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// role attribute (e.g. "status" for streaming markers)
    #[prop(optional, into)]
    role: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "group/marker relative flex min-h-4 w-full items-center gap-2 text-left text-sm text-muted-foreground [&_svg:not([class*='size-'])]:size-4 [a]:underline [a]:underline-offset-3 [a]:hover:text-foreground",
        match variant {
            MarkerVariant::Default => "",
            MarkerVariant::Separator =>
                "before:mr-1 before:h-px before:min-w-0 before:flex-1 before:bg-border after:ml-1 after:h-px after:min-w-0 after:flex-1 after:bg-border",
            MarkerVariant::Border => "border-b border-border pb-2",
        },
        class
    );

    let variant_str = variant.to_string();

    match (href, on_click) {
        (Some(href), _) => view! {
            <a href=href class=merged_class data-name="Marker" data-variant=variant_str role=role>
                {children()}
            </a>
        }
        .into_any(),
        (_, Some(cb)) => view! {
            <button
                type="button"
                class=merged_class
                data-name="Marker"
                data-variant=variant_str
                role=role
                on:click=move |e| cb.run(e)
            >
                {children()}
            </button>
        }
        .into_any(),
        _ => view! {
            <div class=merged_class data-name="Marker" data-variant=variant_str role=role>
                {children()}
            </div>
        }
        .into_any(),
    }
}
```
