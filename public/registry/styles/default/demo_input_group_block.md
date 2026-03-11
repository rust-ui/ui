---
title: "Demo Input Group Block"
name: "demo_input_group_block"
cargo_dependencies: []
registry_dependencies: ["badge", "input_group"]
type: "components:demos"
path: "demos/demo_input_group_block.rs"
---

# Demo Input Group Block

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_block
```

## Component Code

```rust
use icons::{ArrowUp, AtSign, ChevronDown, Globe, Paperclip, WandSparkles};
use leptos::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupTextarea,
};

#[component]
pub fn DemoInputGroupBlock() -> impl IntoView {
    view! {
        <div class="w-full max-w-lg">
            <InputGroup>
                <InputGroupAddon align=InputGroupAddonAlign::BlockStart>
                    <InputGroupButton size=InputGroupButtonSize::Xs>
                        <AtSign />
                        "Add context"
                    </InputGroupButton>
                    <Badge variant=BadgeVariant::Secondary>"Vision"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Research"</Badge>
                </InputGroupAddon>

                <InputGroupTextarea attr:placeholder="Ask anything..." attr:rows="3" />

                <InputGroupAddon align=InputGroupAddonAlign::BlockEnd>
                    <InputGroupButton size=InputGroupButtonSize::IconXs>
                        <Paperclip />
                    </InputGroupButton>
                    <InputGroupButton size=InputGroupButtonSize::Xs>
                        <WandSparkles />
                        "Auto"
                        <ChevronDown />
                    </InputGroupButton>
                    <InputGroupButton size=InputGroupButtonSize::Xs>
                        <Globe />
                        "Sources"
                    </InputGroupButton>
                    <div class="ml-auto">
                        <InputGroupButton
                            size=InputGroupButtonSize::IconXs
                            class="bg-primary text-primary-foreground hover:bg-primary/90"
                        >
                            <ArrowUp />
                        </InputGroupButton>
                    </div>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
```
