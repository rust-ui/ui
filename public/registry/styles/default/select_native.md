---
title: "Select Native"
name: "select_native"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/select_native.rs"
---

# Select Native

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add select_native
```

## Component Code

```rust
use icons::ChevronDown;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

// TODO 💪: Use Label from label.rs.

#[component]
pub fn SelectNative(
    children: Children,
    #[prop(into)] id: &'static str,
    #[prop(optional, into)] class: String,
    #[prop(default = RwSignal::new(String::new()).read_only())] value: ReadSignal<String>,
    #[prop(optional)] on_change: Option<leptos::prelude::Callback<leptos::ev::Event>>,
) -> impl IntoView {
    clx! {
        SelectNativeRoot, select,
        "peer inline-flex w-full cursor-pointer appearance-none items-center rounded-lg h-9 pe-8 ps-3 border border-input bg-background shadow-sm shadow-black/5 transition-shadow text-sm text-foreground focus-visible:border-ring focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/20 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 has-[option[disabled]:checked]:text-muted-foreground",
    }

    view! {
        <div class="relative">
            <SelectNativeRoot
                attr:id=id
                class=class
                prop:value=move || value.get()
                on:change=move |ev| {
                    if let Some(handler) = on_change {
                        handler.run(ev);
                    }
                }
            >
                {children()}
            </SelectNativeRoot>

            <span class="flex absolute inset-y-0 justify-center items-center w-9 h-full pointer-events-none end-0 text-muted-foreground/80 peer-disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4">
                <ChevronDown />
            </span>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn OverlappingLabel(#[prop(into)] r#for: String, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <label
            for=r#for
            class="block absolute top-0 z-10 px-2 text-xs font-medium -translate-y-1/2 start-1 bg-background text-foreground group-has-[select:disabled]:opacity-50"
        >
            {label}
        </label>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn LabelNative(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] r#for: String,
    children: Children,
) -> impl IntoView {
    let class =
        tw_merge!("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70", class);

    view! {
        <label class=class r#for=r#for>
            {children()}
        </label>
    }
}
```
