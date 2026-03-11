---
title: "Demo Input Group Rtl"
name: "demo_input_group_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "input_group"]
type: "components:demos"
path: "demos/demo_input_group_rtl.rs"
---

# Demo Input Group Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_rtl
```

## Component Code

```rust
use icons::{Mail, Search};
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoInputGroupRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="grid gap-6 w-full max-w-sm">
                <InputGroup>
                    <InputGroupInput placeholder="بحث..." />
                    <InputGroupAddon>
                        <Search />
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput placeholder="أدخل بريدك الإلكتروني" />
                    <InputGroupAddon>
                        <Mail />
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput placeholder="أدخل النص هنا" />
                    <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                        <Search />
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </DirectionProvider>
    }
}
```
