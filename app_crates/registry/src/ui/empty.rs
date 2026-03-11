use leptos::prelude::*;
use leptos_ui::{clx, variants};

mod components {
    use super::*;
    clx! {Empty, div, "flex flex-col items-center justify-center gap-4 rounded-lg border border-dashed p-8 text-center"}
    clx! {EmptyHeader, div, "flex flex-col items-center gap-2"}
    clx! {EmptyTitle, h3, "text-lg font-semibold leading-none"}
    clx! {EmptyDescription, p, "text-muted-foreground text-sm"}
    clx! {EmptyContent, div, "flex items-center justify-center gap-2"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

variants! {
    EmptyMedia {
        base: "flex shrink-0 items-center justify-center mb-2 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        variants: {
            variant: {
                Default: "bg-transparent",
                Icon: "bg-muted text-foreground flex size-10 shrink-0 items-center justify-center rounded-lg [&_svg:not([class*='size-'])]:size-6",
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
