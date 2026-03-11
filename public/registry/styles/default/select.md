---
title: "Select"
name: "select"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/select.rs"
description: "Rust/UI component that displays a dropdown menu that allows the user to select an option."
tags: ["select"]
---

# Select

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add select
```

## Component Code

```rust
use icons::{Check, ChevronDown, ChevronUp};
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use strum::{AsRefStr, Display};
use tw_merge::*;

use crate::components::hooks::use_can_scroll_vertical::use_can_scroll_vertical;
use crate::components::hooks::use_random::use_random_id_for;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, AsRefStr)]
pub enum SelectPosition {
    #[default]
    Below,
    Above,
}

mod components {
    use super::*;
    clx! {SelectLabel, span, "px-2 py-1.5 text-sm font-medium data-inset:pl-8", "mb-1"}
    clx! {SelectItem, li, "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4"}
}

pub use components::*;

#[component]
pub fn SelectGroup(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = "Select options".into(), into)] aria_label: String,
) -> impl IntoView {
    let merged_class = tw_merge!("group", class);

    view! {
        <ul data-name="SelectGroup" role="listbox" aria-label=aria_label class=merged_class>
            {children()}
        </ul>
    }
}

#[component]
pub fn SelectValue(#[prop(optional, into)] placeholder: String) -> impl IntoView {
    let select_ctx = expect_context::<SelectContext>();

    view! {
        <span data-name="SelectValue" class="text-sm text-muted-foreground truncate">
            {move || { select_ctx.value_signal.get().unwrap_or_else(|| placeholder.clone()) }}
        </span>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn SelectOption(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = false.into(), into)] aria_selected: Signal<bool>,
    #[prop(optional, into)] value: Option<String>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();

    let merged_class = tw_merge!(
        "group inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm cursor-pointer no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class
    );

    let value_for_check = value.clone();
    let is_selected = move || aria_selected.get() || ctx.value_signal.get() == value_for_check;

    view! {
        <li
            data-name="SelectOption"
            class=merged_class
            role="option"
            tabindex="0"
            aria-selected=move || is_selected().to_string()
            data-select-option="true"
            on:click=move |_| {
                let val = value.clone();
                ctx.value_signal.set(val.clone());
                if let Some(on_change) = ctx.on_change {
                    on_change.run(val);
                }
            }
        >
            {children()}
            <Check class="ml-auto opacity-0 size-4 text-muted-foreground group-aria-selected:opacity-100" />
        </li>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone)]
struct SelectContext {
    target_id: String,
    value_signal: RwSignal<Option<String>>,
    on_change: Option<Callback<Option<String>>>,
}

#[component]
pub fn Select(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional)] on_change: Option<Callback<Option<String>>>,
) -> impl IntoView {
    let select_target_id = use_random_id_for("select");
    let value_signal = RwSignal::new(default_value);

    let ctx = SelectContext { target_id: select_target_id, value_signal, on_change };

    let merged_class = tw_merge!("relative w-fit", class);

    view! {
        <Provider value=ctx>
            <div data-name="Select" class=merged_class>
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn SelectTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] id: String,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();

    let peer_class = if !id.is_empty() { format!("peer/{}", id) } else { String::new() };

    let button_class = tw_merge!(
        "w-full p-2 h-9 inline-flex items-center justify-between text-sm font-medium whitespace-nowrap rounded-md transition-colors focus:outline-none focus:ring-1 focus:ring-ring focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4 border bg-background border-input hover:bg-accent hover:text-accent-foreground",
        &peer_class,
        class
    );

    let button_id = if !id.is_empty() { id } else { format!("trigger_{}", ctx.target_id) };

    view! {
        <button
            type="button"
            data-name="SelectTrigger"
            class=button_class
            id=button_id
            tabindex="0"
            data-select-trigger=ctx.target_id
        >
            {children()}
            <ChevronDown class="text-muted-foreground" />
        </button>
    }
}

