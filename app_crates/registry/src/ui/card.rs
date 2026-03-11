use leptos::prelude::*;
use leptos_ui::clx;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardSize {
    #[default]
    Default,
    Sm,
}

mod components {
    use super::*;

    clx! {CardHeader, div, "@container/card-header flex flex-col items-start gap-1.5 px-6 [[data-size=sm]_&]:px-4 [.border-b]:pb-6 sm:grid sm:auto-rows-min sm:grid-rows-[auto_auto] has-data-[slot=card-action]:sm:grid-cols-[1fr_auto]"}
    clx! {CardTitle, h2, "leading-none font-semibold"}
    clx! {CardContent, div, "px-6 [[data-size=sm]_&]:px-4"}
    clx! {CardDescription, p, "text-muted-foreground text-sm"}
    clx! {CardFooter, footer, "flex items-center px-6 [[data-size=sm]_&]:px-4 [.border-t]:pt-6", "gap-2"}

    // TODO. Change data-slot=card-action by data-name="CardAction".
    clx! {CardAction, div, "self-start sm:col-start-2 sm:row-span-2 sm:row-start-1 sm:justify-self-end"}
    clx! {CardList, ul, "flex flex-col gap-4"}
    clx! {CardItem, li, "flex items-center [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0"}
}

#[component]
pub fn Card(
    #[prop(into, optional)] size: CardSize,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let size_classes = match size {
        CardSize::Default => "py-6 gap-4",
        CardSize::Sm => "py-4 gap-3",
    };
    let data_size = match size {
        CardSize::Default => "default",
        CardSize::Sm => "sm",
    };
    let merged = tw_merge::tw_merge!(
        "bg-card text-card-foreground flex flex-col rounded-xl border shadow-sm",
        size_classes,
        class
    );

    view! {
        <div class=merged data-size=data_size data-name="Card">
            {children()}
        </div>
    }
}

pub use components::*;
