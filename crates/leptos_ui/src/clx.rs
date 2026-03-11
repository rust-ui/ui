pub use leptos::prelude::*;
pub use paste;
pub use tw_merge::*;

pub use crate::utils::Utils;

/// A macro that creates a component with tailwind class merging
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use leptos_ui::clx;
///
/// // Define the component
/// clx! {Card, div, "rounded-lg p-4", "bg-sky-500"} // 🩵
///
/// #[component]
/// pub fn DemoCard() -> impl IntoView {
///     view! {
///         <Card>"Default: bg-sky-500 🩵"</Card>
///         <Card class="bg-orange-500">"Override: bg-orange-500 🧡"</Card>
///         // └──> 🤯 NO CLASS CONFLICT! Still using the SAME component.
///     }
/// }
/// ```
#[macro_export]
macro_rules! clx {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: String,
            children: Children,
        ) -> impl IntoView {
            let merged_classes = tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class);

            view! {
                <$element
                    class=merged_classes
                    data-name=stringify!($name)
                >
                    {children()}
                </$element>
            }
        }
    };
}

/// A macro that creates self-closing components with tailwind class merging
/// See: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use leptos_ui::void;
///
/// // Define self-closing components
/// void! {MyImage, img, "rounded-lg border"}
/// void! {MyInput, input, "px-3 py-2 border rounded"}
/// void! {MyDiv, div, "w-full h-4 bg-gray-200"}
///
/// #[component]
/// pub fn Demo() -> impl IntoView {
///     view! {
///         <MyImage attr:src="test.jpg" class="w-32" />
///         <MyInput prop:value=move || url().to_string() attr:readonly=true class="flex-1" />
///         <MyDiv class="bg-sky-500" />
///     }
/// }
/// ```
#[macro_export]
macro_rules! void {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: String,
        ) -> impl IntoView {
            let merged_classes = tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class);

            view! {
                <$element
                    class=merged_classes
                    data-name=stringify!($name)
                />
            }
        }
    };
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

// * Must be used with Utils::use_random_transition_name().
#[macro_export]
macro_rules! transition {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: String,
            children: Children,
        ) -> impl IntoView {
            let merged_classes = tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class);

            let random_name = Utils::use_random_transition_name();

            view! {
                <$element
                    class=merged_classes
                    data-name=stringify!($name)
                >
                    {children()}
                </$element>
            }
        }
    };
}
