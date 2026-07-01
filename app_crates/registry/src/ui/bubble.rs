use leptos::ev;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum BubbleReactionsSide {
    Top,
    #[default]
    Bottom,
}

/* ========================================================== */
/*                     Components (clx!)                      */
/* ========================================================== */

mod components {
    use super::*;

    clx! { BubbleGroup, div, "flex min-w-0 flex-col gap-2" }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Bubble(
    #[prop(optional)] variant: BubbleVariant,
    #[prop(optional)] align: BubbleAlign,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    // Variant classes target BubbleContent children via *:data-[name=BubbleContent].
    // shadcn uses CVA + data-[slot=bubble-content]; we use data-[name=BubbleContent]
    // (PascalCase, set by clx! / #[component] data-name convention).
    let variant_class = match variant {
        BubbleVariant::Default => {
            "*:data-[name=BubbleContent]:bg-primary *:data-[name=BubbleContent]:text-primary-foreground [&>[data-name=BubbleContent]:is(button,a):hover]:bg-primary/80"
        }
        BubbleVariant::Secondary => {
            "*:data-[name=BubbleContent]:bg-secondary *:data-[name=BubbleContent]:text-secondary-foreground [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]"
        }
        BubbleVariant::Muted => {
            "*:data-[name=BubbleContent]:bg-muted [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        }
        BubbleVariant::Tinted => {
            "*:data-[name=BubbleContent]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] *:data-[name=BubbleContent]:text-foreground dark:*:data-[name=BubbleContent]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
        }
        BubbleVariant::Outline => {
            "*:data-[name=BubbleContent]:border-border *:data-[name=BubbleContent]:bg-background [&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted [&>[data-name=BubbleContent]:is(button,a):hover]:text-foreground dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-input/30"
        }
        BubbleVariant::Ghost => {
            "border-none *:data-[name=BubbleContent]:rounded-none *:data-[name=BubbleContent]:bg-transparent *:data-[name=BubbleContent]:p-0 [&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted [&>[data-name=BubbleContent]:is(button,a):hover]:text-foreground dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted/50"
        }
        BubbleVariant::Destructive => {
            "*:data-[name=BubbleContent]:bg-destructive/10 *:data-[name=BubbleContent]:text-destructive dark:*:data-[name=BubbleContent]:bg-destructive/20 [&>[data-name=BubbleContent]:is(button,a):hover]:bg-destructive/20 dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-destructive/30"
        }
    };

    let merged_class = tw_merge!(
        "group/bubble relative flex w-fit max-w-[80%] min-w-0 flex-col gap-1 group-data-[align=End]/message:self-end data-[align=End]:self-end data-[variant=Ghost]:max-w-full",
        variant_class,
        class
    );

    let variant_str = variant.to_string();
    let align_str = align.to_string();

    view! {
        <div class=merged_class data-name="Bubble" data-variant=variant_str data-align=align_str>
            {children()}
        </div>
    }
}

#[component]
pub fn BubbleContent(
    // TODO PORT: shadcn uses BubbleContent asChild (Radix Slot) to swap element type
    // (e.g. <BubbleContent asChild><button onClick={...}>...</button></BubbleContent>).
    // Leptos has no asChild. We split into href (→ <a>) and on_click (→ <button>),
    // with <div> as the default. Matches shadcn behavior for all 3 cases.
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "w-fit max-w-full min-w-0 overflow-hidden rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed wrap-break-word group-data-[align=End]/bubble:self-end [button]:text-left [button,a]:transition-colors [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50",
        class
    );

    match (href, on_click) {
        (Some(href), _) => view! {
            <a href=href class=merged_class data-name="BubbleContent">
                {children()}
            </a>
        }
        .into_any(),
        (_, Some(cb)) => view! {
            <button type="button" class=merged_class data-name="BubbleContent" on:click=move |e| cb.run(e)>
                {children()}
            </button>
        }
        .into_any(),
        _ => view! {
            <div class=merged_class data-name="BubbleContent">
                {children()}
            </div>
        }
        .into_any(),
    }
}

#[component]
pub fn BubbleReactions(
    #[prop(optional)] side: BubbleReactionsSide,
    #[prop(optional)] align: BubbleAlign,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let side_class = match side {
        BubbleReactionsSide::Top => "top-0 -translate-y-3/4",
        BubbleReactionsSide::Bottom => "bottom-0 translate-y-3/4",
    };
    let align_class = match align {
        BubbleAlign::Start => "left-3",
        BubbleAlign::End => "right-3",
    };

    let merged_class = tw_merge!(
        "absolute z-10 flex w-fit shrink-0 items-center justify-center gap-1 rounded-full bg-muted px-1.5 py-0.5 text-sm ring-3 ring-card has-[button]:p-0",
        side_class,
        align_class,
        class
    );

    let side_str = side.to_string();
    let align_str = align.to_string();

    view! {
        <div class=merged_class data-name="BubbleReactions" data-side=side_str data-align=align_str>
            {children()}
        </div>
    }
}
