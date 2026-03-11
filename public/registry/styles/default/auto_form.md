---
title: "Auto Form"
name: "auto_form"
cargo_dependencies: []
registry_dependencies: ["checkbox", "form", "label", "switch"]
type: "components:ui"
path: "ui/auto_form.rs"
---

# Auto Form

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add auto_form
```

## Component Code

```rust
//! AutoForm component for automatic form generation.
//!
//! Use the `#[derive(AutoForm)]` macro to automatically generate form UI from Rust structs.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tw_merge::tw_merge;
use validator::Validate;

use crate::components::hooks::use_form::{AutoFormFields, FieldContext, Form, FormContext};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::form::{Form as FormElement, FormProvider, FormSet};
use crate::components::ui::label::Label;
use crate::components::ui::switch::SwitchLabel;

/// AutoForm component that automatically renders form fields based on a struct.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(AutoForm, Validate, Serialize, Deserialize, Clone, Default)]
/// struct ContactForm {
///     #[autoform(label = "Full Name", placeholder = "John Doe")]
///     name: String,
///
///     #[autoform(field_type = "textarea")]
///     bio: Option<String>,
///
///     subscribe: bool,
/// }
///
/// #[component]
/// fn MyPage() -> impl IntoView {
///     let form = use_form::<ContactForm>();
///
///     view! {
///         <AutoForm
///             form=form
///             on_submit=|data: ContactForm| {
///                 // Handle submission
///             }
///         >
///             <Button type="submit">"Submit"</Button>
///         </AutoForm>
///     }
/// }
/// ```
#[component]
pub fn AutoForm<T>(
    /// The form instance from `use_form::<T>()`
    form: Form<T>,
    /// Optional callback when form is submitted with valid data
    #[prop(optional, into)]
    on_submit: Option<Callback<T>>,
    /// Optional CSS class for the form element
    #[prop(into, optional)]
    class: String,
    /// Optional children (typically a submit button)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView
where
    T: AutoFormFields + Validate + Clone + Default + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    let merged_class = tw_merge!("max-w-md", class);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(callback) = on_submit
            && let Ok(data) = form.validate_and_get()
        {
            callback.run(data);
        }
    };

    view! {
        <FormProvider form=form>
            <FormElement class=merged_class on:submit=handle_submit>
                <FormSet>
                    <AutoFormFieldsWrapper<T> form=form />
                </FormSet>
                {children.map(|c| view! { <div class="mt-6">{c()}</div> })}
            </FormElement>
        </FormProvider>
    }
}

/// Inner wrapper component to ensure FormContext is available when rendering fields
#[component]
fn AutoFormFieldsWrapper<T>(form: Form<T>) -> impl IntoView
where
    T: AutoFormFields + 'static,
{
    T::render_fields(form)
}

/// Form textarea component that integrates with the form context.
#[component]
pub fn FormTextarea(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let field_name = expect_context::<FieldContext>().name;
    let form_ctx = expect_context::<FormContext>();

    let merged_class = tw_merge!(
        "border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        class
    );

    view! {
        <textarea
            data-name="FormTextarea"
            id=field_name.clone()
            class=merged_class
            placeholder=placeholder
            aria-invalid={
                let field_name = field_name.clone();
                move || {
                    let is_touched = form_ctx.touched_signal.get().contains(&field_name);
                    let has_error = form_ctx.errors_signal.get().get(&field_name).is_some_and(|e| e.is_some());
                    (is_touched && has_error).then_some("true")
                }
            }
            prop:value={
                let field_name = field_name.clone();
                move || form_ctx.values_signal.get().get(&field_name).cloned().unwrap_or_default()
            }
            on:input={
                let field_name = field_name.clone();
                move |ev| {
                    form_ctx.set_value.with_value(|f| f(&field_name, event_target_value(&ev)));
                }
            }
            on:blur=move |_| {
                form_ctx.touch_field.with_value(|f| f(&field_name));
            }
        />
    }
}

/// Form checkbox component that integrates with the form context.
#[component]
pub fn FormCheckbox(#[prop(into, optional)] class: String, #[prop(into)] label: String) -> impl IntoView {
    let field_name = expect_context::<FieldContext>().name;
    let form_ctx = expect_context::<FormContext>();

    // Get the current checked state from form values
    let checked = Signal::derive({
        let field_name = field_name.clone();
        move || form_ctx.values_signal.get().get(&field_name).map(|v| v == "true").unwrap_or(false)
    });

    let on_change = {
        let field_name = field_name.clone();
        Callback::new(move |new_value: bool| {
            form_ctx.set_value.with_value(|f| f(&field_name, new_value.to_string()));
        })
    };

    let merged_class = tw_merge!("flex items-center gap-2", class);

    view! {
        <div class=merged_class>
            <Checkbox checked=checked on_checked_change=on_change />
            <Label html_for=field_name class="text-sm font-medium cursor-pointer">
                {label}
            </Label>
        </div>
    }
}

/// Form switch component that integrates with the form context.
#[component]
pub fn FormSwitch(#[prop(into, optional)] class: String, #[prop(into)] label: String) -> impl IntoView {
    let field_name = expect_context::<FieldContext>().name;
    let form_ctx = expect_context::<FormContext>();

    // Get the current checked state from form values
    let checked = {
        let field_name = field_name.clone();
        move || form_ctx.values_signal.get().get(&field_name).map(|v| v == "true").unwrap_or(false)
    };

    let handle_change = {
        let field_name = field_name.clone();
        move |ev: leptos::ev::Event| {
            let target = event_target::<web_sys::HtmlInputElement>(&ev);
            let new_value = target.checked();
            form_ctx.set_value.with_value(|f| f(&field_name, new_value.to_string()));
        }
    };

    let merged_class = tw_merge!("flex gap-2", class);

    view! {
        <div class=merged_class>
            <label class="inline-flex relative items-center cursor-pointer" tabindex="0">
                <input
                    type="checkbox"
                    value=""
                    class="hidden peer"
                    id=field_name
                    checked=checked
                    on:change=handle_change
                />
                <div
                    data-name="Switch"
                    class="w-11 h-6 bg-gray-200 rounded-full peer-focus:outline-hidden peer-focus:ring-ring/50 peer-focus:ring-[3px] peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:size-5 after:transition-all peer-checked:bg-primary"
                />
            </label>
            <SwitchLabel>{label}</SwitchLabel>
        </div>
    }
}
```
