---
title: "Sonner"
name: "sonner"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/sonner.rs"
description: "Rust/UI Toast, inspired by Sonner."
tags: []
---

# Sonner

Pure Rust/Leptos toast system with global context, trigger helpers, reactive rendering, auto-dismiss, stacking, and swipe-to-dismiss.

## Installation

To add this component in your app, run:

```bash
# cargo install ui-cli --force
ui add sonner
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display, Hash)]
pub enum ToastType {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display, Hash)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

pub fn provide_sonner() {
    if use_context::<SonnerContext>().is_none() {
        provide_context(new_sonner_context());
    }
}

pub fn show_toast() -> ToastApi {
    ToastApi { ctx: expect_context::<SonnerContext>() }
}

#[component]
pub fn SonnerTrigger(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = ToastType::default())] variant: ToastType,
    #[prop(into)] title: String,
    #[prop(into, optional)] description: String,
    #[prop(into, optional)] position: String,
) -> impl IntoView {
    let variant_classes = match variant {
        ToastType::Default => "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
        ToastType::Success => "bg-success text-success-foreground hover:bg-success/90",
        ToastType::Error => "bg-destructive text-white shadow-xs hover:bg-destructive/90 dark:bg-destructive/60",
        ToastType::Warning => "bg-warning text-warning-foreground hover:bg-warning/90",
        ToastType::Info => "bg-info text-info-foreground shadow-xs hover:bg-info/90",
        ToastType::Loading => "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
    };

    let merged_class = tw_merge!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] w-fit cursor-pointer h-9 px-4 py-2",
        variant_classes,
        class
    );

    view! {
        <button
            class=merged_class
            data-name="SonnerTrigger"
            on:click=move |_| {
                show_toast()
                    .message(title.clone())
                    .variant(variant)
                    .description(description.clone())
                    .push();
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SonnerToaster(#[prop(default = SonnerPosition::default())] position: SonnerPosition) -> impl IntoView {
    if use_context::<SonnerContext>().is_none() {
        provide_context(new_sonner_context());
    }

    view! {
        <style>{SONNER_STYLE}</style>
        <SonnerContainer position=position>
            <SonnerList position=position />
        </SonnerContainer>
    }
}
```
