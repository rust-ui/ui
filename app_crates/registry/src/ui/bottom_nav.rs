use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn BottomNav(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "z-50 mx-auto w-full max-w-lg border-t border-border bg-background pb-[env(safe-area-inset-bottom,0px)]",
        class.as_deref().unwrap_or("")
    );
    rsx! { nav { "data-name": "BottomNav", class: "{c}", {children} } }
}

#[component]
pub fn BottomNavGrid(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "grid grid-flow-col auto-cols-fr h-[var(--bottom__nav__height)] font-medium",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "BottomNavGrid", class: "{c}", {children} } }
}

#[component]
pub fn BottomNavLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "text-sm text-muted-foreground group-hover:text-primary group-aria-[current=page]:text-primary",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "BottomNavLabel", class: "{c}", {children} } }
}

#[component]
pub fn BottomNavButton(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: EventHandler<MouseEvent>,
    #[props(into, optional)] aria_current: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "inline-flex flex-col justify-center items-center px-5 group [&_svg]:mb-2 [&_svg]:text-muted-foreground hover:[&_svg]:text-primary aria-[current=page]:[&_svg]:text-primary active:scale-[0.98]",
        "touch-manipulation [-webkit-tap-highlight-color:transparent] select-none [-webkit-touch-callout:none]",
        "supports-[-webkit-touch-callout:none]:justify-end supports-[-webkit-touch-callout:none]:pb-0 supports-[-webkit-touch-callout:none]:translate-y-1",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            "data-name": "BottomNavButton",
            class: "{c}",
            onclick: move |e| onclick.call(e),
            "aria-current": aria_current.as_deref().unwrap_or(""),
            {children}
        }
    }
}
