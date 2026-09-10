use std::sync::Arc;

use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::hooks::use_form::{FieldContext, Form as FormHook, FormContext, FormData};
use crate::ui::label::Label;
use crate::ui::separator::Separator;

/* ========================================================== */
/*                     ✨ CLX COMPONENTS ✨                   */
/* ========================================================== */

#[component]
pub fn FormSet(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex flex-col gap-6 has-[>[data-name=CheckboxGroup]]:gap-3 has-[>[data-name=RadioGroup]]:gap-3",
        class.as_deref().unwrap_or("")
    );
    rsx! { fieldset { class: "{merged}", {children} } }
}

#[component]
pub fn FormGroup(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] data_name: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "group/field-group @container/field-group flex flex-col gap-7 w-full data-[name=CheckboxGroup]:gap-3 [&>[data-name=FormGroup]]:gap-4",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": data_name.as_deref().unwrap_or("FormGroup"),
            class: "{merged}",
            {children}
        }
    }
}

#[component]
pub fn FormContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged =
        tw_merge!("group/field-content flex flex-1 flex-col gap-1.5 leading-snug", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FormContent", class: "{merged}", {children} } }
}

#[component]
pub fn FormTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-2 text-sm leading-snug font-medium w-fit group-data-[disabled=true]/field:opacity-50",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "FormTitle", class: "{merged}", {children} } }
}

#[component]
pub fn FormDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "text-muted-foreground text-sm leading-normal font-normal group-has-[[data-orientation=horizontal]]/field:text-balance last:mt-0 nth-last-2:-mt-1 [[data-variant=legend]+&]:-mt-1.5 [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4",
        class.as_deref().unwrap_or("")
    );
    rsx! { p { "data-name": "FormDescription", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FormProvider<T: FormData>(form: FormHook<T>, children: Element) -> Element {
    let set_value_fn: crate::hooks::use_form::SetValueFn = Arc::new({
        move |field: &str, value: String| {
            form.set_value(field, value);
        }
    });

    let touch_field_fn: crate::hooks::use_form::TouchFieldFn = Arc::new({
        move |field: &str| {
            form.touch_field(field);
        }
    });

    let ctx = FormContext {
        values_signal: form.values_signal,
        errors_signal: form.errors_signal,
        touched_signal: form.touched_signal,
        set_value: set_value_fn,
        touch_field: touch_field_fn,
    };

    use_context_provider(|| ctx);
    rsx! { {children} }
}

#[component]
pub fn Form(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let _ctx = consume_context::<FormContext>();
    let merged = tw_merge!("w-full", class.as_deref().unwrap_or(""));
    rsx! { form { class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ FORM LEGEND ✨                      */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum FormLegendVariant {
    #[default]
    Legend,
    Label,
}

impl FormLegendVariant {
    fn as_str(&self) -> &'static str {
        match self {
            FormLegendVariant::Legend => "Legend",
            FormLegendVariant::Label => "Label",
        }
    }
}

#[component]
pub fn FormLegend(
    #[props(into, optional)] class: Option<String>,
    #[props(default = FormLegendVariant::Legend)] variant: FormLegendVariant,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "mb-3 font-medium data-[variant=Legend]:text-base data-[variant=Label]:text-sm",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        legend {
            "data-name": "FormLegend",
            "data-variant": "{variant.as_str()}",
            class: "{merged}",
            {children}
        }
    }
}

/* ========================================================== */
/*                  ✨ FORM FIELD WRAPPER ✨                  */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum FormFieldVariant {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FormFieldVariant {
    fn as_str(&self) -> &'static str {
        match self {
            FormFieldVariant::Vertical => "Vertical",
            FormFieldVariant::Horizontal => "Horizontal",
            FormFieldVariant::Responsive => "Responsive",
        }
    }

    fn class(&self) -> &'static str {
        match self {
            FormFieldVariant::Vertical => "flex-col [&>*]:w-full [&>.hidden]:w-auto",
            FormFieldVariant::Horizontal => {
                "flex-row items-center [&>[data-name=FieldLabel]]:flex-auto has-[>[data-name=FormContent]]:items-start has-[>[data-name=FormContent]]:[&>[role=checkbox],[role=radio]]:mt-px"
            }
            FormFieldVariant::Responsive => {
                "flex-col [&>*]:w-full [&>.hidden]:w-auto @md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto @md/field-group:[&>[data-name=FieldLabel]]:flex-auto @md/field-group:has-[>[data-name=FormContent]]:items-start @md/field-group:has-[>[data-name=FormContent]]:[&>[role=checkbox],[role=radio]]:mt-px"
            }
        }
    }
}

#[component]
pub fn FormFieldWrapper(
    #[props(into, optional)] class: Option<String>,
    #[props(default = FormFieldVariant::Vertical)] variant: FormFieldVariant,
    #[props(into, optional)] data_invalid: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "group/field flex gap-3 w-full data-[invalid=true]:text-destructive",
        variant.class(),
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": "FormField",
            "data-variant": "{variant.as_str()}",
            "data-invalid": data_invalid.as_deref().unwrap_or("false"),
            class: "{merged}",
            {children}
        }
    }
}

/* ========================================================== */
/*                   ✨ FORM LABEL ✨                         */
/* ========================================================== */

