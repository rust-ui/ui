---
title: "Form"
name: "form"
cargo_dependencies: []
registry_dependencies: ["input", "label", "separator"]
type: "components:ui"
path: "ui/form.rs"
description: "Rust/UI components for building accessible forms with labels, descriptions, and error messages."
tags: ["input"]
---

# Form

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add form
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::{clx, variants};
use serde::{Deserialize, Serialize};
use strum::Display;
use tw_merge::tw_merge;
use validator::Validate;

use crate::components::hooks::use_form::{FieldContext, Form as FormHook, FormContext};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::separator::Separator;

mod components {
    use super::*;

    clx! {FormSet, fieldset, "flex flex-col gap-6 has-[>[data-name=CheckboxGroup]]:gap-3 has-[>[data-name=RadioGroup]]:gap-3"}
    clx! {FormGroup, div, "group/field-group @container/field-group flex flex-col gap-7 w-full data-[name=CheckboxGroup]:gap-3 [&>[data-name=FormGroup]]:gap-4"}
    clx! {FormContent, div, "group/field-content flex flex-1 flex-col gap-1.5 leading-snug"}
    clx! {FormTitle, div, "flex items-center gap-2 text-sm leading-snug font-medium w-fit group-data-[disabled=true]/field:opacity-50"}
    clx! {FormDescription, p, "text-muted-foreground text-sm leading-normal font-normal group-has-[[data-orientation=horizontal]]/field:text-balance last:mt-0 nth-last-2:-mt-1 [[data-variant=legend]+&]:-mt-1.5 [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FormProvider<T>(form: FormHook<T>, children: Children) -> impl IntoView
where
    T: Validate + Clone + Default + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    use crate::components::hooks::use_form::{FormContext, SetValueFn, TouchFieldFn};

    let set_value_fn: SetValueFn = Box::new(move |field: &str, value: String| {
        form.set_value(field, value);
    });

    let touch_field_fn: TouchFieldFn = Box::new(move |field: &str| {
        form.touch_field(field);
    });

    let ctx = FormContext {
        values_signal: form.values_signal,
        errors_signal: form.errors_signal,
        touched_signal: form.touched_signal,
        set_value: StoredValue::new(set_value_fn),
        touch_field: StoredValue::new(touch_field_fn),
    };

    provide_context(ctx);
    children()
}

#[component]
pub fn Form(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let _ctx = expect_context::<FormContext>();

    let merged_class = tw_merge!("w-full", class);

    view! { <form class=merged_class>{children()}</form> }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Display)]
pub enum FormLegendVariant {
    Legend,
    Label,
}

#[component]
pub fn FormLegend(
    #[prop(into, optional)] class: String,
    #[prop(default = FormLegendVariant::Legend)] variant: FormLegendVariant,
    children: Children,
) -> impl IntoView {
    let merged_class =
        tw_merge!("mb-3 font-medium data-[variant=Legend]:text-base data-[variant=Label]:text-sm", class);

    view! {
        <legend data-name="FormLegend" attr:data-variant=variant.to_string() class=merged_class>
            {children()}
        </legend>
    }
}

variants! {
    FormFieldWrapper {
        base: "group/field flex gap-3 w-full data-[invalid=true]:text-destructive",
        variants: {
            variant: {
                Vertical: "flex-col [&>*]:w-full [&>.hidden]:w-auto", // sr-only -> hidden
                Horizontal: "flex-row items-center [&>[data-name=FieldLabel]]:flex-auto has-[>[data-name=FormContent]]:items-start has-[>[data-name=FormContent]]:[&>[role=checkbox],[role=radio]]:mt-px",
                Responsive: "flex-col [&>*]:w-full [&>.hidden]:w-auto @md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto @md/field-group:[&>[data-name=FieldLabel]]:flex-auto @md/field-group:has-[>[data-name=FormContent]]:items-start @md/field-group:has-[>[data-name=FormContent]]:[&>[role=checkbox],[role=radio]]:mt-px", // sr-only -> hidden
            },
            size: {
                Default: "",
            }
        },
        component: {
            element: div
        }
    }
}

#[component]
pub fn FormLabel(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] html_for: String,
    children: Children,
) -> impl IntoView {
    let field_name = if html_for.is_empty() {
        use_context::<FieldContext>().map(|ctx| ctx.name).unwrap_or_default()
    } else {
        html_for
    };

    let merged_class = tw_merge!(
        "group/form-label peer/form-label flex gap-2 leading-snug w-fit group-data-[disabled=true]/field:opacity-50 has-[>[data-name=Field]]:w-full has-[>[data-name=Field]]:flex-col has-[>[data-name=Field]]:rounded-md has-[>[data-name=Field]]:border [&>*]:data-[name=Field]:p-4 has-data-[state=checked]:bg-primary/5 has-data-[state=checked]:border-primary dark:has-data-[state=checked]:bg-primary/10",
        class
    );

    view! {
        <Label attr:data-name="FormLabel" class=merged_class html_for=field_name>
            {children()}
        </Label>
    }
}

