+++
title = "Dioxus Forms: State, Validation, and Accessible Errors"
description = "Build Dioxus forms with signals, a typed state struct, pure validation, accessible error messages, and clear submit states. Full Rust examples included."
category = "Interaction"
publish_date = "2026-09-18"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Forms with Rust state"
keywords = ["Dioxus forms", "Dioxus validation", "Rust form state", "Dioxus signals form", "accessible form errors", "Dioxus onsubmit", "Dioxus input"]
+++

A good Dioxus form keeps values in one typed struct, validates with a pure function, shows errors next to the field that caused them, and makes the submit state visible. Forms are where UI architecture gets tested, because input state, validation, async submission, and server feedback all meet in one screen.

**TL;DR:** Model the form as a Rust struct, not loose strings. Keep errors separate from values, and only show an error after a field is touched or the form is submitted. Validate with a pure `fn(&Form) -> Errors` so you can test it and reuse it on the server. Wire inputs as controlled components with signals, announce errors with `role="alert"`, and model submit as an enum (`Idle`, `Submitting`, `Success`, `Failed`).

- **State:** one struct for values, one for errors, one for "touched" flags.
- **Validation:** pure functions, run on blur and on submit, never while the user is mid-word.
- **Accessibility:** labels, `aria-invalid`, `aria-describedby`, and an alert region for errors.
- **Submit:** prevent the browser default, disable duplicate submits, and keep values on failure.
- **Reuse:** the same validation function can run in a server function.

---

## Who is this guide for?

This guide is for Rust developers building their first real form in Dioxus: signup, settings, search filters, or a checkout step. You should know signals and `rsx!`; if not, start with [your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust). Jump to the validation section if your errors flash too early, to accessibility if you need screen reader support, and to the submit section if you are wiring a server request.

---

## How should you model form state in Dioxus?

**Use a small typed struct for values, and keep errors in a separate struct.** Keeping them apart means an untouched field cannot look invalid on first render, and adding a field changes one struct instead of every event handler.

```rust
#[derive(Clone, Default, PartialEq)]
struct SignupForm {
    email: String,
    project: String,
    accepted_terms: bool,
}

#[derive(Clone, Default, PartialEq)]
struct FormErrors {
    email: Option<String>,
    project: Option<String>,
    accepted_terms: Option<String>,
}

#[derive(Clone, Default, PartialEq)]
struct Touched {
    email: bool,
    project: bool,
}
```

This is easier to reason about than a `HashMap<String, String>` because the compiler checks every field name. A misspelled field name is a build error, not a form that silently never validates.

![Form flow diagram: an input updates a signal, validation runs on the value, and the result either shows a field error or allows submit](/articles/dioxus-forms-state-validation-asset-1.svg)

*Every keystroke updates one signal. Validation is a pure step between the value and the UI, and its result decides between a field error and a submit.*

| Approach | Compile-time safety | Easy to test | Best for |
|---|---|---|---|
| Struct per form (this guide) | Yes | Yes | Most forms |
| `HashMap<String, String>` | No | Partly | Dynamic, schema-driven forms |
| One signal per field | Yes | Harder | Tiny forms with two fields |
| `onsubmit` form values only | No | No | Uncontrolled, submit-only forms |

> **Bottom line:** Start with a struct. You can always split it later, but you cannot cheaply recover type safety you never had.

---

## How do you wire controlled inputs?

**Read the signal into `value` and write it back from `oninput`.** A controlled input makes the signal the single source of truth, so the screen can never disagree with your data.

```rust
use dioxus::prelude::*;

#[component]
fn SignupFormView() -> Element {
    let mut form = use_signal(SignupForm::default);
    let mut touched = use_signal(Touched::default);
    let errors = use_memo(move || validate(&form()));

    rsx! {
        div { class: "space-y-1.5",
            label { r#for: "email", class: "text-sm font-medium", "Email" }
            input {
                id: "email",
                r#type: "email",
                class: "w-full rounded-md border border-input bg-background px-3 py-2",
                value: form().email,
                aria_invalid: (touched().email && errors().email.is_some()).to_string(),
                aria_describedby: "email-error",
                oninput: move |event| form.write().email = event.value(),
                onblur: move |_| touched.write().email = true,
            }
            if touched().email {
                if let Some(message) = errors().email {
                    p { id: "email-error", role: "alert", class: "text-sm text-destructive", "{message}" }
                }
            }
        }
    }
}
```

