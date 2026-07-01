use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum MarkerVariant {
    #[default]
    Default,
    Separator,
    Border,
}

/* ========================================================== */
/*                     Components (clx!)                      */
/* ========================================================== */

mod components {
    use super::*;

    // data-name="MarkerContent" auto-set by clx!
    clx! {
        MarkerContent,
        span,
        "min-w-0 wrap-break-word group-data-[variant=Separator]/marker:flex-none group-data-[variant=Separator]/marker:text-center *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground"
    }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn MarkerIcon(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "size-4 shrink-0 [&_svg:not([class*='size-'])]:size-4",
        class
    );

    view! {
        <span
            class=merged_class
            data-name="MarkerIcon"
            aria-hidden="true"
        >
            {children()}
        </span>
    }
}

#[component]
pub fn Marker(
    #[prop(optional)] variant: MarkerVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "group/marker relative flex min-h-4 w-full items-center gap-2 text-left text-sm text-muted-foreground [&_svg:not([class*='size-'])]:size-4 [a]:underline [a]:underline-offset-3 [a]:hover:text-foreground",
        match variant {
            MarkerVariant::Default => "",
            MarkerVariant::Separator => "before:mr-1 before:h-px before:min-w-0 before:flex-1 before:bg-border after:ml-1 after:h-px after:min-w-0 after:flex-1 after:bg-border",
            MarkerVariant::Border => "border-b border-border pb-2",
        },
        class
    );

    view! {
        <div
            class=merged_class
            data-name="Marker"
            data-variant=variant.to_string()
        >
            {children()}
        </div>
    }
}
