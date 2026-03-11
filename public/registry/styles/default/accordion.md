---
title: "Accordion"
name: "accordion"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/accordion.rs"
description: "Rust/UI component that displays an Accordion."
tags: ["accordion"]
---

# Accordion

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add accordion
```

## Component Code

```rust
use icons::ChevronDown;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id;
mod components {
    use super::*;
    clx! {Accordion, div, "divide-y divide-input w-full"}
    clx! {AccordionItem, div, "w-full [&:has(>input:checked)>label>svg:last-child]:rotate-180"}
    clx! {AccordionTitle, h4, "text-sm font-medium"}
    clx! {AccordionHeader, div, "flex gap-2 items-center [&_svg:not([class*='size-'])]:size-4"}
    clx! {RootContent, article, "grid overflow-hidden transition-all duration-400 grid-rows-[0fr] peer-checked:grid-rows-[1fr]"}
    clx! {AccordionDescription, p, "text-muted-foreground text-sm"}
    clx! {AccordionLink, a, "grid gap-2.5 items-center p-2 grid-cols-[auto_1fr] [&_svg:not([class*='size-'])]:size-4 hover:bg-muted"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn AccordionContent(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("p-3 pt-0", class);

    view! {
        <RootContent>
            // * Used for the animation using grid CSS trick.
            <div data-name="__AccordionContentInner" class="min-h-[0]">
                <div class=merged_class>{children()}</div>
            </div>
        </RootContent>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Default)]
pub enum AccordionTriggerIcon {
    #[default]
    ChevronDown,
    Plus,
}

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] class: String,
    #[prop(default = false)] open: bool,
    // TODO. AccrodionTriggerIcon
    children: Children,
) -> impl IntoView {
    let accordion_id = use_random_id();
    let label_class = tw_merge!(
        "flex justify-between items-center p-3 list-none cursor-pointer [&_svg:not([class*='size-'])]:size-4 peer-focus-visible:ring-2 peer-focus-visible:ring-ring peer-focus-visible:ring-offset-2",
        class
    );

    view! {
        <>
            <input
                id=accordion_id.clone()
                type="checkbox"
                class="overflow-hidden absolute p-0 -m-px w-px h-px whitespace-nowrap border-0 peer"
                style="clip: rect(0, 0, 0, 0)"
                checked=open
            />
            <label for=accordion_id class=label_class>
                {children()}
                <ChevronDown class="transition-all duration-300" />
            </label>
        </>
    }
}
```
