# autoform

Derive macro for automatic form generation in Leptos applications.

## Usage

```rust
use autoform::AutoForm;
use validator::Validate;
use serde::{Deserialize, Serialize};

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
```

## Attribute Options

- `label` - Custom label text (defaults to field name in Title Case)
- `placeholder` - Input placeholder text
- `description` - Description text shown below the field
- `field_type` - Override field type: "text", "textarea", "switch", "checkbox", "number", "email", "password"
- `hidden` - Hide this field from the form

## License

MIT
