use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

mod components {
    use super::*;

    clx! {ChatCard, div, "flex flex-col w-full rounded-lg border"}
    clx! {ChatHeader, header, "flex items-center border-b"}
    clx! {ChatMessageList, div, "space-y-4"}
    clx! {ChatMessageReceived, div, "flex max-w-[85%]"}
    clx! {ChatMessageSent, div, "flex ml-auto max-w-[85%]"}
    clx! {ChatMessageAvatar, span, "flex shrink-0 overflow-hidden rounded-full"}
    clx! {ChatMessageBubble, div, "py-2 px-3 text-sm rounded-lg"}
    clx! {ChatMessageContent, p, "leading-normal wrap-break-word"}
    clx! {ChatMessageTime, p, "mt-1 text-xs text-right"}
    clx! {ChatFooter, footer, "flex items-center border-t"}
}

pub use components::*;

/// Chat body with auto-scroll to bottom on mount
#[component]
pub fn ChatBody(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("overflow-hidden flex-1", &class);

    Effect::new(move |_| {
        if let Some(el) = document().query_selector("[data-name='ChatBody']").ok().flatten() {
            el.set_scroll_top(el.scroll_height());
        }
    });

    view! {
        <div data-name="ChatBody" class=merged_class>
            {children()}
        </div>
    }
}
