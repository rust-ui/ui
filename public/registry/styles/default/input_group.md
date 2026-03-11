---
title: "Input Group"
name: "input_group"
cargo_dependencies: []
registry_dependencies: ["input", "textarea"]
type: "components:ui"
path: "ui/input_group.rs"
---

# Input Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add input_group
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::{clx, variants};
use tw_merge::{TwClass, TwVariant, tw_merge};

use crate::components::ui::input::{Input, InputType};
use crate::components::ui::textarea::Textarea;

mod components {
    use super::*;
    clx! {InputGroupText, span, "text-muted-foreground flex items-center gap-2 text-sm [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn InputGroup(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "group/input-group border-input dark:bg-input/30 relative flex w-full items-center rounded-md border shadow-xs transition-[color,box-shadow] outline-none h-9 min-w-0 has-[>textarea]:h-auto has-[>[data-align=inline-start]]:[&>input]:pl-2 has-[>[data-align=inline-end]]:[&>input]:pr-2 has-[>[data-align=block-start]]:h-auto has-[>[data-align=block-start]]:flex-col has-[>[data-align=block-start]]:[&>input]:pb-3 has-[>[data-align=block-end]]:h-auto has-[>[data-align=block-end]]:flex-col has-[>[data-align=block-end]]:[&>input]:pt-3 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot=input-group-control]:focus-visible]:ring-[3px] has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40",
        class
    );

    view! {
        <div data-name="InputGroup" data-slot="input-group" role="group" class=merged_class>
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupAddon(
    #[prop(default = InputGroupAddonAlign::default())] align: InputGroupAddonAlign,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let addon = InputGroupAddonClass { align };
    let merged_class = addon.with_class(class);

    let align_attr = match align {
        InputGroupAddonAlign::InlineStart => "inline-start",
        InputGroupAddonAlign::InlineEnd => "inline-end",
        InputGroupAddonAlign::BlockStart => "block-start",
        InputGroupAddonAlign::BlockEnd => "block-end",
    };

    view! {
        <div
            data-name="InputGroupAddon"
            data-slot="input-group-addon"
            data-align=align_attr
            role="group"
            class=merged_class
        >
            {children()}
        </div>
    }
}

#[derive(TwClass, Default)]
#[tw(
    class = "text-muted-foreground flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-medium select-none [&>svg:not([class*='size-'])]:size-4 [&>kbd]:rounded-[calc(var(--radius)-5px)] group-data-[disabled=true]/input-group:opacity-50"
)]
struct InputGroupAddonClass {
    align: InputGroupAddonAlign,
}

#[derive(TwVariant)]
pub enum InputGroupAddonAlign {
    #[tw(default, class = "order-first pl-3 has-[>button]:ml-[-0.45rem] has-[>kbd]:ml-[-0.35rem]")]
    InlineStart,
    #[tw(class = "order-last pr-3 has-[>button]:mr-[-0.45rem] has-[>kbd]:mr-[-0.35rem]")]
    InlineEnd,
    #[tw(class = "order-first w-full justify-start px-3 pt-3 [.border-b]:pb-3 group-has-[>input]/input-group:pt-2.5")]
    BlockStart,
    #[tw(class = "order-last w-full justify-start px-3 pb-3 [.border-t]:pt-3 group-has-[>input]/input-group:pb-2.5")]
    BlockEnd,
}

variants! {
    InputGroupButton {
        base: "text-sm shadow-none flex gap-2 items-center",
        variants: {
            variant: {
                Ghost: "",
            },
            size: {
                Xs: "h-6 gap-1 px-2 rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-3.5 has-[>svg]:px-2",
                Sm: "h-8 px-2.5 gap-1.5 rounded-md has-[>svg]:px-2.5",
                IconXs: "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0",
                IconSm: "size-8 p-0 has-[>svg]:p-0",
            }
        },
        component: {
            element: button
        }
    }
}

#[component]
pub fn InputGroupInput(
    #[prop(into, optional)] class: String,
    #[prop(default = InputType::default())] r#type: InputType,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] title: String,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] readonly: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] autofocus: bool,
    #[prop(into, optional)] min: String,
    #[prop(into, optional)] max: String,
    #[prop(into, optional)] step: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex-1 rounded-none border-0 bg-transparent shadow-none focus-visible:ring-0 dark:bg-transparent",
        class
    );

    view! {
        <Input
            attr:data-slot="input-group-control"
            class=merged_class
            r#type=r#type
            placeholder=placeholder
            name=name
            id=id
            title=title
            disabled=disabled
            readonly=readonly
            required=required
            autofocus=autofocus
            min=min
            max=max
            step=step
        />
    }
}

#[component]
pub fn InputGroupTextarea(#[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex-1 resize-none rounded-none border-0 bg-transparent py-3 shadow-none focus-visible:ring-0 dark:bg-transparent",
        class
    );

    view! { <Textarea class=merged_class attr:data-slot="input-group-control" /> }
}
```
