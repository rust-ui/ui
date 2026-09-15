use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::model::{TwClassContainer, TwClassField};

pub fn class_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwClassContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_ident = &container.ident;
    let builder_ident = format_ident!("{struct_ident}Builder");

    let fields = container.data.take_struct().expect("Expected struct fields");

    let base_class = container.class.as_ref().map(syn::LitStr::value).unwrap_or_default();

    let merger = {
        if let Some(merger) = container.merger {
            let ident = merger.as_ident();
            quote! {#ident}
        } else {
            quote! {TailwindMerge}
        }
    };

    let builder_struct = {
        let builder_fields = fields.iter().map(|field| {
            let TwClassField { ident, ty, .. } = field;
            quote! { #ident: Option<#ty> }
        });

        quote! {
            #[derive(Copy, Clone, Default)]
            pub struct #builder_ident {
                #(#builder_fields,)*
            }
        }
    };

    let field_idents =
        fields.iter().map(|field| field.ident.as_ref().expect("struct field has ident")).collect::<Vec<_>>();

    let builder_impl = {
        let builder_set_methods = fields.iter().map(|field| {
            let TwClassField { ident, ty, .. } = field;
            quote! {
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
            }
        });

        quote! {
            impl #builder_ident {
                #(#builder_set_methods)*

                pub fn build(self) -> #struct_ident {
                    #struct_ident {
                        #(#field_idents: self.#field_idents.unwrap_or_default(),)*
                    }
                }
            }
        }
    };

    let builder_to_tailwind = {
        quote! {
            impl IntoTailwindClass for #builder_ident {
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    (*self).build().with_class(class)
                }
            }
        }
    };

    let struct_to_tailwind = {
        let field_refs = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.as_class(),
            }
        });

        quote! {
            impl IntoTailwindClass for #struct_ident {
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    let classes = [
                        #base_class,
                        #( #field_refs )*
                        class.as_ref(),
                    ];
                    #merger.compose_classes(&classes)
                }
            }
        }
    };

    let tokens = quote! {
        #builder_struct

        #builder_impl

        impl IntoBuilder for #struct_ident {
            type Builder = #builder_ident;

            fn builder() -> Self::Builder {
                Default::default()
            }
            fn into_builder(self) -> Self::Builder {
                self.into()
            }
        }

        impl From<#struct_ident> for #builder_ident {
            fn from(value: #struct_ident) -> Self {
                #builder_ident {
                    #(#field_idents: Some(value.#field_idents),)*
                }
            }
        }

        impl From<#builder_ident> for #struct_ident {
            fn from(value: #builder_ident) -> Self {
                value.build()
            }
        }

        #builder_to_tailwind

        #struct_to_tailwind
    };

    tokens.into()
}
