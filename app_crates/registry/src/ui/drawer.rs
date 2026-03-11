use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};

mod components {
    use super::*;
    clx! {DrawerBody, div, "flex flex-col gap-4 mx-auto max-w-[500px]"}
    clx! {DrawerTitle, h3, "text-lg leading-none font-semibold"}
    clx! {DrawerDescription, p, "text-sm text-muted-foreground"}
    clx! {DrawerHeader, div, "flex flex-col gap-2"}
    clx! {DrawerFooter, footer, "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"}
}

pub use components::*;

#[component]
pub fn DrawerTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    view! {
        <Button data_name="DrawerTrigger" class=class variant=variant size=size>
            {children()}
        </Button>
    }
}

#[component]
pub fn DrawerClose(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    view! {
        <Button data_name="DrawerClose" class=class variant=variant size=size>
            {children()}
        </Button>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Drawer(
    children: Children,
    #[prop(optional, default = true)] show_overlay: bool,
    #[prop(optional, default = true)] lock_body_scroll: bool,
) -> impl IntoView {
    let overlay_class = if show_overlay { "hidden fixed inset-0 z-200 bg-black/50" } else { "!hidden" };
    let lock_scroll_attr = if lock_body_scroll { "true" } else { "false" };

    view! {
        <link rel="stylesheet" href="/app/vaul_drawer.css" />

        <div
            data-name="DrawerOverlay"
            class=overlay_class
            data-vaul-overlay=""
            data-vaul-snap-points="false"
            data-vaul-animate="true"
            data-state="closed"
            data-lock-body-scroll=lock_scroll_attr
        ></div>

        {children()}

        <script type="module" src="/app/vaul_drawer.js"></script>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum DrawerPosition {
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum DrawerVariant {
    #[default]
    Inset,
    Floating,
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = DrawerPosition::default())] position: DrawerPosition,
    #[prop(optional, default = DrawerVariant::default())] variant: DrawerVariant,
    #[prop(into, default = "--initial-transform: 100%;".to_string())] style: String,
    #[prop(into, default = "true".to_string())] dismissible: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex flex-col pt-3 pb-6 px-6 hidden fixed right-0 bottom-0 left-0 z-210 bg-background max-h-[96vh] rounded-t-[10px]",
        class
    );

    view! {
        <div
            data-name="DrawerContent"
            class=merged_class
            data-vaul-drawer=""
            data-vaul-drawer-position=position.to_string()
            data-vaul-variant=variant.to_string()
            data-vaul-snap-points="false"
            data-vaul-animate="true"
            data-vaul-dismissible=dismissible
            data-state="closed"
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerHandle() -> impl IntoView {
    view! {
        <div
            class="block relative mx-auto mb-8 w-8 rounded-2xl opacity-70 hover:opacity-100 active:opacity-100 shrink-0 bg-[#e2e2e4] h-[5px]"
            data-vaul-handle=""
        >
            <span data-vaul-handle-hitarea=""></span>
        </div>
    }
}
