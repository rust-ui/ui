use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}

impl MessageAlign {
    fn as_str(&self) -> &'static str {
        match self {
            MessageAlign::Start => "Start",
            MessageAlign::End => "End",
        }
    }
}

#[component]
pub fn MessageGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex min-w-0 flex-col gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "MessageGroup", class: "{merged}", {children} } }
}

#[component]
pub fn MessageContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex w-full min-w-0 flex-col gap-2.5 wrap-break-word group-data-[align=End]/message:*:data-name:self-end",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "MessageContent", class: "{merged}", {children} } }
}

#[component]
pub fn MessageHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex max-w-full min-w-0 items-center px-3 text-xs font-medium text-muted-foreground group-has-data-[variant=Ghost]/message:px-0",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "MessageHeader", class: "{merged}", {children} } }
}

#[component]
pub fn MessageFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex max-w-full min-w-0 items-center px-3 text-xs font-medium text-muted-foreground group-has-data-[variant=Ghost]/message:px-0 group-data-[align=End]/message:justify-end",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "MessageFooter", class: "{merged}", {children} } }
}

#[component]
pub fn MessageAvatar(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex w-fit min-w-8 shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted group-has-data-[name=MessageFooter]/message:-translate-y-8",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "MessageAvatar", class: "{merged}", {children} } }
}

#[component]
pub fn Message(
    #[props(default = MessageAlign::default())] align: MessageAlign,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "group/message relative flex w-full min-w-0 gap-2 text-sm data-[align=End]:flex-row-reverse",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { "data-name": "Message", "data-align": align.as_str(), class: "{merged}", {children} }
    }
}
