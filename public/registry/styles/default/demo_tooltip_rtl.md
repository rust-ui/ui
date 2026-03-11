---
title: "Demo Tooltip Rtl"
name: "demo_tooltip_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "direction_provider", "tooltip"]
type: "components:demos"
path: "demos/demo_tooltip_rtl.rs"
---

# Demo Tooltip Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_tooltip_rtl
```

## Component Code

```rust
use icons::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition, TooltipProvider};

#[component]
pub fn DemoTooltipRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <TooltipProvider />

            <div class="flex flex-col gap-4">
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowUp />
                    </Button>
                    <TooltipContent position=TooltipPosition::Top>"أعلى"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowLeft />
                    </Button>
                    <TooltipContent position=TooltipPosition::Left>"يسار"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowRight />
                    </Button>
                    <TooltipContent position=TooltipPosition::Right>"يمين"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowDown />
                    </Button>
                    <TooltipContent position=TooltipPosition::Bottom>"أسفل"</TooltipContent>
                </Tooltip>
            </div>
        </DirectionProvider>
    }
}
```
