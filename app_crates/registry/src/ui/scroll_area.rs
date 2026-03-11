use leptos::prelude::*;
use leptos_ui::void;
use tw_merge::*;

mod components {
    use super::*;
    void! {ScrollAreaThumb, div, "bg-border relative flex-1 rounded-full"}
    void! {ScrollAreaCorner, div, "bg-border"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ COMPONENTS ✨                       */
/* ========================================================== */

#[component]
pub fn ScrollArea(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("relative overflow-hidden", class);

    view! {
        <div data-name="ScrollArea" class=merged_class>
            <ScrollAreaViewport>{children()}</ScrollAreaViewport>
            <ScrollBar />
            <ScrollAreaCorner />
        </div>
    }
}

#[component]
pub fn ScrollAreaViewport(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!(
        "focus-visible:ring-ring/50 size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:outline-1 overflow-auto",
        class
    );

    view! {
        <div data-name="ScrollAreaViewport" class=merged_class>
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                       🧬 ENUMS 🧬                          */
/* ========================================================== */

#[derive(Clone, Copy, Default)]
pub enum ScrollBarOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[component]
pub fn ScrollBar(
    #[prop(default = ScrollBarOrientation::default())] orientation: ScrollBarOrientation,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let orientation_class = match orientation {
        ScrollBarOrientation::Vertical => "h-full w-2.5 border-l border-l-transparent",
        ScrollBarOrientation::Horizontal => "h-2.5 flex-col border-t border-t-transparent",
    };

    let merged_class = tw_merge!("flex touch-none p-px transition-colors select-none", orientation_class, class);

    view! {
        <div data-name="ScrollBar" class=merged_class>
            <ScrollAreaThumb />
        </div>
    }
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

#[component]
pub fn SnapScrollArea(
    #[prop(into, default = SnapAreaVariant::default())] variant: SnapAreaVariant,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let snap_item = SnapAreaClass { variant };
    let merged_class = snap_item.with_class(class);

    view! {
        <div data-name="SnapScrollArea" class=merged_class>
            {children()}
        </div>
    }
}

#[derive(TwClass, Default)]
#[tw(class = "")]
pub struct SnapAreaClass {
    variant: SnapAreaVariant,
}

#[derive(TwVariant)]
pub enum SnapAreaVariant {
    // * snap-x by default
    #[tw(default, class = "overflow-x-auto snap-x")]
    Center,
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

#[component]
pub fn SnapItem(
    #[prop(into, default = SnapVariant::default())] variant: SnapVariant,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let snap_item = SnapItemClass { variant };
    let merged_class = snap_item.with_class(class);

    view! {
        <div data-name="SnapItem" class=merged_class>
            {children()}
        </div>
    }
}

#[derive(TwClass, Default)]
#[tw(class = "shrink-0")]
pub struct SnapItemClass {
    variant: SnapVariant,
}

#[derive(TwVariant)]
pub enum SnapVariant {
    // * snap-center by default
    #[tw(default, class = "snap-center")]
    Center,
}
