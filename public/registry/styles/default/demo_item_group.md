---
title: "Demo Item Group"
name: "demo_item_group"
cargo_dependencies: []
registry_dependencies: ["avatar", "badge", "item"]
type: "components:demos"
path: "demos/demo_item_group.rs"
---

# Demo Item Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_item_group
```

## Component Code

```rust
use icons::ChevronRight;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia, ItemSeparator, ItemTitle,
};

#[component]
pub fn DemoItemGroup() -> impl IntoView {
    view! {
        <ItemGroup attr:role="list" class="w-full max-w-md rounded-md border">
            <Item class="py-3 px-4 rounded-none">
                <ItemHeader>
                    <Badge variant=BadgeVariant::Secondary>"Message"</Badge>
                    <span class="text-xs text-muted-foreground">"2m ago"</span>
                </ItemHeader>
                <ItemMedia>
                    <Avatar class="size-8">
                        <AvatarFallback>"RS"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Ryan Smith sent a message"</ItemTitle>
                    <ItemDescription>"Hey, are you free for a quick call?"</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
            </Item>
            <ItemSeparator />
            <Item class="py-3 px-4 rounded-none">
                <ItemHeader>
                    <Badge variant=BadgeVariant::Secondary>"Alert"</Badge>
                    <span class="text-xs text-muted-foreground">"15m ago"</span>
                </ItemHeader>
                <ItemMedia>
                    <Avatar class="size-8">
                        <AvatarFallback>"SY"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Deploy completed"</ItemTitle>
                    <ItemDescription>"Production build succeeded with no errors."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
            </Item>
            <ItemSeparator />
            <Item class="py-3 px-4 rounded-none">
                <ItemHeader>
                    <Badge variant=BadgeVariant::Secondary>"Update"</Badge>
                    <span class="text-xs text-muted-foreground">"1h ago"</span>
                </ItemHeader>
                <ItemMedia>
                    <Avatar class="size-8">
                        <AvatarFallback>"MK"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Morgan updated the docs"</ItemTitle>
                    <ItemDescription>"Installation guide revised for Leptos 0.7."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
            </Item>
        </ItemGroup>
    }
}
```
