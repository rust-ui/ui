use leptos::ev;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentSize {
    #[default]
    Default,
    Sm,
    Xs,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentState {
    #[default]
    Done,
    Idle,
    Uploading,
    Processing,
    Error,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentMediaVariant {
    #[default]
    Icon,
    Image,
}

/* ========================================================== */
/*                     Components (clx!)                      */
/* ========================================================== */

mod components {
    use super::*;

    clx! { AttachmentContent, div, "flex min-w-0 flex-1 flex-col" }

    clx! { AttachmentTitle, span, "block min-w-0 truncate font-medium" }

    clx! {
        AttachmentDescription,
        span,
        // TODO PORT: data-[state=error] → data-[state=Error] (PascalCase enum via strum::Display)
        "mt-0.5 block min-w-0 truncate text-xs text-muted-foreground group-data-[state=Error]/attachment:text-destructive/80 max-w-full"
    }

    clx! {
        AttachmentGroup,
        div,
        // TODO PORT: *:data-[slot=attachment]:flex-none/snap-start →
        // *:data-[name=Attachment]:flex-none/snap-start (data-name PascalCase convention)
        "flex min-w-0 scroll-fade-x snap-x snap-mandatory scroll-px-1 scrollbar-none gap-3 overflow-x-auto overscroll-x-contain py-1 *:data-[name=Attachment]:flex-none *:data-[name=Attachment]:snap-start"
    }

    clx! {
        AttachmentActions,
        div,
        // TODO PORT: group-data-[orientation=vertical] → group-data-[orientation=Vertical] (PascalCase)
        "relative z-20 flex shrink-0 items-center group-data-[orientation=Vertical]/attachment:absolute group-data-[orientation=Vertical]/attachment:top-3 group-data-[orientation=Vertical]/attachment:right-3 group-data-[orientation=Vertical]/attachment:gap-1"
    }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Attachment(
    #[prop(optional)] size: AttachmentSize,
    #[prop(optional)] orientation: AttachmentOrientation,
    #[prop(optional)] state: AttachmentState,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    // TODO PORT: has-data-[slot=attachment-content/media] → has-data-[name=AttachmentContent/AttachmentMedia]
    // (data-slot → data-name PascalCase convention throughout)
    let size_class = match size {
        AttachmentSize::Default => {
            "gap-2 text-sm has-data-[name=AttachmentContent]:px-2.5 has-data-[name=AttachmentContent]:py-2 has-data-[name=AttachmentMedia]:p-2"
        }
        AttachmentSize::Sm => {
            "gap-2.5 text-xs has-data-[name=AttachmentContent]:px-2 has-data-[name=AttachmentContent]:py-1.5 has-data-[name=AttachmentMedia]:p-1.5"
        }
        AttachmentSize::Xs => {
            "gap-1.5 rounded-lg text-xs has-data-[name=AttachmentContent]:px-1.5 has-data-[name=AttachmentContent]:py-1 has-data-[name=AttachmentMedia]:p-1"
        }
    };
    let orientation_class = match orientation {
        AttachmentOrientation::Horizontal => "min-w-40 items-center",
        AttachmentOrientation::Vertical => "w-24 flex-col has-data-[name=AttachmentContent]:w-30",
    };
    let merged_class = tw_merge!(
        // TODO PORT: data-[state=error/idle] → data-[state=Error/Idle] (PascalCase enum)
        "group/attachment relative flex w-fit max-w-full min-w-0 shrink-0 flex-wrap rounded-xl border bg-card text-card-foreground transition-colors focus-within:ring-1 focus-within:ring-ring/50 has-[>a,>button]:hover:bg-muted/50 data-[state=Error]:border-destructive/30 data-[state=Idle]:border-dashed",
        size_class,
        orientation_class,
        class
    );
    let size_str = size.to_string();
    let orientation_str = orientation.to_string();
    let state_str = state.to_string();
    view! {
        <div
            class=merged_class
            data-name="Attachment"
            data-size=size_str
            data-orientation=orientation_str
            data-state=state_str
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AttachmentMedia(
    #[prop(optional)] variant: AttachmentMediaVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        AttachmentMediaVariant::Icon => "",
        // TODO PORT: group-data-[state=done/idle] → Done/Idle (PascalCase enums)
        AttachmentMediaVariant::Image => {
            "opacity-60 group-data-[state=Done]/attachment:opacity-100 group-data-[state=Idle]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
        }
    };
    let merged_class = tw_merge!(
        // TODO PORT: all group-data-[x=y] use PascalCase y (enum strum::Display output):
        // orientation=Vertical, size=Sm/Xs, state=Error
        // *:data-[slot=spinner] → *:data-[name=Spinner]
        "relative flex aspect-square w-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-muted text-foreground group-data-[orientation=Vertical]/attachment:w-full group-data-[size=Sm]/attachment:w-8 group-data-[size=Xs]/attachment:w-7 group-data-[size=Xs]/attachment:rounded-md group-data-[state=Error]/attachment:bg-destructive/10 group-data-[state=Error]/attachment:text-destructive group-data-[orientation=Vertical]/attachment:*:data-[name=Spinner]:size-6! [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 group-data-[orientation=Vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 group-data-[size=Xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5",
        variant_class,
        class
    );
    view! {
        <div class=merged_class data-name="AttachmentMedia">
            {children()}
        </div>
    }
}

#[component]
pub fn AttachmentAction(
    // Defaults: variant=Ghost, size=IconXs — matching shadcn AttachmentAction defaults.
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Ghost);
    let size = size.unwrap_or(ButtonSize::IconXs);
    view! {
        <Button variant=variant size=size class=class>
            {children()}
        </Button>
    }
}

#[component]
pub fn AttachmentTrigger(
    // TODO PORT: shadcn AttachmentTrigger uses Base UI useRender to become any element
    // (e.g. render={<a href="..."/>} makes it an <a>; used as DialogTrigger render target).
    // We split into href (→ <a>) and on_click (→ <button>), default <button>.
    // DialogTrigger render integration cannot be ported 1:1 without Leptos Dialog support.
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!("absolute inset-0 z-10 outline-none", class);
    match (href, on_click) {
        (Some(href), _) => view! { <a href=href class=merged_class data-name="AttachmentTrigger" /> }
        .into_any(),
        (_, Some(cb)) => view! { <button type="button" class=merged_class data-name="AttachmentTrigger" on:click=move |e| cb.run(e) /> }
        .into_any(),
        _ => view! { <button type="button" class=merged_class data-name="AttachmentTrigger" /> }
        .into_any(),
    }
}
