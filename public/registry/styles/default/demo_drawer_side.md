---
title: "Demo Drawer Side"
name: "demo_drawer_side"
cargo_dependencies: []
registry_dependencies: ["drawer"]
type: "components:demos"
path: "demos/demo_drawer_side.rs"
---

# Demo Drawer Side

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer_side
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerContent, DrawerDescription, DrawerHeader, DrawerPosition, DrawerTitle, DrawerTrigger,
};

#[component]
pub fn DemoDrawerSide() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent
                position=DrawerPosition::Right
                class="top-0 bottom-0 left-auto h-full max-h-full rounded-t-none w-[300px] rounded-l-[10px]"
            >
                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Different Directions"</DrawerTitle>
                        <DrawerDescription>"It supports all directions."</DrawerDescription>
                        <DrawerDescription>
                            "This one specifically is not touching the edge of the screen, but that is not required for a side drawer."
                        </DrawerDescription>
                    </DrawerHeader>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
```
