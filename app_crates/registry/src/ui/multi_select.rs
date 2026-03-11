use std::collections::HashSet;

use icons::{Check, ChevronDown, ChevronUp};
use leptos::context::Provider;
use leptos::prelude::*;
use tw_merge::*;

use crate::hooks::use_can_scroll_vertical::use_can_scroll_vertical;
use crate::hooks::use_random::use_random_id_for;
// * Reuse @select.rs
pub use crate::ui::select::{
    SelectGroup as MultiSelectGroup, SelectItem as MultiSelectItem, SelectLabel as MultiSelectLabel,
};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiSelectAlign {
    Start,
    #[default]
    Center,
    End,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiSelectPosition {
    #[default]
    Below,
    Above,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn MultiSelectValue(#[prop(optional, into)] placeholder: String) -> impl IntoView {
    let multi_select_ctx = expect_context::<MultiSelectContext>();

    view! {
        <span data-name="MultiSelectValue" class="text-sm text-muted-foreground truncate">
            {move || {
                let values = multi_select_ctx.values_signal.get();
                if values.is_empty() {
                    placeholder.clone()
                } else {
                    let count = values.len();
                    if count == 1 { "1 selected".to_string() } else { format!("{} selected", count) }
                }
            }}
        </span>
    }
}

#[component]
pub fn MultiSelectOption(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] value: Option<String>,
) -> impl IntoView {
    let multi_select_ctx = expect_context::<MultiSelectContext>();

    let value_clone = value.clone();
    let is_selected = Signal::derive(move || {
        if let Some(ref val) = value_clone {
            multi_select_ctx.values_signal.with(|values| values.contains(val))
        } else {
            false
        }
    });

    let class = tw_merge!(
        "group inline-flex gap-2 items-center w-full text-sm text-left transition-colors duration-200 focus:outline-none focus-visible:outline-none text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground disabled:cursor-not-allowed disabled:opacity-50",
        class
    );

    view! {
        <button
            type="button"
            data-name="MultiSelectOption"
            class=class
            role="option"
            aria-selected=move || is_selected.get().to_string()
            on:click=move |ev: web_sys::MouseEvent| {
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(val) = value.clone() {
                    multi_select_ctx
                        .values_signal
                        .update(|values| {
                            if values.contains(&val) {
                                values.remove(&val);
                            } else {
                                values.insert(val);
                            }
                        });
                }
            }
        >
            {children()}
            <Check class="ml-auto opacity-0 size-4 text-muted-foreground group-aria-selected:opacity-100" />
        </button>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone)]
struct MultiSelectContext {
    target_id: String,
    values_signal: RwSignal<HashSet<String>>,
    align: MultiSelectAlign,
}

#[component]
pub fn MultiSelect(
    children: Children,
    #[prop(optional, into)] values: Option<RwSignal<HashSet<String>>>,
    #[prop(default = MultiSelectAlign::default())] align: MultiSelectAlign,
) -> impl IntoView {
    let multi_select_target_id = use_random_id_for("multi_select");
    let values_signal = values.unwrap_or_else(|| RwSignal::new(HashSet::<String>::new()));

    let multi_select_ctx = MultiSelectContext { target_id: multi_select_target_id, values_signal, align };

    view! {
        <Provider value=multi_select_ctx>
            <div data-name="MultiSelect" class="relative w-fit">
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn MultiSelectTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] id: String,
) -> impl IntoView {
    let multi_select_ctx = expect_context::<MultiSelectContext>();

    let peer_class = if !id.is_empty() { format!("peer/{}", id) } else { String::new() };

    let button_class = tw_merge!(
        "w-full p-2 h-9 inline-flex items-center justify-between text-sm font-medium whitespace-nowrap rounded-md transition-colors focus:outline-none focus:ring-1 focus:ring-ring focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4  border bg-background border-input hover:bg-accent hover:text-accent-foreground",
        &peer_class,
        class
    );

    let button_id = if !id.is_empty() { id } else { format!("trigger_{}", multi_select_ctx.target_id) };

    view! {
        <button
            type="button"
            data-name="MultiSelectTrigger"
            class=button_class
            id=button_id
            tabindex="0"
            data-multi-select-trigger=multi_select_ctx.target_id
        >
            {children()}
            <ChevronDown class="text-muted-foreground" />
        </button>
    }
}

#[component]
pub fn MultiSelectContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let multi_select_ctx = expect_context::<MultiSelectContext>();

    let align_str = match multi_select_ctx.align {
        MultiSelectAlign::Start => "start",
        MultiSelectAlign::Center => "center",
        MultiSelectAlign::End => "end",
    };

    let class = tw_merge!(
        "w-[150px] overflow-auto z-50 p-1 rounded-md border bg-card shadow-md h-fit max-h-[300px] absolute top-[calc(100%+4px)] transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100 data-[align=start]:left-0 data-[align=center]:left-1/2 data-[align=center]:-translate-x-1/2 data-[align=end]:right-0 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
        class
    );

    let target_id_for_script = multi_select_ctx.target_id.clone();

    // Scroll indicator signals
    let (on_scroll, can_scroll_up_signal, can_scroll_down_signal) = use_can_scroll_vertical();

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name="MultiSelectContent"
            class=class
            id=multi_select_ctx.target_id
            data-target="target__multi_select"
            data-state="closed"
            data-align=align_str
            style="pointer-events: none;"
            on:scroll=on_scroll
        >
            <div
                data-scroll-up="true"
                class=move || {
                    if can_scroll_up_signal.get() {
                        "sticky -top-1 z-10 flex items-center justify-center py-1 bg-card"
                    } else {
                        "hidden"
                    }
                }
            >
                <ChevronUp class="size-4 text-muted-foreground" />
            </div>
            {children()}
            <div
                data-scroll-down="true"
                class=move || {
                    if can_scroll_down_signal.get() {
                        "sticky -bottom-1 z-10 flex items-center justify-center py-1 bg-card"
                    } else {
                        "hidden"
                    }
                }
            >
                <ChevronDown class="size-4 text-muted-foreground" />
            </div>
        </div>

        <script>
            {format!(
                r#"
                (function() {{
                    const setupMultiSelect = () => {{
                        const multiSelect = document.querySelector('#{}');
                        const trigger = document.querySelector('[data-multi-select-trigger="{}"]');

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

                            // Lock all scrollable elements
                            window.ScrollLock.lock();

                            multiSelect.setAttribute('data-state', 'open');
                            multiSelect.style.pointerEvents = 'auto';

                            // Set min-width to match trigger
                            const triggerRect = trigger.getBoundingClientRect();
                            multiSelect.style.minWidth = `${{triggerRect.width}}px`;

                            // Trigger scroll event to update indicators
                            multiSelect.dispatchEvent(new Event('scroll'));

                            // Close on click outside
                            setTimeout(() => {{
                                document.addEventListener('click', handleClickOutside);
                            }}, 0);
                        }};

                        const closeMultiSelect = () => {{
                            isOpen = false;
                            multiSelect.setAttribute('data-state', 'closed');
                            multiSelect.style.pointerEvents = 'none';
                            document.removeEventListener('click', handleClickOutside);

                            // Unlock scroll after animation (200ms delay)
                            window.ScrollLock.unlock(200);
                        }};

                        const handleClickOutside = (e) => {{
                            if (!multiSelect.contains(e.target) && !trigger.contains(e.target)) {{
                                closeMultiSelect();
                            }}
                        }};

                        // Toggle multi-select when trigger is clicked
                        trigger.addEventListener('click', (e) => {{
                            e.stopPropagation();
                            if (isOpen) {{
                                closeMultiSelect();
                            }} else {{
                                openMultiSelect();
                            }}
                        }});

                        // Handle ESC key to close
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
                "#,
                target_id_for_script,
                target_id_for_script,
            )}
        </script>
    }
}
