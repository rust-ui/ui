use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn BottomNav(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "sm:hidden fixed bottom-0 inset-x-0 z-50",
        "border-t bg-background/80 backdrop-blur-sm",
        "pb-[env(safe-area-inset-bottom)]",
        class.as_deref().unwrap_or("")
    );
    rsx! { nav { class: "{merged}", {children} } }
}

#[component]
pub fn BottomNavGrid(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "grid grid-cols-3 h-16",
        // iOS Safari: push icons closer to home indicator
        "supports-[-webkit-touch-callout:none]:justify-end supports-[-webkit-touch-callout:none]:pb-0 supports-[-webkit-touch-callout:none]:translate-y-1",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn BottomNavButton(
    #[props(default = false)] active: bool,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex flex-col items-center justify-center gap-1 text-xs transition-transform active:scale-95",
        if active { "text-primary" } else { "text-muted-foreground hover:text-foreground" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            class: "{merged}",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn BottomNavLabel(children: Element) -> Element {
    rsx! { span { class: "text-[10px] font-medium", {children} } }
}