Three things are happening. `use_memo` derives the errors from the form, so they are always in sync and never stored twice. The error only renders once the field is touched (`onblur`), which prevents the red message from appearing the instant the user starts typing. And the input carries `aria-invalid` and `aria-describedby`, which connect it to its error for assistive technology.

Rust/UI ships ready-made [Input](/docs/components/input), [Label](/docs/components/label), [Textarea](/docs/components/textarea), and [Field](/docs/components/field) components that already apply these tokens and attributes, so you can focus on the state.

---

## When should validation run?

**Validate on blur and on submit, and let the user finish typing.** Showing "Enter a valid email" after the first character is hostile, because the value is not wrong yet, it is unfinished.

Validation itself should be a pure function: values in, errors out. That makes it trivial to test and safe to share.

```rust
fn validate(form: &SignupForm) -> FormErrors {
    FormErrors {
        email: validate_email(&form.email),
        project: (form.project.trim().chars().count() < 3)
            .then(|| "Project name needs at least 3 characters.".to_string()),
        accepted_terms: (!form.accepted_terms)
            .then(|| "You must accept the terms to continue.".to_string()),
    }
}

fn validate_email(email: &str) -> Option<String> {
    let email = email.trim();
    if email.is_empty() {
        return Some("Email is required.".to_string());
    }
    // Deliberately loose: the server confirms by sending a message.
    if !email.contains('@') || email.starts_with('@') || email.ends_with('@') {
        return Some("Enter a valid email address.".to_string());
    }
    None
}
```

Two design rules sit behind this code. First, keep client validation loose and let the server be strict. A regex that tries to accept every legal address will reject real ones, and the only reliable check is sending a confirmation email. Second, count characters with `.chars().count()`, not `.len()`, because `len()` returns bytes and a name like "Zoë" would count as four.

| Timing | Good for | Risk |
|---|---|---|
| On every keystroke | Password strength meters, character counters | Annoying errors on unfinished input |
| On blur | Most field-level errors | Feedback only after leaving the field |
| On submit | Cross-field rules, final gate | Late feedback for long forms |
| Debounced async | Username availability | Needs stale-response handling, see [async data and loading states](/articles/async-data-loading-states-dioxus) |

For the `validator` crate, you can derive rules on the struct and run the same check on the server. If you do, keep the human messages in your UI layer so the wording stays consistent.

---

## How do you make form errors accessible?

**Associate every error with its input, announce it, and never rely on color alone.** About one in twelve men has a color vision deficiency, and a red border does nothing for a screen reader user.

- Give every input a visible `label` bound with `for` and `id`. A placeholder is not a label, because it vanishes as soon as the user types.
- Set `aria-invalid="true"` on a field that has an error.
- Point `aria-describedby` at the error element so the message is read with the field.
- Render the error in an element with `role="alert"` so it is announced when it appears.
- On a failed submit, move focus to the first invalid field so keyboard users land on the problem.
- Add an icon or text prefix to errors in addition to the red color.

```rust
let first_error_id = move || {
    let e = errors();
    if e.email.is_some() { Some("email") }
    else if e.project.is_some() { Some("project") }
    else { None }
};
```

Then focus that element after a failed submit (through `onmounted` and `set_focus`, or a tiny platform hook). An error summary at the top of a long form, linking to each invalid field, is a strong pattern for forms with more than five inputs.

---

## How do you handle submit and server feedback?

**Prevent the browser default, validate, then run the request while exposing a clear state.** Model submission as an enum so impossible combinations such as "submitting and succeeded" cannot exist.

