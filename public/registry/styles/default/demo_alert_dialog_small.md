---
title: "Demo Alert Dialog Small"
name: "demo_alert_dialog_small"
cargo_dependencies: []
registry_dependencies: ["alert_dialog", "button"]
type: "components:demos"
path: "demos/demo_alert_dialog_small.rs"
---

# Demo Alert Dialog Small

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert_dialog_small
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::components::ui::button::Button;

#[component]
pub fn DemoAlertDialogSmall() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Show Dialog"</AlertDialogTrigger>
            <AlertDialogContent class="w-[300px]">
                <AlertDialogBody>
                    <AlertDialogHeader class="items-center sm:items-center sm:text-center">
                        <AlertDialogTitle>"Allow accessory to connect?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Do you want to allow the USB accessory to connect to this device?"
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter class="flex-row">
                        <AlertDialogClose class="flex-1">"Don't allow"</AlertDialogClose>
                        <Button attr:r#type="submit" class="flex-1">
                            "Allow"
                        </Button>
                    </AlertDialogFooter>
                </AlertDialogBody>
            </AlertDialogContent>
        </AlertDialog>
    }
}
```
