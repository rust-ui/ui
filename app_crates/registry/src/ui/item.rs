use leptos::prelude::*;
use leptos_ui::{clx, variants};
use tw_merge::tw_merge;

use crate::ui::separator::Separator;

mod components {
    use super::*;
    clx! {ItemGroup, div, "group/item-group flex flex-col"}
    clx! {ItemContent, div, "flex flex-1 flex-col gap-1 [&+[data-slot=item-content]]:flex-none"}
    clx! {ItemTitle, div, "flex w-fit items-center gap-2 text-sm leading-snug font-medium"}
    clx! {ItemDescription, p, "text-muted-foreground line-clamp-2 text-sm leading-normal font-normal text-balance [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4"}
    clx! {ItemActions, div, "flex items-center gap-2"}
    clx! {ItemHeader, div, "flex basis-full items-center justify-between gap-2"}
    clx! {ItemFooter, div, "flex basis-full items-center justify-between gap-2"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

variants! {
    Item {
        base: "group/item flex items-center border border-transparent text-sm rounded-md transition-colors [a]:hover:bg-accent/50 [a]:transition-colors duration-100 flex-wrap outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
        variants: {
            variant: {
                Default: "bg-transparent",
                Outline: "border-border",
                Muted: "bg-muted/50",
            },
            size: {
                Default: "p-4 gap-4",
                Sm: "py-3 px-4 gap-2.5",
                Xs: "py-2 px-3 gap-2",
            }
        },
        component: {
            element: div,
            support_href: true
        }
    }
}

variants! {
    ItemMedia {
        base: "flex shrink-0 items-center justify-center gap-2 group-has-[[data-slot=item-description]]/item:self-start [&_svg]:pointer-events-none group-has-[[data-slot=item-description]]/item:translate-y-0.5",
        variants: {
            variant: {
                Default: "bg-transparent",
                Icon: "size-8 border rounded-sm bg-muted [&_svg:not([class*='size-'])]:size-4",
                Image: "size-10 rounded-sm overflow-hidden [&_img]:size-full [&_img]:object-cover",
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

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ItemSeparator(#[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("my-0", class);

    view! { <Separator attr:data-name="ItemSeparator" class=merged_class /> }
}
