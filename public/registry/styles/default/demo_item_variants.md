---
title: "Demo Item Variants"
name: "demo_item_variants"
cargo_dependencies: []
registry_dependencies: ["button", "item"]
type: "components:demos"
path: "demos/demo_item_variants.rs"
---

# Demo Item Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_item_variants
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::item::{Item, ItemActions, ItemContent, ItemDescription, ItemTitle, ItemVariant};

#[component]
pub fn DemoItemVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <Item>
                <ItemContent>
                    <ItemTitle>"Default Variant"</ItemTitle>
                    <ItemDescription>"Standard styling with subtle background and borders."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Open"
                    </Button>
                </ItemActions>
            </Item>

            <Item variant=ItemVariant::Outline>
                <ItemContent>
                    <ItemTitle>"Outline Variant"</ItemTitle>
                    <ItemDescription>"Outlined style with clear borders and transparent background."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Open"
                    </Button>
                </ItemActions>
            </Item>

            <Item variant=ItemVariant::Muted>
                <ItemContent>
                    <ItemTitle>"Muted Variant"</ItemTitle>
                    <ItemDescription>"Subdued appearance with muted colors for secondary content."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Open"
                    </Button>
                </ItemActions>
            </Item>
        </div>
    }
}
```
