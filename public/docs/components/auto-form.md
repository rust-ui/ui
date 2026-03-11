+++
title = "AutoForm"
description = "Automatically generate form UI from Rust structs using a derive macro with built-in validation support."
tags = ["input"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticAutoForm />




## Installation

<StaticInstallAutoForm />



## Usage

AutoForm uses a derive macro to automatically generate form fields from your struct definition. Simply annotate your struct with `#[derive(AutoForm)]` and use the `#[autoform(...)]` attribute to customize field rendering.

```rust
use autoform::AutoForm;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::components::ui::auto_form::AutoForm;
use crate::components::ui::button::Button;
use crate::components::hooks::use_form::use_form;
```

```rust
#[derive(AutoForm, Validate, Serialize, Deserialize, Clone, Default)]
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
pub fn MyPage() -> impl IntoView {
    let form = use_form::<ContactForm>();

    view! {
        <AutoForm form=form on_submit=|data: ContactForm| {
            // Handle form submission
        }>
            <Button attr:r#type="submit">"Submit"</Button>
        </AutoForm>
    }
}
```



## Type Mapping

AutoForm automatically maps Rust types to appropriate form components:

- `String` → `FormInput` (text)
- `i32`, `u32`, `f64`... → `FormInput` (number)
- `bool` → `Switch`
- `Option<T>` → Same as T |



## Attribute Options

Use `#[autoform(...)]` to customize field rendering:

- `label` - Custom label text (defaults to Title Case): `label = "Full Name"`
- `placeholder` - Input placeholder text: `placeholder = "John Doe"`
- `description` - Help text below the field: `description = "Enter your full name"`
- `field_type` - Override input type: `field_type = "textarea"`
- `hidden` - Hide field from form: `hidden = true`

### Field Types

Override the default field type with `field_type`:

- `"text"` - Default text input
- `"textarea"` - Multi-line text area
- `"number"` - Numeric input
- `"email"` - Email input with validation
- `"password"` - Password input (masked)
- `"checkbox"` - Checkbox toggle
- `"switch"` - Switch toggle



## Validation

AutoForm integrates with the [validator](https://crates.io/crates/validator) crate for field validation. Errors are displayed automatically using on-blur validation - errors only appear after the user leaves a field, providing a smooth user experience.

```rust
#[derive(AutoForm, Validate, Serialize, Deserialize, Clone, Default)]
struct SignupForm {
    #[validate(length(min = 2, message = "Name is required"))]
    name: String,

    #[validate(email(message = "Invalid email address"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[autoform(field_type = "password")]
    password: String,
}
```



## Comparison

AutoForm reduces boilerplate significantly compared to manual form building:

### Before (Manual Form)

```rust
view! {
    <FormProvider form=form>
        <Form class="max-w-md">
            <FormSet>
                <FormField field="name">
                    <FormLabel>"Full Name"</FormLabel>
                    <FormInput attr:placeholder="John Doe" />
                    <FormError />
                </FormField>
                <FormField field="email">
                    <FormLabel>"Email"</FormLabel>
                    <FormInput attr:placeholder="you@example.com" />
                    <FormError />
                </FormField>
                // ... more fields
            </FormSet>
        </Form>
    </FormProvider>
}
```

### After (AutoForm)

```rust
view! {
    <AutoForm form=form>
        <Button attr:r#type="submit">"Submit"</Button>
    </AutoForm>
}
```


## See Also

- [Form](/docs/components/form)
- [Input](/docs/components/input)
- [Select](/docs/components/select)
