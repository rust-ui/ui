use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use tw_merge::tw_merge;

static POPOVER_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_popover_id() -> String {
    use_hook(|| {
        let id = POPOVER_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{id}")
    })
}

#[derive(Clone)]
struct PopoverContext {
    anchor_name: String,
    trigger_id: String,
    content_id: String,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum PopoverAlign {
    Start,
    StartOuter,
    End,
    EndOuter,
    #[default]
    Center,
}

#[component]
pub fn Popover(#[props(default = PopoverAlign::Center)] align: PopoverAlign, children: Element) -> Element {
    let id = use_popover_id();
    let anchor_name = format!("--anchor_{id}");
    let trigger_id = format!("popover-trigger-{id}");
    let content_id = format!("popover_{id}");

    provide_context(PopoverContext {
        anchor_name: anchor_name.clone(),
        trigger_id: trigger_id.clone(),
        content_id: content_id.clone(),
    });

    let (position_styles, transform_origin) = match align {
        PopoverAlign::Start => (
            "left: anchor(left); bottom: anchor(top); margin-bottom: 8px; @position-try(flip-block) { top: anchor(bottom); bottom: auto; margin-top: 8px; margin-bottom: 0; }".to_string(),
            "left top".to_string(),
        ),
        PopoverAlign::StartOuter => (
            "right: anchor(left); top: anchor(top); margin-right: 8px; @position-try(flip-block) { top: anchor(bottom); margin-top: 8px; }".to_string(),
            "right top".to_string(),
        ),
        PopoverAlign::End => (
            "right: anchor(right); bottom: anchor(top); margin-bottom: 8px; @position-try(flip-block) { top: anchor(bottom); bottom: auto; margin-top: 8px; margin-bottom: 0; }".to_string(),
            "right top".to_string(),
        ),
        PopoverAlign::EndOuter => (
            "left: anchor(right); top: anchor(top); margin-left: 8px; @position-try(flip-block) { top: anchor(bottom); margin-top: 8px; }".to_string(),
            "left top".to_string(),
        ),
        PopoverAlign::Center => ("position-area: block-start;".to_string(), "center top".to_string()),
    };

    let css = format!(
        "#{content_id} {{ position: fixed; position-anchor: {anchor_name}; {position_styles} transform-origin: {transform_origin}; }}"
    );

    let tid = trigger_id.clone();
    let cid = content_id.clone();
    let script = format!(
        r#"(function() {{
        const setup = () => {{
            const trigger = document.getElementById('{tid}');
            const content = document.getElementById('{cid}');
            if (!trigger || !content) {{ setTimeout(setup, 50); return; }}
            if (trigger.hasAttribute('data-popover-init')) return;
            trigger.setAttribute('data-popover-init', 'true');
            const close = () => content.setAttribute('data-state', 'closed');
            trigger.addEventListener('click', e => {{
                e.stopPropagation();
                const isOpen = content.getAttribute('data-state') === 'open';
                content.setAttribute('data-state', isOpen ? 'closed' : 'open');
            }});
            document.addEventListener('click', e => {{
                if (!content.contains(e.target) && e.target !== trigger) close();
            }});
            document.addEventListener('keydown', e => {{ if (e.key === 'Escape') close(); }});
        }};
        if (document.readyState === 'loading') {{ document.addEventListener('DOMContentLoaded', setup); }} else {{ setup(); }}
    }})();"#
    );

    rsx! {
        style { dangerous_inner_html: "{css}" }
        {children}
        script { dangerous_inner_html: "{script}" }
    }
}

#[component]
pub fn PopoverTrigger(
    #[props(into, optional)] class: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_context::<PopoverContext>();
    rsx! {
        button {
            "data-name": "PopoverTrigger",
            id: "{ctx.trigger_id}",
            class: tw_merge!(
                "px-4 py-2 h-9 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2  border bg-background border-input hover:bg-accent hover:text-accent-foreground",
                class.as_deref().unwrap_or("")
            ),
            disabled,
            "aria-label": aria_label.as_deref(),
            style: "anchor-name: {ctx.anchor_name};",
            {children}
        }
    }
}

#[component]
pub fn PopoverContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<PopoverContext>();
    // Visual base copied verbatim from leptos PopoverContent. The trailing
    // pointer-events / opacity / data-[state=open] classes are dioxus's JS-state
    // equivalent of leptos's `&:popover-open` CSS (dioxus toggles data-state
    // instead of using the native popover attribute).
    let c = tw_merge!(
        "overflow-visible relative z-50 p-4 rounded-md border bg-card shadow-md my-[1ch] w-[250px] min-h-[150px] pointer-events-none opacity-0 transition-all duration-150 data-[state=open]:opacity-100 data-[state=open]:pointer-events-auto",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": "PopoverContent",
            id: "{ctx.content_id}",
            class: "{c}",
            "data-state": "closed",
            role: "dialog",
            {children}
        }
    }
}

#[component]
pub fn PopoverTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("leading-none font-medium mb-3", class.as_deref().unwrap_or(""));
    rsx! { h3 { "data-name": "PopoverTitle", class: "{merged}", {children} } }
}

#[component]
pub fn PopoverDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-muted-foreground text-sm", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "PopoverDescription", class: "{merged}", {children} } }
}
