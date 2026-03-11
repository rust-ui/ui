---
title: "Demo Drawer Rtl"
name: "demo_drawer_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "drawer"]
type: "components:demos"
path: "demos/demo_drawer_rtl.rs"
---

# Demo Drawer Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <Drawer>
                <DrawerTrigger>"فتح الدرج"</DrawerTrigger>

                <DrawerContent>
                    <DrawerHandle />
                    <DrawerBody>
                        <DrawerHeader>
                            <DrawerTitle>"عنوان الدرج"</DrawerTitle>
                            <DrawerDescription>
                                "اسحب للأسفل للإغلاق أو انقر خارج الدرج."
                            </DrawerDescription>
                        </DrawerHeader>

                        <DrawerClose>"إغلاق"</DrawerClose>
                    </DrawerBody>
                </DrawerContent>
            </Drawer>
        </DirectionProvider>
    }
}
```
