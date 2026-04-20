---
title: "Drawer"
name: "drawer"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:ui"
path: "ui/drawer.rs"
description: "A Drawer for Rust. Inspired by the amazing work of Emil Kowalski."
tags: []
---

# Drawer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add drawer
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

mod components {
    use super::*;
    clx! {DrawerBody, div, "flex flex-col gap-4 mx-auto max-w-[500px]"}
    clx! {DrawerTitle, h2, "text-lg leading-none font-semibold"}
    clx! {DrawerDescription, p, "text-sm text-muted-foreground"}
    clx! {DrawerHeader, div, "flex flex-col gap-2"}
    clx! {DrawerFooter, footer, "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"}
}

pub use components::*;

#[derive(Clone)]
pub struct DrawerContext {
    open: RwSignal<bool>,
}

impl DrawerContext {
    pub fn open(&self) {
        if !self.open.get_untracked() {
            self.open.set(true);
        }
    }

    pub fn close(&self) {
        if self.open.get_untracked() {
            self.open.set(false);
        }
    }
}

#[component]
pub fn DrawerTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = use_context::<DrawerContext>();

    view! {
        <Button
            data_name="DrawerTrigger"
            class=class
            variant=variant
            size=size
            on:click=move |_| {
                if let Some(ctx) = &ctx {
                    ctx.open();
                }
            }
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn DrawerClose(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = use_context::<DrawerContext>();

    view! {
        <Button
            data_name="DrawerClose"
            class=class
            variant=variant
            size=size
            on:click=move |_| {
                if let Some(ctx) = &ctx {
                    ctx.close();
                }
            }
        >
            {children()}
        </Button>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Drawer(
    children: Children,
    #[prop(optional, default = true)] show_overlay: bool,
    #[prop(optional, default = true)] lock_body_scroll: bool,
) -> impl IntoView {
    let overlay_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<html::Div>::new();
    let hidden = RwSignal::new(true);
    let state = RwSignal::new("closed");
    let content_animate = RwSignal::new(true);
    let overlay_animate = RwSignal::new(true);
    let dismissible = RwSignal::new(true);
    let previous_active_element = RwSignal::new(None::<Element>);
    let animation_epoch = RwSignal::new(0_u64);
    let open = RwSignal::new(false);

    provide_context(DrawerContext {
        open,
        overlay_ref,
        content_ref,
        hidden,
        state,
        content_animate,
        overlay_animate,
        dismissible,
        lock_body_scroll,
        previous_active_element,
        animation_epoch,
    });

    view! {
        <div
            data-name="DrawerOverlay"
            node_ref=overlay_ref
            class=move || {
                let mut class = "fixed inset-0 z-200 bg-black/50".to_string();
                if hidden.get() || !show_overlay {
                    class.push_str(" hidden");
                }
                class
            }
            data-vaul-overlay=""
            data-vaul-snap-points="false"
            data-vaul-animate=move || bool_attr(overlay_animate.get())
            data-state=move || state.get()
            data-lock-body-scroll=if lock_body_scroll { "true" } else { "false" }
        ></div>

        {children()}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum DrawerPosition {
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum DrawerVariant {
    #[default]
    Inset,
    Floating,
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = DrawerPosition::default())] position: DrawerPosition,
    #[prop(optional, default = DrawerVariant::default())] variant: DrawerVariant,
    #[prop(into, default = "--initial-transform: 100%;".to_string())] style: String,
    #[prop(optional, default = true)] dismissible: bool,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex flex-col pt-3 pb-6 px-6 hidden fixed right-0 bottom-0 left-0 z-210 bg-background max-h-[96vh] rounded-t-[10px]",
        class
    );

    view! {
        <div
            data-name="DrawerContent"
            class=merged_class
            data-vaul-drawer=""
            data-vaul-drawer-position=position.to_string()
            data-vaul-variant=variant.to_string()
            data-vaul-snap-points="false"
            data-vaul-animate="true"
            data-vaul-dismissible=dismissible
            data-state="closed"
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerHandle() -> impl IntoView {
    view! {
        <div
            class="block relative mx-auto mb-8 w-8 rounded-2xl opacity-70 hover:opacity-100 active:opacity-100 shrink-0 bg-[#e2e2e4] h-[5px]"
            data-vaul-handle=""
        >
            <span data-vaul-handle-hitarea=""></span>
        </div>
    }
}
```
