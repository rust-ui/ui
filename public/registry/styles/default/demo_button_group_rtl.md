---
title: "Demo Button Group Rtl"
name: "demo_button_group_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "button_group", "direction_provider"]
type: "components:demos"
path: "demos/demo_button_group_rtl.rs"
---

# Demo Button Group Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoButtonGroupRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <ButtonGroup>
                <Button variant=ButtonVariant::Outline>"الأول"</Button>
                <Button variant=ButtonVariant::Outline>"الثاني"</Button>
                <Button variant=ButtonVariant::Outline>"الثالث"</Button>
            </ButtonGroup>
        </DirectionProvider>
    }
}
```
