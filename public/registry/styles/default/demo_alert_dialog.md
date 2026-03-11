---
title: "Demo Alert Dialog"
name: "demo_alert_dialog"
cargo_dependencies: []
registry_dependencies: ["alert_dialog", "button"]
type: "components:demos"
path: "demos/demo_alert_dialog.rs"
---

# Demo Alert Dialog

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert_dialog
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
pub fn DemoAlertDialog() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>Open Alert</AlertDialogTrigger>

            <AlertDialogContent class="w-[425px]">
                <AlertDialogBody>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>

                        <AlertDialogDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </AlertDialogDescription>
                    </AlertDialogHeader>

                    <AlertDialogFooter>
                        <AlertDialogClose class="w-full sm:w-fit">"Cancel"</AlertDialogClose>
                        <Button attr:r#type="submit" class="w-full sm:w-fit">
                            "Continue"
                        </Button>
                    </AlertDialogFooter>
                </AlertDialogBody>
            </AlertDialogContent>
        </AlertDialog>
    }
}
```
