use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {BentoGrid, div, "grid gap-2 md:grid-cols-4"}
    clx! {BentoGrid6, div, "grid gap-2 sm:grid-cols-2 md:grid-cols-4"}
    clx! {BentoRow, div, "p-1 min-h-32 rounded-lg"}
    clx! {BentoCell, div, "text-xl rounded-lg size-full center bg-zinc-200 dark:bg-zinc-700"}
}

pub use components::*;
