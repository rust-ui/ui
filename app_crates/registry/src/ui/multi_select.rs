use std::collections::HashSet;

use dioxus::prelude::*;
use icons::{Check, ChevronDown, ChevronUp};
use tw_merge::tw_merge;

use crate::hooks::use_can_scroll_vertical::use_can_scroll_vertical;
use crate::hooks::use_random::use_random_id_for;

/* ========================================================== */
/*                     ✨ TYPES ✨                            */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiSelectAlign {
    Start,
    #[default]
    Center,
    End,
}

/* ========================================================== */
/*                     ✨ CONTEXT ✨                          */
/* ========================================================== */

#[derive(Clone)]
struct MultiSelectContext {
    target_id: String,
    values_signal: Signal<HashSet<String>>,
    align: MultiSelectAlign,
}

/* ========================================================== */
/*                     ✨ LABEL ✨                            */
/* ========================================================== */

#[component]
pub fn MultiSelectLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    // Mirrors leptos `pub use select::SelectLabel as MultiSelectLabel` (span, same class string).
    let merged = tw_merge!("px-2 py-1.5 text-sm font-medium data-inset:pl-8 mb-1", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "MultiSelectLabel", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ GROUP ✨                            */
/* ========================================================== */

#[component]
pub fn MultiSelectGroup(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
) -> Element {
    let merged = tw_merge!("group", class.as_deref().unwrap_or(""));
    rsx! {
        ul {
            "data-name": "MultiSelectGroup",
            role: "listbox",
            "aria-label": aria_label.as_deref().unwrap_or("Multi select options"),
            class: "{merged}",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ ITEM ✨                             */
/* ========================================================== */

#[component]
pub fn MultiSelectItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        li {
            "data-name": "MultiSelectItem",
            class: "{merged}",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ VALUE ✨                            */
/* ========================================================== */

#[component]
pub fn MultiSelectValue(#[props(into, optional)] placeholder: Option<String>) -> Element {
    let multi_select_ctx = use_context::<MultiSelectContext>();

    rsx! {
        span { "data-name": "MultiSelectValue", class: "text-sm text-muted-foreground truncate",
            {
                let values = (multi_select_ctx.values_signal)();
                if values.is_empty() {
                    placeholder.clone().unwrap_or_default()
                } else {
                    let count = values.len();
                    if count == 1 { "1 selected".to_string() } else { format!("{} selected", count) }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ OPTION ✨                           */
/* ========================================================== */

#[component]
pub fn MultiSelectOption(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] value: Option<String>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let multi_select_ctx = use_context::<MultiSelectContext>();

    let value_clone = value.clone();
    let is_selected = use_memo(move || {
        if let Some(ref val) = value_clone { (multi_select_ctx.values_signal)().contains(val) } else { false }
    });

    let merged = tw_merge!(
        "group inline-flex gap-2 items-center w-full text-sm text-left transition-colors duration-200 focus:outline-none focus-visible:outline-none text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            r#type: "button",
            "data-name": "MultiSelectOption",
            class: "{merged}",
            role: "option",
            disabled,
            "aria-selected": if is_selected() { "true" } else { "false" },
            onclick: move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(val) = value.clone() {
                    let mut values_signal = multi_select_ctx.values_signal;
                    values_signal.with_mut(|values| {
                        if values.contains(&val) {
                            values.remove(&val);
                        } else {
                            values.insert(val);
                        }
                    });
                }
            },
            {children}
            Check { class: "ml-auto opacity-0 size-4 text-muted-foreground group-aria-selected:opacity-100" }
        }
    }
}

/* ========================================================== */
/*                     ✨ MULTI SELECT ✨                     */
/* ========================================================== */

#[component]
pub fn MultiSelect(
    children: Element,
    #[props(optional)] values: Option<Signal<HashSet<String>>>,
    #[props(default = MultiSelectAlign::default())] align: MultiSelectAlign,
) -> Element {
    let multi_select_target_id = use_random_id_for("multi_select");
    let values_signal = values.unwrap_or_else(|| use_signal(HashSet::new));

    let multi_select_ctx = MultiSelectContext { target_id: multi_select_target_id, values_signal, align };
    provide_context(multi_select_ctx);

    rsx! {
        div { "data-name": "MultiSelect", class: "relative w-fit",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ TRIGGER ✨                          */
/* ========================================================== */

#[component]
pub fn MultiSelectTrigger(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] id: Option<String>,
) -> Element {
    let multi_select_ctx = use_context::<MultiSelectContext>();

    let id_str = id.clone().unwrap_or_default();
    let peer_class = if !id_str.is_empty() { format!("peer/{}", id_str) } else { String::new() };

    let button_class = tw_merge!(
        "w-full p-2 h-9 inline-flex items-center justify-between text-sm font-medium whitespace-nowrap rounded-md transition-colors focus:outline-none focus:ring-1 focus:ring-ring focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4  border bg-background border-input hover:bg-accent hover:text-accent-foreground",
        &peer_class,
        class.as_deref().unwrap_or("")
    );

    let button_id = if !id_str.is_empty() { id_str } else { format!("trigger_{}", multi_select_ctx.target_id) };

    rsx! {
        button {
            r#type: "button",
            "data-name": "MultiSelectTrigger",
            class: "{button_class}",
            id: "{button_id}",
            tabindex: "0",
            "data-multi-select-trigger": "{multi_select_ctx.target_id}",
            {children}
            ChevronDown { class: "text-muted-foreground" }
        }
    }
}

/* ========================================================== */
/*                     ✨ CONTENT ✨                          */
/* ========================================================== */

#[component]
pub fn MultiSelectContent(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let multi_select_ctx = use_context::<MultiSelectContext>();

    let align_str = match multi_select_ctx.align {
        MultiSelectAlign::Start => "start",
        MultiSelectAlign::Center => "center",
        MultiSelectAlign::End => "end",
    };

    let merged = tw_merge!(
        "w-[150px] overflow-auto z-50 p-1 rounded-md border bg-card shadow-md h-fit max-h-[300px] absolute top-[calc(100%+4px)] transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100 data-[align=start]:left-0 data-[align=center]:left-1/2 data-[align=center]:-translate-x-1/2 data-[align=end]:right-0 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
        class.as_deref().unwrap_or("")
    );

    let target_id_for_script = multi_select_ctx.target_id.clone();
    let (on_scroll, can_scroll_up, can_scroll_down) = use_can_scroll_vertical();

    let script = format!(
        r#"
        (function() {{
            const setupMultiSelect = () => {{
                const multiSelect = document.querySelector('#{target_id_for_script}');
                const trigger = document.querySelector('[data-multi-select-trigger="{target_id_for_script}"]');

                if (!multiSelect || !trigger) {{
                    setTimeout(setupMultiSelect, 50);
                    return;
                }}

                if (multiSelect.hasAttribute('data-initialized')) {{
                    return;
                }}
                multiSelect.setAttribute('data-initialized', 'true');

                let isOpen = false;

                const openMultiSelect = () => {{
                    isOpen = true;

                    if (window.ScrollLock) {{
                        window.ScrollLock.lock();
                    }}

                    multiSelect.setAttribute('data-state', 'open');
                    multiSelect.style.pointerEvents = 'auto';

                    const triggerRect = trigger.getBoundingClientRect();
                    multiSelect.style.minWidth = `${{triggerRect.width}}px`;

                    multiSelect.dispatchEvent(new Event('scroll'));

                    setTimeout(() => {{
                        document.addEventListener('click', handleClickOutside);
                    }}, 0);
                }};

                const closeMultiSelect = () => {{
                    isOpen = false;
                    multiSelect.setAttribute('data-state', 'closed');
                    multiSelect.style.pointerEvents = 'none';
                    document.removeEventListener('click', handleClickOutside);

                    if (window.ScrollLock) {{
                        window.ScrollLock.unlock(200);
                    }}
                }};

                const handleClickOutside = (e) => {{
                    if (!multiSelect.contains(e.target) && !trigger.contains(e.target)) {{
                        closeMultiSelect();
                    }}
                }};

                trigger.addEventListener('click', (e) => {{
                    e.stopPropagation();
                    if (isOpen) {{
                        closeMultiSelect();
                    }} else {{
                        openMultiSelect();
                    }}
                }});

                document.addEventListener('keydown', (e) => {{
                    if (e.key === 'Escape' && isOpen) {{
                        e.preventDefault();
                        closeMultiSelect();
                    }}
                }});
            }};

            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', setupMultiSelect);
            }} else {{
                setupMultiSelect();
            }}
        }})();
        "#
    );

    rsx! {
        div {
            "data-name": "MultiSelectContent",
            class: "{merged}",
            id: "{multi_select_ctx.target_id}",
            "data-target": "target__multi_select",
            "data-state": "closed",
            "data-align": "{align_str}",
            style: "pointer-events: none;",
            onscroll: on_scroll,
            div {
                class: if can_scroll_up() { "sticky -top-1 z-10 flex items-center justify-center py-1 bg-card" } else { "hidden" },
                ChevronUp { class: "size-4 text-muted-foreground" }
            }
            {children}
            div {
                class: if can_scroll_down() { "sticky -bottom-1 z-10 flex items-center justify-center py-1 bg-card" } else { "hidden" },
                ChevronDown { class: "size-4 text-muted-foreground" }
            }
        }
        script { dangerous_inner_html: "{script}" }
    }
}