#[component]
pub fn SelectContent(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = SelectPosition::default())] position: SelectPosition,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();

    let merged_class = tw_merge!(
        "w-[150px] overflow-auto z-50 p-1 rounded-md border bg-card shadow-md h-fit max-h-[300px] absolute top-[calc(100%+4px)] left-0 data-[position=Above]:top-auto data-[position=Above]:bottom-[calc(100%+4px)] transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100 data-[state=closed]:data-[position=Below]:origin-top data-[state=open]:data-[position=Below]:origin-top data-[state=closed]:data-[position=Above]:origin-bottom data-[state=open]:data-[position=Above]:origin-bottom [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
        class
    );

    let target_id_for_script = ctx.target_id.clone();

    // Scroll indicator signals
    let (on_scroll, can_scroll_up_signal, can_scroll_down_signal) = use_can_scroll_vertical();

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name="SelectContent"
            class=merged_class
            id=ctx.target_id
            data-target="target__select"
            data-state="closed"
            data-position=position.to_string()
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
                    const setupSelect = () => {{
                        const select = document.querySelector('#{}');
                        const trigger = document.querySelector('[data-select-trigger="{}"]');

                        if (!select || !trigger) {{
                            setTimeout(setupSelect, 50);
                            return;
                        }}

                        if (select.hasAttribute('data-initialized')) {{
                            return;
                        }}
                        select.setAttribute('data-initialized', 'true');

                        let isOpen = false;

                        const updatePosition = () => {{
                            const triggerRect = trigger.getBoundingClientRect();
                            const viewportHeight = window.innerHeight;
                            const spaceBelow = viewportHeight - triggerRect.bottom;
                            const spaceAbove = triggerRect.top;

                            // Determine if dropdown should go above or below
                            if (spaceBelow < 200 && spaceAbove > spaceBelow) {{
                                select.setAttribute('data-position', 'Above');
                            }} else {{
                                select.setAttribute('data-position', 'Below');
                            }}

                            // Set min-width to match trigger
                            select.style.minWidth = `${{triggerRect.width}}px`;
                        }};

                        const openSelect = () => {{
                            isOpen = true;

                            // Lock scrolling
                            window.ScrollLock.lock();

                            // Update position and open
                            updatePosition();
                            select.setAttribute('data-state', 'open');
                            select.style.pointerEvents = 'auto';

                            // Trigger scroll event to update indicators
                            select.dispatchEvent(new Event('scroll'));

                            // Close on click outside
                            setTimeout(() => {{
                                document.addEventListener('click', handleClickOutside);
                            }}, 0);
                        }};

                        const closeSelect = () => {{
                            isOpen = false;
                            select.setAttribute('data-state', 'closed');
                            select.style.pointerEvents = 'none';
                            document.removeEventListener('click', handleClickOutside);

                            // Unlock scrolling after animation
                            window.ScrollLock.unlock(200);
                        }};

                        const handleClickOutside = (e) => {{
                            if (!select.contains(e.target) && !trigger.contains(e.target)) {{
                                closeSelect();
                            }}
                        }};

                        // Toggle select when trigger is clicked
                        trigger.addEventListener('click', (e) => {{
                            e.stopPropagation();
                            if (isOpen) {{
                                closeSelect();
                            }} else {{
                                openSelect();
                            }}
                        }});

                        // Close when option is selected
                        const options = select.querySelectorAll('[data-select-option]');
                        options.forEach(option => {{
                            option.addEventListener('click', () => {{
                                closeSelect();
                            }});
                        }});

                        // Handle ESC key to close
                        document.addEventListener('keydown', (e) => {{
                            if (e.key === 'Escape' && isOpen) {{
                                e.preventDefault();
                                closeSelect();
                            }}
                        }});
                    }};

                    if (document.readyState === 'loading') {{
                        document.addEventListener('DOMContentLoaded', setupSelect);
                    }} else {{
                        setupSelect();
                    }}
                }})();
                "#,
                target_id_for_script,
                target_id_for_script,
            )}
        </script>
    }
}
```
