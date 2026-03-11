---
title: "Demo Tooltip"
name: "demo_tooltip"
cargo_dependencies: []
registry_dependencies: ["button", "tooltip"]
type: "components:demos"
path: "demos/demo_tooltip.rs"
---

# Demo Tooltip

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_tooltip
```

## Component Code

```rust
use icons::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Link};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition, TooltipProvider};

#[component]
pub fn DemoTooltip() -> impl IntoView {
    view! {
        <TooltipProvider />

        <div class="flex flex-col gap-4">
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowUp />
                </Button>
                <TooltipContent position=TooltipPosition::Top>"TOP"</TooltipContent>
            </Tooltip>
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowLeft />
                </Button>
                <TooltipContent position=TooltipPosition::Left>"LEFT"</TooltipContent>
            </Tooltip>
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowRight />
                </Button>
                <TooltipContent position=TooltipPosition::Right>"RIGHT"</TooltipContent>
            </Tooltip>
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowDown />
                </Button>
                <TooltipContent position=TooltipPosition::Bottom>"BOTTOM"</TooltipContent>
            </Tooltip>

            <Tooltip>
                <Button variant=ButtonVariant::Secondary attr:href="https://rust-ui.com">
                    <Link />
                </Button>
                <TooltipContent position=TooltipPosition::Right>"Link to Rust/UI"</TooltipContent>
            </Tooltip>
        </div>
    }
}
```