```rust
#[derive(Clone, PartialEq, Default)]
enum SubmitState {
    #[default]
    Idle,
    Submitting,
    Success,
    Failed(String),
}

let mut state = use_signal(SubmitState::default);

let on_submit = move |event: FormEvent| {
    event.prevent_default(); // stop the browser from reloading the page

    // Reveal every error, then stop if the form is invalid.
    touched.set(Touched { email: true, project: true });
    let current = form();
    if validate(&current) != FormErrors::default() {
        return;
    }

    spawn(async move {
        state.set(SubmitState::Submitting);
        match create_account(current).await {
            Ok(()) => state.set(SubmitState::Success),
            Err(error) => state.set(SubmitState::Failed(error.to_string())),
        }
    });
};

rsx! {
    form { onsubmit: on_submit, class: "space-y-4",
        // fields...
        button {
            r#type: "submit",
            class: "rounded-md bg-primary px-4 py-2 text-primary-foreground",
            disabled: state() == SubmitState::Submitting,
            if state() == SubmitState::Submitting { "Creating account..." } else { "Create account" }
        }
        match state() {
            SubmitState::Failed(message) => rsx! { p { role: "alert", class: "text-sm text-destructive", "{message}" } },
            SubmitState::Success => rsx! { p { role: "status", "Account created." } },
            _ => rsx! {},
        }
    }
}
```

Four details make this feel calm. The button is disabled while the request runs, so a double click cannot create two accounts. The form values are never cleared on failure, so the user fixes the problem instead of retyping. The status line uses `role="status"` for success and `role="alert"` for failure. And the submit handler marks every field as touched, so a user who never blurred an input still sees what is wrong.

Server errors need two homes. A field error from the server ("That email is already registered") belongs next to the email input, so map it into `FormErrors`. A general error ("We could not reach the server") belongs near the button. Mixing them into one banner forces users to guess which field to change.

---

## How do you share validation between client and server?

**Put the validation function in a shared crate or module, and call it on both sides.** The client check gives instant feedback. The server check is the one you trust, because anyone can bypass your UI.

```rust
#[server]
async fn create_account(form: SignupForm) -> Result<(), ServerFnError> {
    let errors = validate(&form); // same function the UI uses
    if errors != FormErrors::default() {
        return Err(ServerFnError::new("Invalid form"));
    }
    // persist, send the confirmation email, and so on
    Ok(())
}
```

With Dioxus fullstack, the form struct derives `Serialize` and `Deserialize`, crosses the boundary as typed data, and returns a typed result. That removes a whole category of bugs where the client and server disagree about field names. Typed boundaries also help when the rules change. Raising the minimum project name length is a one-line edit in `validate`, and both the instant feedback and the server enforcement update together, with no drift to chase across files. Never treat client validation as a security control. It improves user experience; the server enforces the rules.

---

## What does a complete form screen need?

**Every field needs six things: a label, a type, a value binding, an error path, a touched flag, and a keyboard story.** A short checklist keeps forms consistent across a product:

1. The right input `type` (`email`, `tel`, `number`, `password`) so mobile keyboards adapt.
2. `autocomplete` hints (`email`, `current-password`, `one-time-code`) so password managers work.
3. A visible label and helper text if the format is not obvious.
4. Inline error text tied to the field, not only a toast.
5. Enter submits the form, and Tab order follows the visual order.
6. A disabled and a loading state for the button.

On small screens, keep labels above inputs, use full-width fields, and make sure the keyboard does not hide the submit button, as described in [responsive Rust UI with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind). For checkboxes, switches, selects, and radio groups, the same pattern applies; Rust/UI has [Checkbox](/docs/components/checkbox), [Select](/docs/components/select), and a higher level [Form](/docs/components/form) component to start from. If you build your own field components, design their props with the guidance in [reusable Dioxus components with typed props](/articles/reusable-dioxus-components-typed-props).

---

## How do dynamic fields and lists work?

**Model repeatable fields as a `Vec`, give each row a stable id, and key the loop.** A form that lets people add teammates or line items is a list of small forms.

```rust
#[derive(Clone, PartialEq)]
struct Member {
    id: u32,
    email: String,
}

let mut members = use_signal(|| vec![Member { id: 0, email: String::new() }]);
let mut next_id = use_signal(|| 1u32);

rsx! {
    for member in members() {
        div { key: "{member.id}", class: "flex gap-2",
            input {
                value: "{member.email}",
                oninput: move |event| {
                    let id = member.id;
                    if let Some(row) = members.write().iter_mut().find(|row| row.id == id) {
                        row.email = event.value();
                    }
                },
            }
            button {
                r#type: "button",
                onclick: move |_| members.write().retain(|row| row.id != member.id),
                "Remove"
            }
        }
    }
    button {
        r#type: "button",
        onclick: move |_| {
            let id = next_id();
            next_id.set(id + 1);
            members.write().push(Member { id, email: String::new() });
        },
        "Add member"
    }
}
```

