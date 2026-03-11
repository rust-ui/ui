---
title: "Avatar"
name: "avatar"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/avatar.rs"
description: "Rust/UI component that displays an avatar with image and fallback support."
tags: []
---

# Avatar

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add avatar
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Default,
    Lg,
}

/* ========================================================== */
/*                     Components (clx!)                      */
/* ========================================================== */

mod components {
    use super::*;
    clx! {AvatarFallback, div, "absolute inset-0 flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground group-data-[size=sm]/avatar:text-xs"}
    clx! {AvatarGroup, div, "group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background"}
    clx! {AvatarGroupCount, div, "relative flex size-8 shrink-0 items-center justify-center rounded-full bg-muted text-sm text-muted-foreground ring-2 ring-background group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&>svg]:size-4 group-has-data-[size=lg]/avatar-group:[&>svg]:size-5 group-has-data-[size=sm]/avatar-group:[&>svg]:size-3"}
}

pub use components::*;

#[component]
pub fn AvatarBadge(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-primary text-primary-foreground ring-2 ring-background select-none group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&>svg]:hidden group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&>svg]:size-2 group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&>svg]:size-2",
        class
    );
    view! { <span class=merged_class>{children.map(|c| c())}</span> }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Avatar(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: AvatarSize,
) -> impl IntoView {
    let size_str = match size {
        AvatarSize::Sm => "sm",
        AvatarSize::Default => "default",
        AvatarSize::Lg => "lg",
    };

    let merged_class = tw_merge!(
        "group/avatar relative flex size-8 shrink-0 overflow-hidden rounded-full select-none after:absolute after:inset-0 after:rounded-full after:border after:border-border after:mix-blend-darken data-[size=lg]:size-10 data-[size=sm]:size-6 dark:after:mix-blend-lighten",
        class
    );

    view! {
        <div class=merged_class data-slot="avatar" data-size=size_str>
            {children()}
        </div>
    }
}

#[component]
pub fn AvatarImage(#[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("absolute inset-0 aspect-square size-full z-10 rounded-full object-cover", class);
    let node_ref = NodeRef::<leptos::html::Img>::new();

    view! {
        <img
            node_ref=node_ref
            class=merged_class
            data-slot="avatar-image"
            on:error=move |_| {
                if let Some(img) = node_ref.get() {
                    let _ = img.set_attribute("style", "display: none;");
                }
            }
        />
    }
}
```
