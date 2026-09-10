+++
title = "Form"
description = "Rust/UI components for building accessible forms with labels, descriptions, and error messages."
tags = ["input"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticForm />

## Installation

<StaticInstallForm />

## Usage

```rust
use crate::ui::form::{
    Form,
    FormContent,
    FormDescription,
    FormError,
    FormField,
    FormGroup,
    FormInput,
    FormLabel,
    FormProvider,
    FormSet,
};
use crate::hooks::use_form::use_form;
```

```rust
rsx! {
    DemoForm {}
}
```

## Examples

### Basic Form

Typed form composition with labels, inputs, and grouped sections.

<StaticForm />

### Validation

Validation-oriented example showing errors and touched-field behavior.

<StaticFormValidation />

### Group

Grouped form layout for related inputs.

<StaticFormGroup />

### Fieldset

Fieldset-style form grouping with legends and descriptions.

<StaticFormFieldset />

### Error

Inline form error presentation.

<StaticFormError />

### Select

Form usage with select controls.

<StaticFormSelect />

### Auto Form

Auto-generated form fields from a typed schema.

<StaticAutoForm />

## See Also

- [Field](/components/field)
- [Input](/components/input)