#[component]
pub fn FormLabel(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] html_for: Option<String>,
    children: Element,
) -> Element {
    let field_name = if let Some(f) = html_for { f } else { consume_context::<FieldContext>().name };

    let merged = tw_merge!(
        "group/form-label peer/form-label flex gap-2 leading-snug w-fit group-data-[disabled=true]/field:opacity-50 has-[>[data-name=Field]]:w-full has-[>[data-name=Field]]:flex-col has-[>[data-name=Field]]:rounded-md has-[>[data-name=Field]]:border [&>*]:data-[name=Field]:p-4 has-data-[state=checked]:bg-primary/5 has-data-[state=checked]:border-primary dark:has-data-[state=checked]:bg-primary/10",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        Label { class: "{merged}", html_for: field_name,
            {children}
        }
    }
}

/* ========================================================== */
/*                   ✨ FORM SEPARATOR ✨                     */
/* ========================================================== */

#[component]
pub fn FormSeparator(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] children: Option<Element>,
) -> Element {
    let has_content = children.is_some();
    let merged = tw_merge!(
        "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "FormSeparator",
            "data-content": "{has_content}",
            class: "{merged}",
            Separator { class: "absolute inset-0 top-1/2" }
            if let Some(children) = children {
                span {
                    class: "block relative px-2 mx-auto bg-background text-muted-foreground w-fit",
                    "data-name": "FormSeparatorContent",
                    {children}
                }
            }
        }
    }
}

/* ========================================================== */
/*                   ✨ FORM ERROR ✨                         */
/* ========================================================== */

#[component]
pub fn FormError(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] children: Option<Element>,
    #[props(optional)] errors: Option<Vec<String>>,
) -> Element {
    // If children is provided, render it directly
    if let Some(children) = children {
        let merged = tw_merge!("text-destructive text-sm font-normal", class.as_deref().unwrap_or(""));
        return rsx! {
            div { role: "alert", "data-name": "FormError", class: "{merged}",
                {children}
            }
        };
    }

    // If errors provided, handle them
    if let Some(ref errs) = errors {
        let merged = tw_merge!("text-destructive text-sm font-normal", class.as_deref().unwrap_or(""));
        if errs.is_empty() {
            return rsx! { {} };
        } else if errs.len() == 1 {
            let msg = errs[0].clone();
            return rsx! {
                div { role: "alert", "data-name": "FormError", class: "{merged}",
                    span { "{msg}" }
                }
            };
        } else {
            let errs = errs.clone();
            return rsx! {
                div { role: "alert", "data-name": "FormError", class: "{merged}",
                    ul { class: "flex flex-col gap-1 ml-4 list-disc",
                        for error in errs {
                            li { "{error}" }
                        }
                    }
                }
            };
        }
    }

    // Otherwise, try to get error from field context
    let field_ctx = consume_context::<FieldContext>();
    let form_ctx = consume_context::<FormContext>();

    let field_name = field_ctx.name.clone();
    let merged = tw_merge!("text-destructive text-sm font-normal", class.as_deref().unwrap_or(""));
    let is_touched = form_ctx.touched_signal.read().contains(&field_name);
    if !is_touched {
        return rsx! { {} };
    }
    let err = form_ctx.errors_signal.read().get(&field_name).and_then(|e| e.clone());
    if let Some(err) = err {
        return rsx! {
            div { role: "alert", "data-name": "FormError", class: "{merged}",
                span { "{err}" }
            }
        };
    }

    rsx! { {} }
}

/* ========================================================== */
/*                  ✨ FORM COMPONENTS ✨                     */
/* ========================================================== */

#[component]
pub fn FormField(#[props(into)] field: String, children: Element) -> Element {
    use_context_provider(|| FieldContext { name: field.clone() });

    let ctx = consume_context::<FormContext>();
    let is_touched = ctx.touched_signal.read().contains(&field);
    let has_error = ctx.errors_signal.read().get(&field).is_some_and(|e| e.is_some());
    let invalid = if is_touched && has_error { "true" } else { "false" };

    rsx! {
        FormFieldWrapper { data_invalid: invalid.to_string(),
            {children}
        }
    }
}

#[component]
pub fn FormInput(
    #[props(into, optional)] placeholder: Option<String>,
    #[props(into, optional)] r#type: Option<String>,
    #[props(into, optional)] id: Option<String>,
) -> Element {
    let field_name = consume_context::<FieldContext>().name;
    let form_ctx = consume_context::<FormContext>();
    let field_id = id.unwrap_or_else(|| field_name.clone());
    let input_type = r#type.unwrap_or_else(|| "text".to_string());

    let current_value = form_ctx.values_signal.read().get(&field_name).cloned().unwrap_or_default();

    // Mirrors leptos `FormInput`, which wires `attr:aria-invalid` reactively from touched + error state.
    let is_touched = form_ctx.touched_signal.read().contains(&field_name);
    let has_error = form_ctx.errors_signal.read().get(&field_name).is_some_and(|e| e.is_some());
    let aria_invalid = if is_touched && has_error { Some("true") } else { None };

    let set_value = form_ctx.set_value.clone();
    let touch_fn = form_ctx.touch_field.clone();
    let field_name_input = field_name.clone();
    let field_name_blur = field_name.clone();

    // Verbatim leptos `Input` base string (leptos `FormInput` delegates to `<Input />`).
    let class = tw_merge!(
        "text-foreground file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-ring focus-visible:ring-ring/50",
        "focus-visible:ring-2",
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        "read-only:bg-muted"
    );

    rsx! {
        input {
            "data-slot": "input-group-control",
            "data-name": "Input",
            id: "{field_id}",
            r#type: "{input_type}",
            placeholder: placeholder.as_deref().unwrap_or(""),
            value: "{current_value}",
            "aria-invalid": aria_invalid,
            class: "{class}",
            oninput: move |ev| {
                set_value(&field_name_input, ev.value());
            },
            onblur: move |_| {
                touch_fn(&field_name_blur);
            }
        }
    }
}
