// Re-export dependencies for macro usage
pub use leptos;
pub use paste;
pub use tw_merge;

/// Ultra-sophisticated variants macro for standardized Tailwind CSS component patterns
///
/// This macro generates:
/// - A main class struct with TwClass derive for tw_merge integration
/// - Variant enums with TwVariant derive and proper Default implementations
/// - Size enums with TwVariant derive and proper Default implementations
/// - Optional Leptos component function with standard props
/// - First variant/size automatically becomes the default
///
/// # Usage
///
/// ```rust
/// use leptos_ui::variants;
///
/// variants! {
///     Badge {
///         base: "inline-flex items-center font-semibold rounded-md border transition-colors w-fit",
///         variants: {
///             variant: {
///                 Default: "border-transparent shadow bg-primary text-primary-foreground hover:bg-primary/80",
///                 Secondary: "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
///                 Destructive: "border-transparent shadow bg-destructive text-destructive-foreground hover:bg-destructive/80",
///                 Success: "border-transparent shadow bg-success text-success-foreground hover:bg-success/80",
///                 Warning: "border-transparent shadow bg-warning text-warning-foreground hover:bg-warning/80",
///                 Outline: "text-foreground border-border",
///             },
///             size: {
///                 Default: "px-2.5 py-0.5 text-xs",
///                 Sm: "px-1.5 py-0.5 text-[10px]",
///                 Lg: "px-3 py-1 text-sm",
///                 Xl: "px-4 py-1.5 text-base",
///             }
///         }
///     }
/// }
/// ```
///
/// This generates:
/// - `BadgeClass` struct
/// - `BadgeVariant` enum with Default, Secondary, Destructive, Success, Warning, Outline variants
/// - `BadgeSize` enum with Default, Sm, Lg, Xl variants
///
/// # With Component Generation
///
/// ```rust
/// variants! {
///     Badge {
///         base: "inline-flex items-center font-semibold rounded-md border transition-colors w-fit",
///         variants: {
///             variant: {
///                 Default: "border-transparent shadow bg-primary text-primary-foreground hover:bg-primary/80",
///                 Secondary: "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
///             },
///             size: {
///                 Default: "px-2.5 py-0.5 text-xs",
///                 Sm: "px-1.5 py-0.5 text-[10px]",
///             }
///         },
///         component: {
///             element: span
///         }
///     }
/// }
/// ```
///
/// This generates the same structs/enums PLUS a `Badge` component function.
///
#[macro_export]
macro_rules! variants {
    // Main pattern with component: variant + size + component generation
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            },
            component: {
                element: $element:ident
            }
        }
    ) => {
        $crate::paste::paste! {
            use $crate::tw_merge::*;

            // Generate the main TwClass struct
            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            // Generate variant enum
            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(
                    #[tw(class = $variant_class)]
                    $variant_key,
                )*
            }

            // Generate size enum
            #[derive(TwVariant)]
            pub enum [<$component Size>] {
                #[tw(default, class = $first_size_class)]
                $first_size,
                $(
                    #[tw(class = $size_class)]
                    $size_key,
                )*
            }

            // Generate the component function
            #[::leptos::component]
            pub fn $component(
                #[prop(into, optional)] variant: ::leptos::prelude::Signal<[<$component Variant>]>,
                #[prop(into, optional)] size: ::leptos::prelude::Signal<[<$component Size>]>,
                #[prop(into, optional)] class: ::leptos::prelude::Signal<String>,
                #[prop(into, optional)] data_name: Option<String>,
                children: ::leptos::prelude::Children,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;

                let computed_class = move || {
                    let variant = variant.try_get().unwrap_or_default();
                    let size = size.try_get().unwrap_or_default();
                    let component_class = [<$component Class>] { variant, size };
                    component_class.with_class(class.try_get().unwrap_or_default())
                };

                let data_name = data_name.unwrap_or_else(|| stringify!($component).to_string());

                view! {
                    <$element class=computed_class data-name=data_name>
                        {children()}
                    </$element>
                }
            }
        }
    };

    // Component with support_href + support_aria_current
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            },
            component: {
                element: $element:ident,
                support_href: true,
                support_aria_current: true
            }
        }
    ) => {
        $crate::paste::paste! {
            use $crate::tw_merge::*;

            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(#[tw(class = $variant_class)] $variant_key,)*
            }

            #[derive(TwVariant)]
            pub enum [<$component Size>] {
                #[tw(default, class = $first_size_class)]
                $first_size,
                $(#[tw(class = $size_class)] $size_key,)*
            }

            #[::leptos::component]
            pub fn $component(
                #[prop(into, optional)] variant: ::leptos::prelude::Signal<[<$component Variant>]>,
                #[prop(into, optional)] size: ::leptos::prelude::Signal<[<$component Size>]>,
                #[prop(into, optional)] class: ::leptos::prelude::Signal<String>,
                #[prop(into, optional)] data_name: Option<String>,
                #[prop(into, optional)] href: Option<String>,
                children: ::leptos::prelude::Children,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;

                let computed_class = move || {
                    let variant = variant.try_get().unwrap_or_default();
                    let size = size.try_get().unwrap_or_default();
                    let component_class = [<$component Class>] { variant, size };
                    component_class.with_class(class.try_get().unwrap_or_default())
                };

                let data_name = data_name.unwrap_or_else(|| stringify!($component).to_string());

                if href.is_some() {
                    use ::leptos_router::hooks::use_location;

                    let location = use_location();
                    let href_clone = href.clone();
                    let is_active = move || {
                        href_clone.as_ref().map_or(false, |h| {
                            let path = location.pathname.try_get().unwrap_or_default();
                            path == *h || path.starts_with(&format!("{}/", h))
                        })
                    };
                    let aria_current = move || if is_active() { "page" } else { "false" };

                    view! {
                        <a class=computed_class href=href.unwrap() aria-current=aria_current data-name=data_name>
                            {children()}
                        </a>
                    }
                    .into_any()
                } else {
                    view! {
                        <$element class=computed_class data-name=data_name>
                            {children()}
                        </$element>
                    }
                    .into_any()
                }
            }
        }
    };

    // Component with support_href only
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            },
            component: {
                element: $element:ident,
                support_href: true
            }
        }
    ) => {
        $crate::paste::paste! {
            use $crate::tw_merge::*;

            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(#[tw(class = $variant_class)] $variant_key,)*
            }

            #[derive(TwVariant)]
            pub enum [<$component Size>] {
                #[tw(default, class = $first_size_class)]
                $first_size,
                $(#[tw(class = $size_class)] $size_key,)*
            }

            #[::leptos::component]
            pub fn $component(
                #[prop(into, optional)] variant: ::leptos::prelude::Signal<[<$component Variant>]>,
                #[prop(into, optional)] size: ::leptos::prelude::Signal<[<$component Size>]>,
                #[prop(into, optional)] class: ::leptos::prelude::Signal<String>,
                #[prop(into, optional)] data_name: Option<String>,
                #[prop(into, optional)] href: Option<String>,
                children: ::leptos::prelude::Children,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;

                let computed_class = move || {
                    let variant = variant.try_get().unwrap_or_default();
                    let size = size.try_get().unwrap_or_default();
                    let component_class = [<$component Class>] { variant, size };
                    component_class.with_class(class.try_get().unwrap_or_default())
                };

                let data_name = data_name.unwrap_or_else(|| stringify!($component).to_string());

                if href.is_some() {
                    view! {
                        <a class=computed_class href=href.unwrap() data-name=data_name>
                            {children()}
                        </a>
                    }
                    .into_any()
                } else {
                    view! {
                        <$element class=computed_class data-name=data_name>
                            {children()}
                        </$element>
                    }
                    .into_any()
                }
            }
        }
    };

    // Main pattern: variant + size (lowercase structural keywords)
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            use $crate::tw_merge::*;

            // Generate the main TwClass struct
            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            // Generate variant enum - inline expansion, no nested macros
            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(
                    #[tw(class = $variant_class)]
                    $variant_key,
                )*
            }

            // Generate size enum - inline expansion, no nested macros
            #[derive(TwVariant)]
            pub enum [<$component Size>] {
                #[tw(default, class = $first_size_class)]
                $first_size,
                $(
                    #[tw(class = $size_class)]
                    $size_key,
                )*
            }

            // TwVariant derive automatically provides Default, Clone, Copy implementations
        }
    };

    // Variant-only pattern (no size)
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            use $crate::tw_merge::*;

            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
            }

            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(
                    #[tw(class = $variant_class)]
                    $variant_key,
                )*
            }

            // TwVariant derive automatically provides Default, Clone, Copy implementations
        }
    };

    // Size-only pattern (no variants)
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            use $crate::tw_merge::*;

            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub size: [<$component Size>],
            }

            #[derive(TwVariant)]
            pub enum [<$component Size>] {
                #[tw(default, class = $first_size_class)]
                $first_size,
                $(
                    #[tw(class = $size_class)]
                    $size_key,
                )*
            }

            // TwVariant derive automatically provides Default, Clone, Copy implementations
        }
    };
}
