use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}

/* ========================================================== */
/*                     Components (clx!)                      */
/* ========================================================== */

mod components {
    use super::*;

    clx! { MessageGroup, div, "flex min-w-0 flex-col gap-2" }

    clx! {
        MessageContent,
        div,
        // TODO PORT: group-data-[align=End]/message:*:data-name:self-end is a
        // Tailwind v4 presence selector — any direct child with a data-name attr
        // gets self-end on End-aligned messages. If child alignment doesn't work,
        // use explicit per-child selectors (e.g. *:data-[name=Bubble]:self-end).
        "flex w-full min-w-0 flex-col gap-2.5 wrap-break-word group-data-[align=End]/message:*:data-name:self-end"
    }

    clx! {
        MessageHeader,
        div,
        "flex max-w-full min-w-0 items-center px-3 text-xs font-medium text-muted-foreground group-has-data-[variant=Ghost]/message:px-0"
    }

    clx! {
        MessageFooter,
        div,
        "flex max-w-full min-w-0 items-center px-3 text-xs font-medium text-muted-foreground group-has-data-[variant=Ghost]/message:px-0 group-data-[align=End]/message:justify-end"
    }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn MessageAvatar(
    // TODO PORT: group-has-data-[name=MessageFooter]/message checks for a
    // MessageFooter descendant to shift avatar up. Requires Tailwind v4
    // group-has-data-[name=X]/scope syntax. If avatar doesn't shift, this
    // selector may not be supported and a JS or signal-based solution is needed.
    #[prop(optional, into)] class: String,
    // Optional: empty <MessageAvatar /> used as spacing placeholder in grouped messages
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex w-fit min-w-8 shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted group-has-data-[name=MessageFooter]/message:-translate-y-8",
        class
    );
    view! {
        <div class=merged_class data-name="MessageAvatar">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn Message(
    #[prop(optional)] align: MessageAlign,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged_class =
        tw_merge!("group/message relative flex w-full min-w-0 gap-2 text-sm data-[align=End]:flex-row-reverse", class);
    let align_str = align.to_string();
    view! {
        <div class=merged_class data-name="Message" data-align=align_str>
            {children()}
        </div>
    }
}
