---
title: "Demo Bubble Tooltip"
name: "demo_bubble_tooltip"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["bubble", "button", "tooltip"]
type: "components:demos"
path: "demos/demo_bubble_tooltip.rs"
---

# Demo Bubble Tooltip

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bubble_tooltip
```

## Component Code

```rust
use icons::Check;
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipProvider};

#[component]
pub fn DemoBubbleTooltip() -> impl IntoView {
    view! {
        <TooltipProvider />
        <div class="flex w-full max-w-sm flex-col gap-4 py-12">
            <Bubble variant=BubbleVariant::Secondary>
                <BubbleContent>"Did you remove the stale route?"</BubbleContent>
            </Bubble>
            <Bubble align=BubbleAlign::End>
                <BubbleContent>"Yes, removed it from the registry."</BubbleContent>
                <BubbleReactions>
                    // TODO PORT: shadcn uses TooltipTrigger render={<Button .../>} (asChild pattern).
                    // Ported as Button inside Tooltip — Tooltip wraps the trigger element directly.
                    <Tooltip>
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::IconXs>
                            <Check />
                        </Button>
                        <TooltipContent>"Read on Jan 5, 2026 at 4:32 PM"</TooltipContent>
                    </Tooltip>
                </BubbleReactions>
            </Bubble>
        </div>
    }
}
```
