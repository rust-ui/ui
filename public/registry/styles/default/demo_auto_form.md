---
title: "Demo Auto Form"
name: "demo_auto_form"
cargo_dependencies: []
registry_dependencies: ["auto_form", "button"]
type: "components:demos"
path: "demos/demo_auto_form.rs"
---

# Demo Auto Form

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_auto_form
```

## Component Code

```rust
use autoform::AutoForm;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::components::hooks::use_form::use_form;
use crate::components::ui::auto_form::AutoForm;
use crate::components::ui::button::Button;

#[derive(Debug, AutoForm, Validate, Serialize, Deserialize, Clone, Default)]
struct ContactForm {
    #[autoform(label = "Full Name", placeholder = "John Doe")]
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    name: String,

    #[autoform(placeholder = "you@example.com")]
    #[validate(email(message = "Please enter a valid email address"))]
    email: String,

    #[autoform(field_type = "textarea", placeholder = "Tell us about yourself")]
    bio: Option<String>,

    #[autoform(label = "Subscribe to newsletter")]
    subscribe: bool,
}

#[component]
pub fn DemoAutoForm() -> impl IntoView {
    let form = use_form::<ContactForm>();

    // Live JSON display of form values
    let json_display = move || {
        let values = form.values_signal.get();
        serde_json::to_string_pretty(&values).unwrap_or_default()
    };

    view! {
        <div class="flex flex-col gap-8 my-8 w-full max-w-4xl md:flex-row">
            <div class="flex-1">
                <AutoForm form=form>
                    <Button attr:r#type="submit">"Submit"</Button>
                </AutoForm>
            </div>

            <div class="flex-1">
                <h3 class="mb-2 text-sm font-medium whitespace-nowrap text-muted-foreground">"Live Form Data"</h3>
                <pre class="overflow-auto p-4 max-h-80 font-mono text-sm rounded-lg bg-muted">{json_display}</pre>
            </div>
        </div>
    }
}
```
