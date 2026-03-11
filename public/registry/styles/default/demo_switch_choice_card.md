---
title: "Demo Switch Choice Card"
name: "demo_switch_choice_card"
cargo_dependencies: []
registry_dependencies: ["field", "switch"]
type: "components:demos"
path: "demos/demo_switch_choice_card.rs"
---

# Demo Switch Choice Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_switch_choice_card
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::field::{Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldTitle, FieldVariant};
use crate::components::ui::switch::Switch;

#[component]
pub fn DemoSwitchChoiceCard() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-sm">
            <FieldLabel>
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Share across devices"</FieldTitle>
                        <FieldDescription>
                            "Focus is shared across devices, and turns off when you leave the app."
                        </FieldDescription>
                    </FieldContent>
                    <Switch id="switch-share" />
                </Field>
            </FieldLabel>
            <FieldLabel>
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Enable notifications"</FieldTitle>
                        <FieldDescription>
                            "Receive notifications when focus mode is enabled or disabled."
                        </FieldDescription>
                    </FieldContent>
                    <Switch id="switch-notifications" checked=true />
                </Field>
            </FieldLabel>
        </FieldGroup>
    }
}
```
