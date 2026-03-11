#[cfg(feature = "leptos")]
use leptos::prelude::*;
#[cfg(feature = "leptos")]
use leptos_ui::clx;

#[cfg(feature = "leptos")]
mod components {
    use super::*;
    clx! {Card, div, "bg-card text-card-foreground flex flex-col gap-4 rounded-xl border py-6 shadow-sm"}
    // TODO. Change data-slot=card-action by data-name="CardAction".
    clx! {CardHeader, div, "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6"}
    clx! {CardTitle, h3, "leading-none font-semibold"}
    clx! {CardContent, div, "px-6"}
    clx! {CardDescription, p, "text-muted-foreground text-sm"}
    clx! {CardFooter, footer, "flex items-center px-6 [.border-t]:pt-6", "gap-2"}

    clx! {CardAction, div, "col-start-2 row-span-2 row-start-1 self-start justify-self-end"}
}

#[cfg(feature = "leptos")]
pub use components::*;
