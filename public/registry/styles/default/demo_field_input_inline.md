---
title: "Demo Field Input Inline"
name: "demo_field_input_inline"
cargo_dependencies: []
registry_dependencies: ["button", "field", "input"]
type: "components:demos"
path: "demos/demo_field_input_inline.rs"
---

# Demo Field Input Inline

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_field_input_inline
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::field::{Field, FieldVariant};
use crate::components::ui::input::Input;

#[component]
pub fn DemoFieldInputInline() -> impl IntoView {
    view! {
        <Field variant=FieldVariant::Horizontal>
            <Input attr:r#type="search" placeholder="Search..." />
            <Button>"Search"</Button>
        </Field>
    }
}
```
