---
title: "Demo Radio Button"
name: "demo_radio_button"
cargo_dependencies: []
registry_dependencies: ["label", "radio_button"]
type: "components:demos"
path: "demos/demo_radio_button.rs"
---

# Demo Radio Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_radio_button
```

## Component Code

```rust
use leptos::prelude::*;
use strum::{IntoStaticStr, VariantArray};

use crate::components::ui::label::Label;
use crate::components::ui::radio_button::{RadioGroup, RadioGroupItem};

#[derive(Clone, Copy, Default, VariantArray, IntoStaticStr)]
enum Frequency {
    Daily,
    #[default]
    Weekly,
    Monthly,
}

#[component]
pub fn DemoRadioButton() -> impl IntoView {
    let default: &'static str = Frequency::default().into();
    let value_signal = RwSignal::new(default.to_string());

    view! {
        <RadioGroup value=value_signal>
            {Frequency::VARIANTS
                .iter()
                .map(|variant| {
                    let name: &'static str = (*variant).into();
                    view! {
                        <div class="flex gap-3 items-center">
                            <RadioGroupItem value=name id=name />
                            <Label html_for=name>{name}</Label>
                        </div>
                    }
                })
                .collect_view()}
        </RadioGroup>
    }
}
```
