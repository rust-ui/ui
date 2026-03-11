---
title: "Demo Drawer Non Dismissable"
name: "demo_drawer_non_dismissable"
cargo_dependencies: []
registry_dependencies: ["drawer"]
type: "components:demos"
path: "demos/demo_drawer_non_dismissable.rs"
---

# Demo Drawer Non Dismissable

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer_non_dismissable
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerNonDismissable() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent style="--initial-transform: 100%; pointer-events: auto;" dismissible="false">
                <DrawerHandle />

                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Are you absolutely sure?"</DrawerTitle>
                        <DrawerDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </DrawerDescription>
                    </DrawerHeader>

                    <DrawerClose>"Confirm"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
```
