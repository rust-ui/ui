---
title: "Demo Alert Dialog Media"
name: "demo_alert_dialog_media"
cargo_dependencies: []
registry_dependencies: ["alert_dialog", "button"]
type: "components:demos"
path: "demos/demo_alert_dialog_media.rs"
---

# Demo Alert Dialog Media

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert_dialog_media
```

## Component Code

```rust
use icons::CirclePlus;
use leptos::prelude::*;

use crate::components::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::components::ui::button::Button;

#[component]
pub fn DemoAlertDialogMedia() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Share Project"</AlertDialogTrigger>
            <AlertDialogContent class="w-[425px]">
                <AlertDialogBody>
                    <AlertDialogHeader>
                        <div class="flex justify-center items-center mb-2 rounded-md size-10 bg-muted">
                            <CirclePlus class="size-5" />
                        </div>
                        <AlertDialogTitle>"Share this project?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Anyone with the link will be able to view and edit this project."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogClose class="w-full sm:w-fit">"Cancel"</AlertDialogClose>
                        <Button attr:r#type="submit" class="w-full sm:w-fit">
                            "Share"
                        </Button>
                    </AlertDialogFooter>
                </AlertDialogBody>
            </AlertDialogContent>
        </AlertDialog>
    }
}
```
