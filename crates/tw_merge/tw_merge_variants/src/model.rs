use darling::util::{Flag, IdentString};
use darling::{FromDeriveInput, FromField, FromVariant, ast};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tw), supports(enum_unit))]
pub struct TwVariantContainer {
    pub ident: syn::Ident,
    pub data: ast::Data<TwVariantOption, ()>,
    /// The base Tailwind class for the variant.
    pub class: Option<syn::LitStr>,
}

#[derive(Debug, FromVariant)]
#[darling(supports(unit), attributes(tw, default))]
pub struct TwVariantOption {
    pub ident: syn::Ident,
    pub class: String,
    pub default: Flag,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tw), supports(struct_named))]
pub struct TwClassContainer {
    pub ident: syn::Ident,
    pub data: ast::Data<(), TwClassField>,
    pub class: Option<syn::LitStr>,
    /// Defaults to using `tw_merge`.
    pub merger: Option<IdentString>,
}

#[derive(Debug, FromField)]
#[darling(attributes(tw))]
pub struct TwClassField {
    pub ty: syn::Type,
    pub ident: Option<syn::Ident>,
}
