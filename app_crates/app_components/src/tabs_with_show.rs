use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {TabList, div, "w-fit inline-flex gap-2 justify-center items-center p-1 h-10 rounded-md text-muted-foreground bg-muted", "outline-none"}
    clx! { TabTrigger, button, "inline-flex justify-center items-center py-1.5 px-3 text-sm font-medium whitespace-nowrap rounded-sm transition-all focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm ring-offset-background focus-visible:ring-ring   [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4"}
}

pub use components::*;
