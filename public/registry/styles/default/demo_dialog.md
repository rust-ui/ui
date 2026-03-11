---
title: "Demo Dialog"
name: "demo_dialog"
cargo_dependencies: []
registry_dependencies: ["button", "dialog", "input", "label"]
type: "components:demos"
path: "demos/demo_dialog.rs"
---

# Demo Dialog

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dialog
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn DemoDialog() -> impl IntoView {
    let name_signal = RwSignal::new("Max Wells".to_string());
    let username_signal = RwSignal::new("@maxwells".to_string());

    view! {
        <Dialog>
            <DialogTrigger>Open Dialog</DialogTrigger>

            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Edit profile"</DialogTitle>

                        <DialogDescription>
                            "Make changes to your profile here. Click save when you're done."
                        </DialogDescription>
                    </DialogHeader>

                    <div class="flex flex-col gap-4 justify-center">
                        <div class="flex flex-col gap-2">
                            <Label html_for="name-1">Name</Label>
                            <Input attr:autofocus=true attr:id="name-1" attr:name="name" prop:value=name_signal />
                        </div>
                        <div class="flex flex-col gap-2">
                            <Label html_for="username-1">Username</Label>
                            <Input attr:id="username-1" attr:name="username" prop:value=username_signal />
                        </div>
                    </div>

                    <DialogFooter>
                        <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        <Button attr:r#type="submit" class="w-full sm:w-fit">
                            Save changes
                        </Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}
```