#[component]
pub fn FormSeparator(
    #[prop(into, optional)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_content = children.is_some();

    let merged_class = tw_merge!("relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2", class);

    view! {
        <div attr:data-name="FormSeparator" attr:data-content=has_content.to_string() class=merged_class>
            <Separator class="absolute inset-0 top-1/2" />
            {children
                .map(|children| {
                    view! {
                        <span
                            class="block relative px-2 mx-auto bg-background text-muted-foreground w-fit"
                            attr:data-name="FormSeparatorContent"
                        >
                            {children()}
                        </span>
                    }
                })}
        </div>
    }
}

#[component]
pub fn FormError(
    #[prop(into, optional)] class: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] errors: Option<Vec<String>>,
) -> impl IntoView {
    // If children is provided, render it directly
    if let Some(children) = children {
        return view! {
            <div
                role="alert"
                attr:data-name="FormError"
                class=tw_merge!("text-destructive text-sm font-normal", &class)
            >
                {children()}
            </div>
        }
        .into_any();
    }

    // If errors provided, handle them reactively
    if errors.is_some() {
        return view! {
            {move || {
                errors
                    .as_ref()
                    .and_then(|errors| {
                        if errors.is_empty() {
                            None
                        } else if errors.len() == 1 {
                            Some(
                                view! {
                                    <div
                                        role="alert"
                                        attr:data-name="FormError"
                                        class=tw_merge!("text-destructive text-sm font-normal", &class)
                                    >
                                        <span>{errors.first().cloned().unwrap_or_default()}</span>
                                    </div>
                                }
                                    .into_any(),
                            )
                        } else {
                            Some(
                                view! {
                                    <div
                                        role="alert"
                                        attr:data-name="FormError"
                                        class=tw_merge!("text-destructive text-sm font-normal", &class)
                                    >
                                        <ul class="flex flex-col gap-1 ml-4 list-disc">
                                            {errors
                                                .iter()
                                                .map(|error| view! { <li>{error.clone()}</li> })
                                                .collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                }
                                    .into_any(),
                            )
                        }
                    })
            }}
        }
        .into_any();
    }

    // Otherwise, try to get error from field context
    // Only show error if field has been touched (blurred)
    let field_ctx = use_context::<FieldContext>();
    let form_ctx = use_context::<FormContext>();

    if let (Some(field_ctx), Some(form_ctx)) = (field_ctx, form_ctx) {
        let field_name = field_ctx.name;
        return view! {
            {move || {
                let is_touched = form_ctx.touched_signal.get().contains(&field_name);
                if !is_touched {
                    return None;
                }
                form_ctx
                    .errors_signal
                    .get()
                    .get(&field_name)
                    .and_then(|e| e.clone())
                    .map(|err| {
                        // Only show error if field is touched

                        view! {
                            <div
                                role="alert"
                                attr:data-name="FormError"
                                class=tw_merge!("text-destructive text-sm font-normal", &class)
                            >
                                <span>{err}</span>
                            </div>
                        }
                    })
            }}
        }
        .into_any();
    }

    // No error to display
    ().into_any()
}

/* ========================================================== */
/*                  ✨ FORM COMPONENTS ✨                     */
/* ========================================================== */

#[component]
pub fn FormField(#[prop(into)] field: String, children: Children) -> impl IntoView {
    provide_context(FieldContext { name: field.clone() });

    let ctx = expect_context::<FormContext>();
    // Only show invalid state if field is touched AND has error
    let has_error = move || {
        let is_touched = ctx.touched_signal.get().contains(&field);
        let has_error = ctx.errors_signal.get().get(&field).is_some_and(|e| e.is_some());
        (is_touched && has_error).then_some("true")
    };

    view! {
        <FormFieldWrapper attr:data-name="FormField" attr:data-invalid=has_error>
            {children()}
        </FormFieldWrapper>
    }
}

#[component]
pub fn FormInput() -> impl IntoView {
    let field_name = expect_context::<FieldContext>().name;
    let form_ctx = expect_context::<FormContext>();

    view! {
        <Input
            attr:id=field_name.clone()
            attr:aria-invalid={
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
```
