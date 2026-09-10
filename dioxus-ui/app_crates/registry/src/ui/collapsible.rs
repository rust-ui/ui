use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: Signal<bool>,
}

#[component]
pub fn Collapsible(
    #[props(default = false)] default_open: bool,
    #[props(into, optional)] open: Option<bool>,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let mut open_signal = use_signal(|| open.unwrap_or(default_open));
    if let Some(controlled) = open
        && controlled != open_signal()
    {
        open_signal.set(controlled);
    }
    provide_context(CollapsibleCtx { open: open_signal });
    let open = open_signal;

    let state = if open() { "open" } else { "closed" };
    let class = tw_merge!("", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            "data-name": "Collapsible",
            "data-state": "{state}",
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let CollapsibleCtx { mut open } = use_context::<CollapsibleCtx>();
    let state = if open() { "open" } else { "closed" };

    rsx! {
        button {
            r#type: "button",
            "data-name": "CollapsibleTrigger",
            "data-state": "{state}",
            class: "{class.as_deref().unwrap_or(\"\")}",
            onclick: move |e| {
                open.set(!open());
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] outer_class: Option<String>,
    children: Element,
) -> Element {
    let CollapsibleCtx { open } = use_context::<CollapsibleCtx>();
    let state = if open() { "open" } else { "closed" };

    let outer = tw_merge!(
        "grid overflow-hidden transition-all duration-300 data-[state=closed]:grid-rows-[0fr] data-[state=open]:grid-rows-[1fr]",
        outer_class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "CollapsibleContent",
            "data-state": "{state}",
            class: "{outer}",
            div { class: tw_merge!("min-h-0", class.as_deref().unwrap_or("")), {children} }
        }
    }
}
