---
title: "Demo Drawer Focus"
name: "demo_drawer_focus"
cargo_dependencies: []
registry_dependencies: ["drawer", "input", "label", "textarea"]
type: "components:demos"
path: "demos/demo_drawer_focus.rs"
---

# Demo Drawer Focus

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer_focus
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::textarea::Textarea;

#[component]
pub fn DemoDrawerFocus() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />

                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Focus Drawer"</DrawerTitle>
                        <DrawerDescription>
                            "Test keyboard navigation: Press Tab to cycle through elements, Shift+Tab to go back, and Escape to close."
                        </DrawerDescription>
                    </DrawerHeader>

                    <form class="flex flex-col gap-4">
                        <div class="flex flex-col gap-2">
                            <Label html_for="test-input">"Text Input"</Label>
                            <Input attr:id="test-input" attr:placeholder="Type something..." />
                        </div>

                        <div class="flex flex-col gap-2">
                            <Label html_for="test-email">"Email"</Label>
                            <Input attr:id="test-email" attr:r#type="email" attr:placeholder="email@example.com" />
                        </div>

                        <div class="flex flex-col gap-2">
                            <Label html_for="test-textarea">"Message"</Label>
                            <Textarea attr:id="test-textarea" attr:rows="4" attr:placeholder="Write a message..." />
                        </div>
                    </form>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
```
