---
title: "Demo Chips"
name: "demo_chips"
cargo_dependencies: []
registry_dependencies: ["chips"]
type: "components:demos"
path: "demos/demo_chips.rs"
---

# Demo Chips

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_chips
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::chips::{ChipItem, ChipsContainer};

#[component]
pub fn DemoChips() -> impl IntoView {
    view! {
        <ChipsContainer>
            <ChipItem label="sunny" />
            <ChipItem label="cloudy" />
            <ChipItem label="rainy" />
            <ChipItem label="windy" />
            <ChipItem label="stormy" />
            <ChipItem label="foggy" />
            <ChipItem label="snowy" />
            <ChipItem label="icy" />
            <ChipItem label="humid" />
            <ChipItem label="dry" />
            <ChipItem label="hot" />
            <ChipItem label="cold" />
            <ChipItem label="warm" />
            <ChipItem label="chilly" />
            <ChipItem label="breezy" />
            <ChipItem label="gusty" />
            <ChipItem label="hazy" />
        </ChipsContainer>
    }
}
```
