---
title: "Demo Input Group Tooltip"
name: "demo_input_group_tooltip"
cargo_dependencies: []
registry_dependencies: ["field", "input_group", "tooltip"]
type: "components:demos"
path: "demos/demo_input_group_tooltip.rs"
---

# Demo Input Group Tooltip

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_tooltip
```

## Component Code

```rust
use icons::Info;
use leptos::prelude::*;

use crate::components::ui::field::{Field, FieldGroup, FieldLabel};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupInput,
};
use crate::components::ui::tooltip::{Tooltip, TooltipContent};

#[component]
pub fn DemoInputGroupTooltip() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field>
                <FieldLabel html_for="email-tooltip">"Email"</FieldLabel>
                <InputGroup>
                    <InputGroupInput id="email-tooltip" placeholder="you@example.com" />
                </InputGroup>
            </Field>

            <Field>
                <FieldLabel html_for="username-tooltip">
                    <div class="flex justify-between items-center w-full">
                        "Username" <Tooltip>
                            <InputGroupButton
                                size=InputGroupButtonSize::IconXs
                                class="rounded-full"
                                attr:aria-label="Help"
                            >
                                <Info />
                            </InputGroupButton>
                            <TooltipContent>"Must be 3–20 characters, letters and numbers only."</TooltipContent>
                        </Tooltip>
                    </div>
                </FieldLabel>
                <InputGroup>
                    <InputGroupAddon align=InputGroupAddonAlign::BlockStart class="border-b">
                        <span class="text-sm font-medium text-foreground">"Username"</span>
                        <Tooltip>
                            <InputGroupButton
                                size=InputGroupButtonSize::IconXs
                                class="ml-auto rounded-full"
                                attr:aria-label="Help"
                            >
                                <Info />
                            </InputGroupButton>
                            <TooltipContent>"Must be 3–20 characters, letters and numbers only."</TooltipContent>
                        </Tooltip>
                    </InputGroupAddon>
                    <InputGroupInput id="username-tooltip" placeholder="john_doe" />
                </InputGroup>
            </Field>
        </FieldGroup>
    }
}
```
