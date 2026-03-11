+++
title = "Form"
description = "Rust/UI components for building accessible forms with labels, descriptions, and error messages."
tags = ["input"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticForm />




## Installation

<StaticInstallForm />



## Components

The Form component is composed of several subcomponents:

- **FormProvider**: Provides form context for validation and state management
- **Form**: Main form wrapper component
- **FormSet**: Wraps a group of related fields (fieldset element)
- **FormGroup**: Groups multiple fields together
- **FormField**: Individual field container with validation context
- **FormInput**: Integrated input component with form state binding
- **FormLabel**: Label for the field
- **FormDescription**: Helper text for the field
- **FormError**: Error message display with validation support
- **FormContent**: Content wrapper for field elements
- **FormTitle**: Title text for fields
- **FormLegend**: Legend for fieldsets
- **FormSeparator**: Visual separator between fields



## Usage

You can use the `Form` component in combination with the [Select](/docs/components/select) component.

```rust
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::components::ui::form::{
    Form,
    FormDescription,
    FormError,
    FormField,
    FormGroup,
    FormInput,
    FormLabel,
    FormProvider,
    FormSet,
};
use crate::components::hooks::use_form::use_form;
```

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
struct FormData {
    name: String,
    username: String,
}

#[component]
pub fn MyForm() -> impl IntoView {
    let form = use_form::<FormData>();

    view! {
        <FormProvider form=form>
            <Form class="max-w-md">
                <FormSet>
                    <FormGroup>
                        <FormField field="name">
                            <FormLabel>Full name</FormLabel>
                            <FormInput attr:placeholder="Max Wells" />
                            <FormDescription>This appears on invoices and emails.</FormDescription>
                        </FormField>
                        <FormField field="username">
                            <FormLabel>Username</FormLabel>
                            <FormInput attr:placeholder="maxwells" />
                            <FormError />
                        </FormField>
                    </FormGroup>
                </FormSet>
            </Form>
        </FormProvider>
    }
}
```



## Examples

### Error Display

Form component with validation error messages displayed inline below fields. This example demonstrates how to show field-level errors in Leptos using [Input](/docs/components/input) fields with FormError, automatic error state management, and ARIA attributes for accessibility in Rust applications.

<StaticFormError />

### Dynamic Validation

Real-time form validation with instant feedback as users type in [Input](/docs/components/input) fields. This example showcases live validation in Leptos that checks email format dynamically and displays error messages immediately for improved user experience in Rust applications.

<StaticFormValidation />

### Fieldset with Legend

Form fields organized using semantic fieldset and legend elements for better grouping. This example demonstrates accessible form structure in Leptos using FormSet and FormLegend components to create logical field groups with descriptive labels.

<StaticFormFieldset />

### Form with Select

Form integration with select dropdown components for option-based input. This example shows how to combine FormField with [Select](/docs/components/select) components in Leptos to build accessible forms with proper state binding in Rust applications.

<StaticFormSelect />

### Grouped Fields

Form fields organized into visual groups with separators for improved readability. This example demonstrates how to use FormGroup and FormSeparator in Leptos to create well-structured forms with clear section boundaries in Rust applications.

<StaticFormGroup />





## See Also

- [Input](/docs/components/input)
- [Select](/docs/components/select)
- [Checkbox](/docs/components/checkbox)
- [Switch](/docs/components/switch)
- [Textarea](/docs/components/textarea)
- [Radio Button](/docs/components/radio-button)
- [Label](/docs/components/label)
- [Auto-Form](/docs/components/auto-form)
