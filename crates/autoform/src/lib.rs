//! AutoForm derive macro for automatic form generation.
//!
//! This macro generates form UI from Rust structs at compile-time.
//!
//! # Example
//!
//! ```rust,ignore
//! #[derive(AutoForm, Validate, Serialize, Deserialize, Clone, Default)]
//! struct ContactForm {
//!     #[autoform(label = "Full Name", placeholder = "John Doe")]
//!     #[validate(length(min = 2))]
//!     name: String,
//!
//!     #[autoform(field_type = "textarea")]
//!     bio: Option<String>,
//!
//!     subscribe: bool,
//! }
//! ```

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

/// Parsed field-level attributes from `#[autoform(...)]`
#[derive(Debug, FromField)]
#[darling(attributes(autoform))]
struct AutoFormFieldAttr {
    ident: Option<syn::Ident>,
    ty: Type,
    /// Custom label text (defaults to field name in Title Case)
    #[darling(default)]
    label: Option<String>,
    /// Input placeholder text
    #[darling(default)]
    placeholder: Option<String>,
    /// Description text shown below the field
    #[darling(default)]
    description: Option<String>,
    /// Override field type: "text", "textarea", "switch", "checkbox", "number", "email", "password"
    #[darling(default)]
    field_type: Option<String>,
    /// Hide this field from the form
    #[darling(default)]
    hidden: bool,
}

/// Parsed struct-level attributes from `#[autoform(...)]`
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(autoform), supports(struct_named))]
struct AutoFormContainer {
    ident: syn::Ident,
    data: darling::ast::Data<(), AutoFormFieldAttr>,
}

/// Convert a snake_case field name to Title Case
fn to_title_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extract the inner type from Option<T>
fn extract_option_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return Some(inner);
    }
    None
}

/// Determine if a type is a boolean
fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident == "bool";
    }
    false
}

/// Determine if a type is numeric (i32, u32, f64, etc.)
fn is_numeric_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        let ident = segment.ident.to_string();
        return matches!(
            ident.as_str(),
            "i8" | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "f32"
                | "f64"
        );
    }
    false
}

/// Derive macro for automatic form generation.
#[proc_macro_derive(AutoForm, attributes(autoform))]
pub fn auto_form_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let container = match AutoFormContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let struct_ident = &container.ident;
    let fields = container.data.take_struct().map(|s| s.fields).unwrap_or_default();

    let field_renders: Vec<_> = fields
        .iter()
        .filter(|f| !f.hidden)
        .map(|field| {
            let field_ident = field.ident.as_ref();
            let field_name = field_ident.map(|i| i.to_string()).unwrap_or_default();

            // Check if it's an Option type
            let (base_ty, _is_optional) = match extract_option_inner(&field.ty) {
                Some(inner) => (inner, true),
                None => (&field.ty, false),
            };

            // Determine the label
            let label = field.label.clone().unwrap_or_else(|| to_title_case(&field_name));

            // Determine field type
            let is_bool = is_bool_type(base_ty);
            let is_numeric = is_numeric_type(base_ty);

            let field_type = field.field_type.as_deref().unwrap_or({
                if is_bool {
                    "switch"
                } else if is_numeric {
                    "number"
                } else {
                    "text"
                }
            });

            // Generate placeholder attribute if present
            let placeholder_attr = field.placeholder.as_ref().map(|p| {
                quote! { attr:placeholder=#p }
            });

            // Generate description if present
            let description_view = field.description.as_ref().map(|d| {
                quote! { <FormDescription>#d</FormDescription> }
            });

            // Generate placeholder prop for FormTextarea (uses prop, not attr)
            let placeholder_prop = field.placeholder.as_ref().map(|p| {
                quote! { placeholder=#p }
            });

            // Generate the input based on field type
            match field_type {
                "textarea" => {
                    quote! {
                        <FormField field=#field_name>
                            <FormLabel>#label</FormLabel>
                            <FormTextarea #placeholder_prop />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
                "checkbox" => {
                    quote! {
                        <FormField field=#field_name>
                            <FormCheckbox label=#label />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
                "switch" => {
                    quote! {
                        <FormField field=#field_name>
                            <FormSwitch label=#label />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
                "number" => {
                    quote! {
                        <FormField field=#field_name>
                            <FormLabel>#label</FormLabel>
                            <FormInput attr:r#type="number" #placeholder_attr />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
                "email" => {
                    quote! {
                        <FormField field=#field_name>
                            <FormLabel>#label</FormLabel>
                            <FormInput attr:r#type="email" #placeholder_attr />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
                "password" => {
                    quote! {
                        <FormField field=#field_name>
                            <FormLabel>#label</FormLabel>
                            <FormInput attr:r#type="password" #placeholder_attr />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
                _ => {
                    // Default to text input
                    quote! {
                        <FormField field=#field_name>
                            <FormLabel>#label</FormLabel>
                            <FormInput #placeholder_attr />
                            #description_view
                            <FormError />
                        </FormField>
                    }
                }
            }
        })
        .collect();

    // Use crate:: paths since demos using this macro are inside the registry crate
    let expanded = quote! {
        impl crate::hooks::use_form::AutoFormFields for #struct_ident {
            fn render_fields(
                _form: crate::hooks::use_form::Form<Self>,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;
                use crate::ui::form::{
                    FormField, FormLabel, FormInput, FormError, FormDescription
                };
                use crate::ui::auto_form::{FormTextarea, FormCheckbox, FormSwitch};

                view! {
                    #(#field_renders)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
