---
title: "Demo Sheet No Close Button"
name: "demo_sheet_no_close_button"
cargo_dependencies: []
registry_dependencies: ["button", "sheet"]
type: "components:demos"
path: "demos/demo_sheet_no_close_button.rs"
---

# Demo Sheet No Close Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_sheet_no_close_button
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheetNoCloseButton() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Open Sheet"</SheetTrigger>

            <SheetContent direction=SheetDirection::Right show_close_button=false>
                <SheetHeader>
                    <SheetTitle>"No Close Button"</SheetTitle>
                    <SheetDescription>"This sheet hides the default close button."</SheetDescription>
                </SheetHeader>

                <div class="p-4 text-sm text-muted-foreground">"Use the Cancel button or press ESC to close."</div>

                <SheetFooter>
                    <SheetClose variant=ButtonVariant::Outline>"Cancel"</SheetClose>
                    <Button>"Confirm"</Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
}
```
