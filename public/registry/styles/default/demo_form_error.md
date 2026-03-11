---
title: "Demo Form Error"
name: "demo_form_error"
cargo_dependencies: []
registry_dependencies: ["form", "input"]
type: "components:demos"
path: "demos/demo_form_error.rs"
---

# Demo Form Error

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_form_error
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::form::{FormError, FormFieldWrapper, FormLabel};
use crate::components::ui::input::Input;

#[component]
pub fn DemoFormError() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FormFieldWrapper attr:data-invalid="true">
                <FormLabel html_for="email">Email</FormLabel>
                <Input attr:id="email" attr:r#type="email" attr:aria-invalid="true" />
                <FormError>Enter a valid email address.</FormError>
            </FormFieldWrapper>
        </div>
    }
}
```
