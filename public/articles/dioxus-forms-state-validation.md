+++
title = "Build Dioxus Forms with Rust State and Validation"
description = "Create predictable Dioxus forms with signals, typed validation, accessible errors, and clear submit states."
category = "Interaction"
publish_date = "2026-09-18"
last_updated = "2026-09-18"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Forms with Rust state"
keywords = ["Dioxus", "Rust", "forms", "validation", "signals"]
+++

Forms expose UI architecture quickly. Input state, validation, submit state, and server feedback all meet in one place. Dioxus signals let you keep each piece explicit while the compiler protects your data model.

## Model form state first

Use a small Rust struct for values. Keep validation output separate from values so an untouched field does not look invalid on first render.

```rust
#[derive(Clone, Default, PartialEq)]
struct SignupForm {
    email: String,
    project: String,
}

#[derive(Clone, Default, PartialEq)]
struct FormErrors {
    email: Option<String>,
    project: Option<String>,
}
```

This is easier to reason about than a map of loosely typed strings. It also gives you one place to add a field without changing every event handler signature.

## Validate at the boundary

Validation belongs at submit time and, where useful, after a field has been touched. Keep validation pure: input in, errors out. Pure functions are simple to test and safe to reuse on server and client.

```rust
fn validate(form: &SignupForm) -> FormErrors {
    FormErrors {
        email: (!form.email.contains('@')).then(|| "Enter a valid email.".to_string()),
        project: (form.project.trim().len() < 3)
            .then(|| "Project name needs at least 3 characters.".to_string()),
    }
}
```

Do not block typing with aggressive validation. Let users finish a value, then give feedback close to the field. Preserve entered values when validation fails.

## Wire controlled inputs

Controlled inputs make the source of truth obvious. Read the signal value into `value` and update it from `oninput`.

```rust
let mut form = use_signal(SignupForm::default);
let mut errors = use_signal(FormErrors::default);

rsx! {
    input {
        value: form().email,
        aria_invalid: errors().email.is_some().to_string(),
        oninput: move |event| form.write().email = event.value(),
    }
    if let Some(message) = errors().email {
        p { class: "mt-1 text-sm text-destructive", role: "alert", "{message}" }
    }
}
```

For larger forms, split signals by concern or use a form hook. The rule stays the same: one source of truth, one visible error path, no hidden mutation.

## Make submit state visible

A submit button should communicate `Idle`, `Submitting`, `Success`, and `Error`. Disable duplicate submissions while the request is running, but do not erase the form while the user needs to fix an error.

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SubmitState {
    #[default]
    Idle,
    Submitting,
    Success,
    Error,
}
```

Render state near the action. A small status line with `role="status"` helps screen readers and makes async behavior clear for everyone.

Good forms feel calm because every transition has a visible owner. Rust gives the state shape; Dioxus keeps the rendered result synchronized.
