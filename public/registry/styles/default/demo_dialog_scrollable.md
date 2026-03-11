---
title: "Demo Dialog Scrollable"
name: "demo_dialog_scrollable"
cargo_dependencies: []
registry_dependencies: ["button", "dialog", "scroll_area"]
type: "components:demos"
path: "demos/demo_dialog_scrollable.rs"
---

# Demo Dialog Scrollable

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dialog_scrollable
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::components::ui::scroll_area::ScrollArea;

#[component]
pub fn DemoDialogScrollable() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>Open Scrollable Dialog</DialogTrigger>

            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Terms of Service"</DialogTitle>

                        <DialogDescription>
                            "Please read and accept our terms of service before continuing."
                        </DialogDescription>
                    </DialogHeader>

                    <ScrollArea class="flex flex-col gap-4 pr-2 max-h-[300px]">
                        <p class="text-sm text-muted-foreground">
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris."
                        </p>

                        <p class="text-sm text-muted-foreground">
                            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                        </p>

                        <p class="text-sm text-muted-foreground">
                            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."
                        </p>

                        <p class="text-sm text-muted-foreground">
                            "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt."
                        </p>

                        <p class="text-sm text-muted-foreground">
                            "Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem."
                        </p>

                        <p class="text-sm text-muted-foreground">
                            "Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur."
                        </p>
                    </ScrollArea>

                    <DialogFooter>
                        <DialogClose>"Decline"</DialogClose>
                        <Button>"Accept"</Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}
```
