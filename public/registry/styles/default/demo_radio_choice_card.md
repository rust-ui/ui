---
title: "Demo Radio Choice Card"
name: "demo_radio_choice_card"
cargo_dependencies: []
registry_dependencies: ["field", "radio_button"]
type: "components:demos"
path: "demos/demo_radio_choice_card.rs"
---

# Demo Radio Choice Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_radio_choice_card
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::field::{Field, FieldContent, FieldDescription, FieldLabel, FieldTitle, FieldVariant};
use crate::components::ui::radio_button::{RadioGroup, RadioGroupItem};

#[component]
pub fn DemoRadioChoiceCard() -> impl IntoView {
    let selected = RwSignal::new("pro".to_string());

    view! {
        <RadioGroup value=selected class="w-full max-w-sm">
            <FieldLabel html_for="plan-plus">
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Plus"</FieldTitle>
                        <FieldDescription>"For individuals and small teams."</FieldDescription>
                    </FieldContent>
                    <RadioGroupItem value="plus" id="plan-plus" />
                </Field>
            </FieldLabel>
            <FieldLabel html_for="plan-pro">
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Pro"</FieldTitle>
                        <FieldDescription>"For growing businesses."</FieldDescription>
                    </FieldContent>
                    <RadioGroupItem value="pro" id="plan-pro" />
                </Field>
            </FieldLabel>
            <FieldLabel html_for="plan-enterprise">
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Enterprise"</FieldTitle>
                        <FieldDescription>"For large teams and enterprises."</FieldDescription>
                    </FieldContent>
                    <RadioGroupItem value="enterprise" id="plan-enterprise" />
                </Field>
            </FieldLabel>
        </RadioGroup>
    }
}
```
