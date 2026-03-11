---
title: "Demo Form"
name: "demo_form"
cargo_dependencies: []
registry_dependencies: ["form"]
type: "components:demos"
path: "demos/demo_form.rs"
---

# Demo Form

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_form
```

## Component Code

```rust
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::components::hooks::use_form::use_form;
use crate::components::ui::form::{Form, FormDescription, FormField, FormGroup, FormInput, FormLabel, FormProvider, FormSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
struct FormData {
    username: String,
    password: String,
}

#[component]
pub fn DemoForm() -> impl IntoView {
    let form = use_form::<FormData>();

    view! {
        <FormProvider form=form>
            <Form class="max-w-md">
                <FormSet>
                    <FormGroup>
                        <FormField field="username">
                            <FormLabel>Username</FormLabel>
                            <FormInput attr:placeholder="Max Wells" />
                            <FormDescription>Choose a unique username for your account.</FormDescription>
                        </FormField>
                        <FormField field="password">
                            <FormLabel>Password</FormLabel>
                            <FormDescription>Must be at least 8 characters long.</FormDescription>
                            <FormInput attr:r#type="password" attr:placeholder="********" />
                        </FormField>
                    </FormGroup>
                </FormSet>
            </Form>
        </FormProvider>
    }
}
```
