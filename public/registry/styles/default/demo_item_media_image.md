---
title: "Demo Item Media Image"
name: "demo_item_media_image"
cargo_dependencies: []
registry_dependencies: ["badge", "item"]
type: "components:demos"
path: "demos/demo_item_media_image.rs"
---

# Demo Item Media Image

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_item_media_image
```

## Component Code

```rust
use icons::ChevronRight;
use leptos::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemMedia, ItemMediaVariant, ItemSeparator,
    ItemTitle,
};

#[component]
pub fn DemoItemMediaImage() -> impl IntoView {
    view! {
        <ItemGroup attr:role="list" class="w-full max-w-md rounded-md border">
            <Item class="py-3 px-4 rounded-none">
                <ItemMedia variant=ItemMediaVariant::Image>
                    <div class="bg-gradient-to-br from-blue-400 to-blue-600 size-full" />
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Getting Started with Leptos"</ItemTitle>
                    <ItemDescription>"Learn how to build reactive web apps in Rust."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
                <ItemFooter>
                    <Badge variant=BadgeVariant::Secondary>"Tutorial"</Badge>
                    <span class="text-xs text-muted-foreground">"Jan 2026"</span>
                </ItemFooter>
            </Item>
            <ItemSeparator />
            <Item class="py-3 px-4 rounded-none">
                <ItemMedia variant=ItemMediaVariant::Image>
                    <div class="bg-gradient-to-br from-violet-400 to-violet-600 size-full" />
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Tailwind CSS Best Practices"</ItemTitle>
                    <ItemDescription>"Utility-first CSS patterns for modern apps."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
                <ItemFooter>
                    <Badge variant=BadgeVariant::Secondary>"CSS"</Badge>
                    <span class="text-xs text-muted-foreground">"Feb 2026"</span>
                </ItemFooter>
            </Item>
            <ItemSeparator />
            <Item class="py-3 px-4 rounded-none">
                <ItemMedia variant=ItemMediaVariant::Image>
                    <div class="bg-gradient-to-br from-emerald-400 to-emerald-600 size-full" />
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Rust UI Component System"</ItemTitle>
                    <ItemDescription>"Building a type-safe design system in Rust."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
                <ItemFooter>
                    <Badge variant=BadgeVariant::Secondary>"Design"</Badge>
                    <span class="text-xs text-muted-foreground">"Mar 2026"</span>
                </ItemFooter>
            </Item>
        </ItemGroup>
    }
}
```
