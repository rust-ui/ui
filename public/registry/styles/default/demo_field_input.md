---
title: "Demo Field Input"
name: "demo_field_input"
cargo_dependencies: []
registry_dependencies: ["field", "input"]
type: "components:demos"
path: "demos/demo_field_input.rs"
---

# Demo Field Input

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_field_input
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::field::{Field, FieldDescription, FieldGroup, FieldLabel, FieldSet};
use crate::components::ui::input::Input;

#[component]
pub fn DemoFieldInput() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldGroup>
                <Field>
                    <FieldLabel html_for="username">Username</FieldLabel>
                    <Input attr:id="username" placeholder="Max Leiter" />
                    <FieldDescription>Choose a unique username for your account.</FieldDescription>
                </Field>
                <Field>
                    <FieldLabel html_for="password">Password</FieldLabel>
                    <FieldDescription>Must be at least 8 characters long.</FieldDescription>
                    <Input attr:id="password" attr:r#type="password" placeholder="••••••••" />
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
```
