---
title: "Field"
name: "field"
cargo_dependencies: []
registry_dependencies: ["label", "separator"]
type: "components:ui"
path: "ui/field.rs"
description: "Rust/UI components for composing accessible form fields with labels, descriptions, and error messages."
tags: ["input"]
---

# Field

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add field
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::{clx, variants};
use tw_merge::tw_merge;

use crate::components::ui::label::Label;
use crate::components::ui::separator::Separator;

mod components {
    use super::*;

    clx! {FieldSet, fieldset, "flex flex-col gap-6 has-[>[data-name=CheckboxGroup]]:gap-3 has-[>[data-name=RadioGroup]]:gap-3"}
    clx! {FieldGroup, div, "group/field-group @container/field-group flex flex-col gap-7 w-full has-[>[data-name=CheckboxGroup]]:gap-3 [&>[data-name=FieldGroup]]:gap-4"}
    clx! {FieldContent, div, "group/field-content flex flex-1 flex-col gap-1.5 leading-snug"}
    clx! {FieldTitle, div, "flex items-center gap-2 text-sm leading-snug font-medium w-fit group-data-[disabled=true]/field:opacity-50"}
    clx! {FieldDescription, p, "text-muted-foreground text-sm leading-normal font-normal group-has-[[data-orientation=horizontal]]/field:text-balance last:mt-0 nth-last-2:-mt-1 [[data-variant=legend]+&]:-mt-1.5 [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FieldLegend(
    #[prop(into, optional)] class: String,
    #[prop(default = FieldLegendVariant::Legend)] variant: FieldLegendVariant,
    children: Children,
) -> impl IntoView {
    let variant_attr = match variant {
        FieldLegendVariant::Legend => "legend",
        FieldLegendVariant::Label => "label",
    };

    let merged_class =
        tw_merge!("mb-3 font-medium data-[variant=legend]:text-base data-[variant=label]:text-sm", class);

    view! {
        <legend attr:data-slot="field-legend" attr:data-variant=variant_attr class=merged_class>
            {children()}
        </legend>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FieldLegendVariant {
    Legend,
    Label,
}

variants! {
    Field {
        base: "group/field flex gap-3 w-full data-[invalid=true]:text-destructive",
        variants: {
            variant: {
                Vertical: "flex-col [&>*]:w-full [&>.hidden]:w-auto", // sr-only -> hidden
                Horizontal: "flex-row items-center [&>[data-slot=field-label]]:flex-auto has-[>[data-name=FieldContent]]:items-start has-[>[data-name=FieldContent]]:[&>[role=checkbox],[role=radio]]:mt-px",
                Responsive: "flex-col [&>*]:w-full [&>.hidden]:w-auto @md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto @md/field-group:[&>[data-slot=field-label]]:flex-auto @md/field-group:has-[>[data-name=FieldContent]]:items-start @md/field-group:has-[>[data-name=FieldContent]]:[&>[role=checkbox],[role=radio]]:mt-px", // sr-only -> hidden
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
pub fn FieldLabel(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] html_for: String,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "group/field-label peer/field-label flex gap-2 leading-snug w-fit group-data-[disabled=true]/field:opacity-50 has-[>[data-name=Field]]:w-full has-[>[data-name=Field]]:flex-col has-[>[data-name=Field]]:rounded-md has-[>[data-name=Field]]:border [&>*]:data-[name=Field]:p-4 has-data-[state=checked]:bg-primary/5 has-data-[state=checked]:border-primary dark:has-data-[state=checked]:bg-primary/10 has-[:checked]:bg-primary/5 has-[:checked]:border-primary dark:has-[:checked]:bg-primary/10",
        class
    );

    view! {
        <Label attr:data-slot="field-label" class=merged_class html_for=html_for>
            {children()}
        </Label>
    }
}

#[component]
pub fn FieldSeparator(
    #[prop(into, optional)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_content = children.is_some();

    let merged_class = tw_merge!("relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2", class);

    view! {
        <div attr:data-slot="field-separator" attr:data-content=has_content.to_string() class=merged_class>
            <Separator class="absolute inset-0 top-1/2" />
            {children
                .map(|children| {
                    view! {
                        <span
                            class="block relative px-2 mx-auto bg-background text-muted-foreground w-fit"
                            attr:data-slot="field-separator-content"
                        >
                            {children()}
                        </span>
                    }
                })}
        </div>
    }
}

#[component]
pub fn FieldError(
    #[prop(into, optional)] class: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] errors: Option<Vec<String>>,
) -> impl IntoView {
    // If children is provided, render it directly
    if let Some(children) = children {
        return view! {
            <div
                role="alert"
                attr:data-slot="field-error"
                class=tw_merge!("text-destructive text-sm font-normal", &class)
            >
                {children()}
            </div>
        }
        .into_any();
    }

    // Otherwise, handle errors reactively
    view! {
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
                                    attr:data-slot="field-error"
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
                                    attr:data-slot="field-error"
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
    .into_any()
}
```