The `key` matters: without a stable id, removing the first row can leave the second row showing the first row's text. Use an incrementing id, never the index. Validate each row independently and show its error inside the row. Buttons that are not submit buttons need `type: "button"`, otherwise they submit the form.

---

## How do you keep long forms manageable?

**Split them into steps or sections, and save progress.** A form with fifteen fields feels heavy even when each field is simple. Group related fields under headings, or break the flow into steps with a visible progress indicator. Keep one top-level struct and let each step edit its slice, so validation of the whole form still runs once at the end. Save drafts to storage as the user types, so an accidental refresh or a dropped connection does not destroy their work. Warn before navigating away from unsaved changes, and always show which step failed validation when the final submit is rejected.

---

## Should you use a form library or build it yourself?

**Start by hand, and adopt a helper once the same code repeats three times.** The struct, memo, and touched pattern in this guide is short and fully visible, which makes it easy to debug. A larger product with dozens of forms benefits from a shared field component and a small helper, so labels, errors, and ARIA wiring live in one place.

---

## What are the common form mistakes?

| Mistake | Symptom | Fix |
|---|---|---|
| Validating on every keystroke | Errors flash while typing | Validate on blur and submit |
| Placeholder instead of label | Users forget what a field is | Always render a visible label |
| Clearing the form on error | Users retype everything | Keep values, show errors |
| Error only in color | Invisible to some users | Text, icon, and `aria-describedby` |
| No disabled state while submitting | Duplicate requests | Disable the button, show progress |
| Client-only validation | Bad data reaches the database | Re-validate on the server |
| Strict email regex | Real addresses rejected | Loose client check, confirm by email |
| Errors stored in a second signal | Values and errors drift apart | Derive errors with `use_memo` |

---

## Frequently Asked Questions

### How do I handle form input in Dioxus?

Create a signal for the value, bind it to `value`, and update it from `oninput` with `event.value()`. This makes the input controlled, so the signal is always the source of truth.

### How do I stop the page from reloading when a Dioxus form submits?

Call `event.prevent_default()` at the top of your `onsubmit` handler. Without it, the browser performs its default form submission and reloads the page.

### Should I validate on blur, on change, or on submit?

Use blur for field-level errors and submit for the final gate. Validating on every change shows errors for values the user has not finished typing, which feels hostile.

### How do I show errors for only the fields the user touched?

Keep a separate `Touched` struct set from `onblur`, and render an error only when the field is touched or the form was submitted. On submit, mark every field as touched.

### How do I validate a Dioxus form on the server too?

Put the validation function in code both the UI and a `#[server]` function can call. The client check is for feedback, and the server check is the one you trust.

### How do I make Dioxus form errors accessible?

Use a visible label, set `aria-invalid`, link the error with `aria-describedby`, and render it with `role="alert"`. Move focus to the first invalid field after a failed submit.

### What is the best way to track submit state?

Use an enum such as `Idle`, `Submitting`, `Success`, and `Failed(String)`. It prevents impossible combinations and makes the button text, disabled state, and status message a single `match`.

### Does Rust/UI have form components for Dioxus?

Yes. Rust/UI includes [Input](/docs/components/input), [Label](/docs/components/label), [Textarea](/docs/components/textarea), [Checkbox](/docs/components/checkbox), [Select](/docs/components/select), and [Field](/docs/components/field) and [Form](/docs/components/form) wrappers you copy into your project and own.

---

## Sources

- [Dioxus documentation: forms and events](https://dioxuslabs.com/learn/0.7/)
- [MDN: HTML forms](https://developer.mozilla.org/en-US/docs/Learn_web_development/Extensions/Forms)
- [WAI-ARIA Authoring Practices: forms](https://www.w3.org/WAI/tutorials/forms/)
- [MDN: aria-invalid](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-invalid)
