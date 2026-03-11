---
title: "Demo Sheet"
name: "demo_sheet"
cargo_dependencies: []
registry_dependencies: ["button", "sheet"]
type: "components:demos"
path: "demos/demo_sheet.rs"
---

# Demo Sheet

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_sheet
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
pub fn DemoSheet() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Open Sheet"</SheetTrigger>

            <SheetContent direction=SheetDirection::Right>
                <SheetHeader>
                    <SheetTitle>"Edit Profile"</SheetTitle>
                    <SheetDescription>"Make changes to your profile here."</SheetDescription>
                </SheetHeader>

                <div class="p-4 text-sm text-muted-foreground">
                    "Update your display name, avatar, and other profile details."
                </div>

                <SheetFooter>
                    <SheetClose variant=ButtonVariant::Outline>"Cancel"</SheetClose>
                    <Button>"Save changes"</Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
}
```
