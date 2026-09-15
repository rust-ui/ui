use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::model::TwVariantContainer;

pub fn variant_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwVariantContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let enum_ident = &container.ident;

    let variants = container.data.take_enum().unwrap_or_else(Vec::new);

    let base_class = container.class;

    let defaults = variants.iter().filter(|v| v.default.is_present()).collect::<Vec<_>>();

    if defaults.is_empty() {
        return syn::Error::new(
            enum_ident.span(),
            "No default variant specified. Please mark one variant with `#[tw(default)]`",
        )
        .to_compile_error()
        .into();
    }

    if defaults.len() > 1 {
        let error = format!(
            "Only one variant can be marked as default: {:?}",
            defaults.iter().map(|v| v.ident.to_string()).collect::<Vec<_>>()
        );
        let span = defaults[1].default.span();
        return syn::Error::new(span, error).to_compile_error().into();
    }

    let default_variant = defaults.into_iter().next().map(|v| {
        let variant_ident = &v.ident;
        quote! {
            impl Default for #enum_ident {
                fn default() -> Self {
                    #enum_ident::#variant_ident
                }
            }
        }
    });

    // Make a constant for each field and the base class
    let enum_ident_string = enum_ident.to_string().to_ascii_uppercase();
    let constants = variants
        .iter()
        .map(|variant| {
            (
                variant,
                syn::Ident::new(
                    &format!("{}_{}", variant.ident.to_string().to_ascii_uppercase(), enum_ident_string),
                    proc_macro2::Span::call_site(),
                ),
            )
        })
        .collect::<Vec<_>>();

    let to_class_cases = constants.iter().map(|(variant, constant)| {
        let variant_ident = &variant.ident;

        quote! {
            #enum_ident::#variant_ident =>  #constant,
        }
    });

    let into_tailwind = quote! {
        impl AsTailwindClass for #enum_ident {
            fn as_class(&self) -> &str {
                match self {
                    #( #to_class_cases )*
                }
            }
        }
    };

    let constant_variables = constants.iter().map(|(variant, constant)| {
        let class = &variant.class;
        if let Some(base_class) = &base_class {
            quote! {
                const #constant: &'static str = concat!(#base_class, " ", #class);
            }
        } else {
            quote! {
                const #constant: &'static str = #class;
            }
        }
    });

    let tokens = quote! {
        #default_variant

        #into_tailwind

        #( #constant_variables )*

        impl Copy for #enum_ident {}
        impl Clone for #enum_ident {
            fn clone(&self) -> Self {
                *self
            }
        }
    };

    tokens.into()
}
