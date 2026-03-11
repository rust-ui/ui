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

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add sonner
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum ToastType {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    #[default]
    BottomRight,
    BottomCenter,
    BottomLeft,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SonnerDirection {
    TopDown,
    #[default]
    BottomUp,
}

#[component]
pub fn SonnerTrigger(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = ToastType::default())] variant: ToastType,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
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

    // Only set position attribute if not empty
    let position_attr = if position.is_empty() { None } else { Some(position) };

    view! {
        <button
            class=merged_class
            data-name="SonnerTrigger"
            data-variant=variant.to_string()
            data-toast-title=title
            data-toast-description=description
            data-toast-position=position_attr
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SonnerContainer(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = SonnerPosition::default())] position: SonnerPosition,
) -> impl IntoView {
    let merged_class = tw_merge!("toast__container fixed z-50", class);

    view! {
        <div class=merged_class data-position=position.to_string()>
            {children()}
        </div>
    }
}

#[component]
pub fn SonnerList(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = SonnerPosition::default())] position: SonnerPosition,
    #[prop(optional, default = SonnerDirection::default())] direction: SonnerDirection,
    #[prop(into, default = "false".to_string())] expanded: String,
    #[prop(into, optional)] style: String,
) -> impl IntoView {
    // pointer-events-none: container doesn't block clicks when empty
    // [&>*]:pointer-events-auto: toast items still receive clicks
    let merged_class = tw_merge!(
        "flex relative flex-col opacity-100 gap-[15px] h-[100px] w-[400px] pointer-events-none [&>*]:pointer-events-auto",
        class
    );

    view! {
        <ol
            class=merged_class
            data-name="SonnerList"
            data-sonner-toaster="true"
            data-sonner-theme="light"
            data-position=position.to_string()
            data-expanded=expanded
            data-direction=direction.to_string()
            style=style
        >
            {children()}
        </ol>
    }
}

#[component]
pub fn SonnerToaster(#[prop(default = SonnerPosition::default())] position: SonnerPosition) -> impl IntoView {
    // Auto-derive direction from position
    let direction = match position {
        SonnerPosition::TopLeft | SonnerPosition::TopCenter | SonnerPosition::TopRight => SonnerDirection::TopDown,
        _ => SonnerDirection::BottomUp,
    };

    let container_class = match position {
        SonnerPosition::TopLeft => "left-6 top-6",
        SonnerPosition::TopRight => "right-6 top-6",
        SonnerPosition::TopCenter => "left-1/2 -translate-x-1/2 top-6",
        SonnerPosition::BottomCenter => "left-1/2 -translate-x-1/2 bottom-6",
        SonnerPosition::BottomLeft => "left-6 bottom-6",
        SonnerPosition::BottomRight => "right-6 bottom-6",
    };

    view! {
        <SonnerContainer class=container_class position=position>
            <SonnerList position=position direction=direction>
                ""
            </SonnerList>
        </SonnerContainer>
    }
}
```
