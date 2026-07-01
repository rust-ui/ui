---
title: "Demo Bubble Popover"
name: "demo_bubble_popover"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["bubble", "button", "popover"]
type: "components:demos"
path: "demos/demo_bubble_popover.rs"
---

# Demo Bubble Popover

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bubble_popover
```

## Component Code

```rust
use icons::Info;
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoBubblePopover() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-4 py-12">
            <Bubble align=BubbleAlign::End>
                <BubbleContent>"Run the build script."</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Destructive>
                <BubbleContent>"Failed to run the command."</BubbleContent>
                <BubbleReactions>
                    // TODO PORT: shadcn uses PopoverTrigger render={<Button .../>} (asChild pattern).
                    // Ported as Button inside PopoverTrigger.
                    <Popover>
                        <PopoverTrigger>
                            <Button
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconXs
                                attr:aria-label="Show error details"
                                class="aria-expanded:text-destructive"
                            >
                                <Info />
                            </Button>
                        </PopoverTrigger>
                        <PopoverContent>
                            <PopoverTitle class="text-sm">
                                "Command failed with exit code 1"
                            </PopoverTitle>
                            <PopoverDescription class="text-sm">
                                "ENOENT: no such file or directory, open pnpm-lock.yaml"
                            </PopoverDescription>
                        </PopoverContent>
                    </Popover>
                </BubbleReactions>
            </Bubble>
        </div>
    }
}
```
