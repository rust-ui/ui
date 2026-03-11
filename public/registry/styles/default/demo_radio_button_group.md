---
title: "Demo Radio Button Group"
name: "demo_radio_button_group"
cargo_dependencies: []
registry_dependencies: ["radio_button_group"]
type: "components:demos"
path: "demos/demo_radio_button_group.rs"
---

# Demo Radio Button Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_radio_button_group
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::radio_button_group::{RadioButton, RadioButtonGroup, RadioButtonText};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoRadioButtonGroup() -> impl IntoView {
    view! {
        <style>
            {".radio__button[type=\"radio\"] {
            clip: rect(0 0 0 0);
            clip-path: inset(100%);
            height: 1px;
            overflow: hidden;
            position: absolute;
            white-space: nowrap;
            width: 1px;
            }
            
            .radio__button[type=\"radio\"]:checked + span {
            box-shadow: 0 0 0 0.0625em var(--primary);
            background-color: var(--secondary);
            z-index: 1;
            color: var(--primary);
            }"}
        </style>

        <RadioButtonGroup>
            <RadioButton checked=true>
                <RadioButtonText>Women</RadioButtonText>
            </RadioButton>
            <RadioButton>
                <RadioButtonText>Men</RadioButtonText>
            </RadioButton>
            <RadioButton>
                <RadioButtonText>Divided</RadioButtonText>
            </RadioButton>
        </RadioButtonGroup>
    }
}
```
