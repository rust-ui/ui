use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use icons::X;
use tw_merge::tw_merge;

use crate::ui::button::{ButtonSize, ButtonVariant};

static SHEET_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, PartialEq)]
pub struct SheetContext {
    pub target_id: String,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SheetDirection {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

impl SheetDirection {
    // Position + size copied from leptos `initial_position()` (w-[400px] / h-[400px],
    // no border). The `data-[state=*]` translate pair is dioxus's attribute-driven
    // equivalent of leptos toggling `translate-*` classes from JS.
    fn class(self) -> &'static str {
        match self {
            Self::Top => {
                "top-0 left-0 w-full h-[400px] data-[state=closed]:-translate-y-full data-[state=open]:translate-y-0"
            }
            Self::Bottom => {
                "bottom-0 left-0 w-full h-[400px] data-[state=closed]:translate-y-full data-[state=open]:translate-y-0"
            }
            Self::Left => {
                "top-0 left-0 h-full w-[400px] data-[state=closed]:-translate-x-full data-[state=open]:translate-x-0"
            }
            Self::Right => {
                "top-0 right-0 h-full w-[400px] data-[state=closed]:translate-x-full data-[state=open]:translate-x-0"
            }
        }
    }
}

#[component]
pub fn Sheet(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let sheet_id = use_hook(|| {
        let id = SHEET_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("sheet_{id}")
    });
    provide_context(SheetContext { target_id: sheet_id });
    let c = tw_merge!("w-fit", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "Sheet", class: "{c}", {children} } }
}

#[component]
pub fn SheetTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<SheetContext>();
    rsx! {
        button {
            "data-name": "SheetTrigger",
            "data-sheet-trigger": "{ctx.target_id}",
            class: tw_merge!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 cursor-pointer",
                class.as_deref().unwrap_or("")
            ),
            {children}
        }
    }
}

#[component]
pub fn SheetContent(
    #[props(into, optional)] class: Option<String>,
    #[props(default = SheetDirection::Right)] direction: SheetDirection,
    #[props(default = true)] close_on_backdrop_click: bool,
    #[props(default = true)] show_close_button: bool,
    children: Element,
) -> Element {
    let ctx = use_context::<SheetContext>();
    let target_id = ctx.target_id.clone();
    let backdrop_id = format!("{}_backdrop", target_id);
    let backdrop_behavior = if close_on_backdrop_click { "auto" } else { "manual" };

    let c = tw_merge!(
        "fixed z-100 bg-card shadow-lg p-6 transition-transform duration-300 overflow-y-auto overscroll-y-contain",
        direction.class(),
        class.as_deref().unwrap_or("")
    );

    let close_btn_class = format!(
        "absolute top-4 right-4 p-1 rounded-sm focus:ring-2 focus:ring-offset-2 focus:outline-none [&_svg:not([class*='size-'])]:size-4 focus:ring-ring{}",
        if show_close_button { "" } else { " hidden" }
    );

    let tid = target_id.clone();
    let bid = backdrop_id.clone();
    let script = format!(
        r#"(function() {{
        const setup = () => {{
            const sheet = document.getElementById('{tid}');
            const backdrop = document.getElementById('{bid}');
            const trigger = document.querySelector('[data-sheet-trigger="{tid}"]');
            if (!sheet || !backdrop || !trigger) {{ setTimeout(setup, 50); return; }}
            if (sheet.hasAttribute('data-initialized')) return;
            sheet.setAttribute('data-initialized', 'true');
            const open = () => {{
                sheet.setAttribute('data-state', 'open');
                backdrop.setAttribute('data-state', 'open');
                sheet.style.pointerEvents = 'auto';
                backdrop.style.pointerEvents = 'auto';
                if (window.ScrollLock) window.ScrollLock.lock();
            }};
            const close = () => {{
                sheet.setAttribute('data-state', 'closed');
                backdrop.setAttribute('data-state', 'closed');
                sheet.style.pointerEvents = 'none';
                backdrop.style.pointerEvents = 'none';
                if (window.ScrollLock) window.ScrollLock.unlock(300);
            }};
            trigger.addEventListener('click', open);
            sheet.querySelectorAll('[data-sheet-close]').forEach(b => b.addEventListener('click', close));
            backdrop.addEventListener('click', () => {{ if (sheet.getAttribute('data-backdrop') === 'auto') close(); }});
            document.addEventListener('keydown', e => {{ if (e.key === 'Escape' && sheet.getAttribute('data-state') === 'open') {{ e.preventDefault(); close(); }} }});
        }};
        if (document.readyState === 'loading') {{ document.addEventListener('DOMContentLoaded', setup); }} else {{ setup(); }}
    }})();"#
    );

    rsx! {
        div {
            "data-name": "SheetBackdrop",
            id: "{backdrop_id}",
            class: "fixed inset-0 z-60 bg-black/50 pointer-events-none opacity-0 transition-opacity duration-300 data-[state=open]:opacity-100",
            "data-state": "closed",
            "data-backdrop": "{backdrop_behavior}",
        }
        div {
            "data-name": "SheetContent",
            id: "{target_id}",
            class: "{c}",
            "data-state": "closed",
            style: "pointer-events: none;",
            button {
                r#type: "button",
                class: "{close_btn_class}",
                "data-sheet-close": "{target_id}",
                "aria-label": "Close sheet",
                span { class: "hidden", "Close Sheet" }
                X {}
            }
            {children}
        }
        script { dangerous_inner_html: "{script}" }
    }
}

#[component]
pub fn SheetClose(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default = ButtonSize::Default)] size: ButtonSize,
    children: Element,
) -> Element {
    let ctx = use_context::<SheetContext>();
    rsx! {
        button {
            "data-name": "SheetClose",
            "data-sheet-close": "{ctx.target_id}",
            class: tw_merge!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] hover:cursor-pointer",
                variant.as_str(),
                size.as_str(),
                class.as_deref().unwrap_or("")
            ),
            {children}
        }
    }
}

#[component]
pub fn SheetHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex flex-col gap-0.5 p-4", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SheetHeader", class: "{c}", {children} } }
}

#[component]
pub fn SheetTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("font-bold text-2xl", class.as_deref().unwrap_or(""));
    rsx! { h2 { "data-name": "SheetTitle", class: "{c}", {children} } }
}

#[component]
pub fn SheetDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "SheetDescription", class: "{c}", {children} } }
}

#[component]
pub fn SheetBody(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex flex-col gap-4", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SheetBody", class: "{c}", {children} } }
}

#[component]
pub fn SheetFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("mt-auto flex flex-col gap-2 p-4", class.as_deref().unwrap_or(""));
    rsx! { footer { "data-name": "SheetFooter", class: "{c}", {children} } }
}
