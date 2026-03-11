---
title: "Demo Drawer Dialog"
name: "demo_drawer_dialog"
cargo_dependencies: []
registry_dependencies: ["button", "dialog", "drawer", "input"]
type: "components:demos"
path: "demos/demo_drawer_dialog.rs"
---

# Demo Drawer Dialog

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer_dialog
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHandle, DrawerHeader,
    DrawerTitle, DrawerTrigger,
};
use crate::components::ui::input::Input;

#[component]
pub fn DemoDrawerDialog() -> impl IntoView {
    view! {
        // Mobile — hidden on md+
        <div class="md:hidden">
            <Drawer>
                <DrawerTrigger>"Subscribe"</DrawerTrigger>
                <DrawerContent>
                    <DrawerHandle />
                    <DrawerBody>
                        <DrawerHeader>
                            <DrawerTitle>"Subscribe"</DrawerTitle>
                            <DrawerDescription>"Get the latest updates delivered to your inbox."</DrawerDescription>
                        </DrawerHeader>
                        <Input attr:r#type="email" attr:placeholder="you@example.com" />
                        <DrawerFooter>
                            <DrawerClose class="w-full sm:w-fit">"Cancel"</DrawerClose>
                            <Button class="w-full sm:w-fit">"Subscribe"</Button>
                        </DrawerFooter>
                    </DrawerBody>
                </DrawerContent>
            </Drawer>
        </div>

        // Desktop — hidden below md
        <div class="hidden md:block">
            <Dialog>
                <DialogTrigger>"Subscribe"</DialogTrigger>
                <DialogContent class="sm:max-w-[400px]">
                    <DialogBody>
                        <DialogHeader>
                            <DialogTitle>"Subscribe"</DialogTitle>
                            <DialogDescription>"Get the latest updates delivered to your inbox."</DialogDescription>
                        </DialogHeader>
                        <Input attr:r#type="email" attr:placeholder="you@example.com" />
                        <DialogFooter>
                            <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                            <Button class="w-full sm:w-fit">"Subscribe"</Button>
                        </DialogFooter>
                    </DialogBody>
                </DialogContent>
            </Dialog>
        </div>
    }
}
```
