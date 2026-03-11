---
title: "Demo Drawer"
name: "demo_drawer"
cargo_dependencies: []
registry_dependencies: ["drawer"]
type: "components:demos"
path: "demos/demo_drawer.rs"
---

# Demo Drawer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawer() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />
                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Drawer Title"</DrawerTitle>
                        <DrawerDescription>"Drag down to close or click outside."</DrawerDescription>
                    </DrawerHeader>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
```
