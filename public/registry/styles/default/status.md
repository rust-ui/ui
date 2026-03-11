---
title: "Status"
name: "status"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/status.rs"
description: "Rust/UI component for displaying statuses."
tags: []
---

# Status

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add status
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

const PING_ANIMATION: &str = "animate-ping";
const RELATIVE: &str = "relative";

#[component]
pub fn Status(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] variant: StatusIndactorVariant,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(RELATIVE, class);

    view! {
        <div class=merged_class>
            {children()} <StatusIndactor variant=variant class=PING_ANIMATION /> <StatusIndactor variant=variant />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(TwClass, Clone, Copy)]
#[tw(class = "absolute top-0 right-0 -mt-1 -mr-1 rounded-full size-4")]
pub struct StatusIndactorClass {
    pub variant: StatusIndactorVariant,
}

#[derive(TwVariant)]
pub enum StatusIndactorVariant {
    #[tw(default, class = "bg-neutral-300")]
    Default,
    #[tw(class = "bg-green-300 ")]
    Active,
    #[tw(class = "bg-orange-300 ")]
    Inactive,
    #[tw(class = "bg-sky-300 ")]
    Normal,
}

#[component]
pub fn StatusIndactor(
    #[prop(into, optional)] variant: Signal<StatusIndactorVariant>,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let merged_class = move || {
        let status_indicator = StatusIndactorClass { variant: variant.get() };
        status_indicator.with_class(class.clone())
    };

    view! { <div class=merged_class /> }
}
```
