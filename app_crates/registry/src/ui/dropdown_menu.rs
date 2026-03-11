use icons::{Check, ChevronRight};
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::hooks::use_random::use_random_id_for;
pub use crate::ui::separator::Separator as DropdownMenuSeparator;

mod components {
    use super::*;
    clx! {DropdownMenuLabel, span, "px-2 py-1.5 text-sm font-medium data-inset:pl-8", "mb-1"}
    clx! {DropdownMenuGroup, ul, "group"}
    clx! {DropdownMenuItem, li, "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4"}
    clx! {DropdownMenuSubContent, ul, "dropdown__menu_sub_content", "rounded-md border bg-card shadow-lg p-1 absolute z-[100] min-w-[160px] opacity-0 invisible translate-x-[-8px] transition-all duration-200 ease-out pointer-events-none"}
    clx! {DropdownMenuLink, a, "w-full inline-flex gap-2 items-center"}
}

pub use components::*;

/* ========================================================== */
/*                     RADIO GROUP                            */
/* ========================================================== */

#[derive(Clone)]
struct DropdownMenuRadioContext<T: Clone + PartialEq + Send + Sync + 'static> {
    value_signal: RwSignal<T>,
}

/// A group of radio items where only one can be selected at a time.
#[component]
pub fn DropdownMenuRadioGroup<T>(
    children: Children,
    /// The signal holding the current selected value
    value: RwSignal<T>,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let ctx = DropdownMenuRadioContext { value_signal: value };

    view! {
        <Provider value=ctx>
            <ul data-name="DropdownMenuRadioGroup" role="group" class="group">
                {children()}
            </ul>
        </Provider>
    }
}

/// A radio item that shows a checkmark when selected.
#[component]
pub fn DropdownMenuRadioItem<T>(
    children: Children,
    /// The value this item represents
    value: T,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let ctx = expect_context::<DropdownMenuRadioContext<T>>();

    let value_for_check = value.clone();
    let value_for_click = value;
    let is_selected = move || ctx.value_signal.get() == value_for_check;

    let merged_class = tw_merge!(
        "group inline-flex gap-2 items-center w-full rounded-sm pl-2 pr-2 py-1.5 text-sm cursor-pointer no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class
    );

    view! {
        <li
            data-name="DropdownMenuRadioItem"
            class=merged_class
            role="menuitemradio"
            aria-checked=move || is_selected().to_string()
            data-dropdown-close="true"
            on:click=move |_| {
                ctx.value_signal.set(value_for_click.clone());
            }
        >
            {children()}
            <Check class="ml-auto opacity-0 size-4 text-muted-foreground group-aria-checked:opacity-100" />
        </li>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuActionVariant {
    #[default]
    Default,
    Destructive,
}

/// An action item in a dropdown menu (no checkmark, just triggers an action).
#[component]
pub fn DropdownMenuAction(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = DropdownMenuActionVariant::default())] variant: DropdownMenuActionVariant,
) -> impl IntoView {
    let _ctx = expect_context::<DropdownMenuContext>();

    let variant_class = match variant {
        DropdownMenuActionVariant::Default => "text-popover-foreground hover:bg-accent hover:text-accent-foreground",
        DropdownMenuActionVariant::Destructive => "text-destructive hover:bg-destructive/10 hover:text-destructive",
    };

    let class = tw_merge!(
        "inline-flex gap-2 items-center w-full text-sm text-left transition-colors duration-200 focus:outline-none focus-visible:outline-none [&_svg:not([class*='size-'])]:size-4",
        variant_class,
        class
    );

    if let Some(href) = href {
        // Render as <a> tag when href is provided
        view! {
            <a data-name="DropdownMenuAction" class=class href=href data-dropdown-close="true">
                {children()}
            </a>

            <script>
                {r#"
                (function() {
                const link = document.currentScript.previousElementSibling;
                if (!link) return;
                
                link.addEventListener('click', function() {
                // Close dropdown on route change after navigation
                let currentPath = window.location.pathname;
                const checkRouteChange = () => {
                if (window.location.pathname !== currentPath) {
                currentPath = window.location.pathname;
                
                // Find and close the dropdown
                const dropdown = link.closest('[data-target="target__dropdown"]');
                if (dropdown) {
                dropdown.setAttribute('data-state', 'closed');
                dropdown.style.pointerEvents = 'none';
                
                // Unlock scroll
                if (window.ScrollLock) {
                window.ScrollLock.unlock(200);
                }
                }
                
                clearInterval(routeCheckInterval);
                }
                };
                
                const routeCheckInterval = setInterval(checkRouteChange, 50);
                
                // Clear interval after 2 seconds to prevent memory leaks
                setTimeout(() => clearInterval(routeCheckInterval), 2000);
                });
                })();
                "#}
            </script>
        }
        .into_any()
    } else {
        // Render as <button> tag when no href
        view! {
            <button type="button" data-name="DropdownMenuAction" class=class data-dropdown-close="true">
                {children()}
            </button>
        }
        .into_any()
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuAlign {
    #[default]
    Start,
    StartOuter,
    End,
    EndOuter,
    Center,
}

#[derive(Clone)]
struct DropdownMenuContext {
    target_id: String,
    align: DropdownMenuAlign,
}

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(default = DropdownMenuAlign::default())] align: DropdownMenuAlign,
) -> impl IntoView {
    let dropdown_target_id = use_random_id_for("dropdown");

    let ctx = DropdownMenuContext { target_id: dropdown_target_id, align };

    view! {
        <Provider value=ctx>
            <style>
                "
                /* Submenu Styles */
                .dropdown__menu_sub_content {
                    position: absolute;
                    inset-inline-start: calc(100% + 8px);
                    inset-block-start: -4px;
                    z-index: 100;
                    min-inline-size: 160px;
                    opacity: 0;
                    visibility: hidden;
                    transform: translateX(-8px);
                    transition: all 0.2s ease-out;
                    pointer-events: none;
                }
                
                .dropdown__menu_sub_trigger:hover .dropdown__menu_sub_content {
                    opacity: 1;
                    visibility: visible;
                    transform: translateX(0);
                    pointer-events: auto;
                }
                "
            </style>

            <div data-name="DropdownMenu">{children()}</div>
        </Provider>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    /// Render children directly instead of wrapping in a button.
    /// Use when the child is already a button (e.g. InputGroupButton) to avoid nested buttons.
    #[prop(optional)]
    as_child: bool,
) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();

    if as_child {
        return view! {
            <span data-name="DropdownMenuTrigger" data-dropdown-trigger=ctx.target_id class="contents">
                {children()}
            </span>
        }
        .into_any();
    }

    let button_class = tw_merge!(
        "px-4 py-2 h-9 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus:outline-none focus:ring-1 focus:ring-ring focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4  border bg-background border-input hover:bg-accent hover:text-accent-foreground",
        class
    );

    view! {
        <button
            type="button"
            class=button_class
            data-name="DropdownMenuTrigger"
            data-dropdown-trigger=ctx.target_id
            tabindex="0"
        >
            {children()}
        </button>
    }
    .into_any()
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuPosition {
    #[default]
    Auto,
    Top,
    Bottom,
}

#[component]
pub fn DropdownMenuContent(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = DropdownMenuPosition::default())] position: DropdownMenuPosition,
) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();

    let base_classes = "z-50 p-1 rounded-md border bg-card shadow-md h-fit fixed transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100";
    let width_class = match ctx.align {
        DropdownMenuAlign::Center => "min-w-full",
        _ => "w-[180px]",
    };

    let class = tw_merge!(width_class, base_classes, class);

    let target_id_for_script = ctx.target_id.clone();
    let align_for_script = match ctx.align {
        DropdownMenuAlign::Start => "start",
        DropdownMenuAlign::StartOuter => "start-outer",
        DropdownMenuAlign::End => "end",
        DropdownMenuAlign::EndOuter => "end-outer",
        DropdownMenuAlign::Center => "center",
    };

    let position_for_script = match position {
        DropdownMenuPosition::Auto => "auto",
        DropdownMenuPosition::Top => "top",
        DropdownMenuPosition::Bottom => "bottom",
    };

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name="DropdownMenuContent"
            class=class
            id=ctx.target_id
            data-target="target__dropdown"
            data-state="closed"
            data-align=align_for_script
            data-position=position_for_script
            style="pointer-events: none;"
        >
            {children()}
        </div>

        <script>
            {format!(
                r#"
                (function() {{
                    const setupDropdown = () => {{
                        const dropdown = document.querySelector('#{}');
                        const trigger = document.querySelector('[data-dropdown-trigger="{}"]');

                        if (!dropdown || !trigger) {{
                            setTimeout(setupDropdown, 50);
                            return;
                        }}

                        if (dropdown.hasAttribute('data-initialized')) {{
                            return;
                        }}
                        dropdown.setAttribute('data-initialized', 'true');

                        let isOpen = false;

                        const updatePosition = () => {{
                            const triggerEl = getComputedStyle(trigger).display === 'contents' ? trigger.firstElementChild : trigger;
                            const triggerRect = triggerEl.getBoundingClientRect();
                            const dropdownRect = dropdown.getBoundingClientRect();
                            const viewportHeight = window.innerHeight;
                            const viewportWidth = window.innerWidth;
                            const spaceBelow = viewportHeight - triggerRect.bottom;
                            const spaceAbove = triggerRect.top;

                            const align = dropdown.getAttribute('data-align') || 'start';
                            const position = dropdown.getAttribute('data-position') || 'auto';

                            // Determine if we should position above
                            let shouldPositionAbove = false;
                            if (position === 'top') {{
                                shouldPositionAbove = true;
                            }} else if (position === 'bottom') {{
                                shouldPositionAbove = false;
                            }} else {{
                                // Auto: position above if there's space above AND not enough space below
                                shouldPositionAbove = spaceAbove >= dropdownRect.height && spaceBelow < dropdownRect.height;
                            }}

                            switch (align) {{
                                case 'start':
                                    if (shouldPositionAbove) {{
                                        dropdown.style.top = `${{triggerRect.top - dropdownRect.height - 6}}px`;
                                        dropdown.style.transformOrigin = 'left bottom';
                                    }} else {{
                                        dropdown.style.top = `${{triggerRect.bottom + 6}}px`;
                                        dropdown.style.transformOrigin = 'left top';
                                    }}
                                    dropdown.style.left = `${{triggerRect.left}}px`;
                                    break;

                                case 'end':
                                    if (shouldPositionAbove) {{
                                        dropdown.style.top = `${{triggerRect.top - dropdownRect.height - 6}}px`;
                                        dropdown.style.transformOrigin = 'right bottom';
                                    }} else {{
                                        dropdown.style.top = `${{triggerRect.bottom + 6}}px`;
                                        dropdown.style.transformOrigin = 'right top';
                                    }}
                                    dropdown.style.left = `${{triggerRect.right - dropdownRect.width}}px`;
                                    break;

                                case 'start-outer':
                                    if (shouldPositionAbove) {{
                                        dropdown.style.top = `${{triggerRect.top - dropdownRect.height - 6}}px`;
                                        dropdown.style.transformOrigin = 'right bottom';
                                    }} else {{
                                        dropdown.style.top = `${{triggerRect.top}}px`;
                                        dropdown.style.transformOrigin = 'right top';
                                    }}
                                    dropdown.style.left = `${{triggerRect.left - dropdownRect.width - 16}}px`;
                                    break;

                                case 'end-outer':
                                    if (shouldPositionAbove) {{
                                        dropdown.style.top = `${{triggerRect.top - dropdownRect.height - 6}}px`;
                                        dropdown.style.transformOrigin = 'left bottom';
                                    }} else {{
                                        dropdown.style.top = `${{triggerRect.top}}px`;
                                        dropdown.style.transformOrigin = 'left top';
                                    }}
                                    dropdown.style.left = `${{triggerRect.right + 8}}px`;
                                    break;

                                case 'center':
                                    if (shouldPositionAbove) {{
                                        dropdown.style.top = `${{triggerRect.top - dropdownRect.height - 6}}px`;
                                        dropdown.style.transformOrigin = 'center bottom';
                                    }} else {{
                                        dropdown.style.top = `${{triggerRect.bottom + 6}}px`;
                                        dropdown.style.transformOrigin = 'center top';
                                    }}
                                    dropdown.style.left = `${{triggerRect.left}}px`;
                                    dropdown.style.minWidth = `${{triggerRect.width}}px`;
                                    break;
                            }}
                        }};

                        const openDropdown = () => {{
                            isOpen = true;

                            // Set state to open first to remove scale transform for accurate measurements
                            dropdown.setAttribute('data-state', 'open');

                            // Make dropdown invisible but rendered to measure true height
                            dropdown.style.visibility = 'hidden';
                            dropdown.style.pointerEvents = 'auto';

                            // Force reflow to ensure height is calculated
                            dropdown.offsetHeight;

                            // Calculate position with accurate height
                            updatePosition();

                            // Now make it visible
                            dropdown.style.visibility = 'visible';

                            // Lock all scrollable elements
                            window.ScrollLock.lock();

                            // Close on click outside
                            setTimeout(() => {{
                                document.addEventListener('click', handleClickOutside);
                            }}, 0);
                        }};

                        const closeDropdown = () => {{
                            isOpen = false;
                            dropdown.setAttribute('data-state', 'closed');
                            dropdown.style.pointerEvents = 'none';
                            document.removeEventListener('click', handleClickOutside);

                            // Unlock scroll after animation (200ms delay)
                            window.ScrollLock.unlock(200);
                        }};

                        const handleClickOutside = (e) => {{
                            if (!dropdown.contains(e.target) && !trigger.contains(e.target)) {{
                                closeDropdown();
                            }}
                        }};

                        // Toggle dropdown when trigger is clicked
                        trigger.addEventListener('click', (e) => {{
                            e.stopPropagation();

                            // Check if any other dropdown is open
                            const allDropdowns = document.querySelectorAll('[data-target=\"target__dropdown\"]');
                            let otherDropdownOpen = false;
                            allDropdowns.forEach(dd => {{
                                if (dd !== dropdown && dd.getAttribute('data-state') === 'open') {{
                                    otherDropdownOpen = true;
                                    dd.setAttribute('data-state', 'closed');
                                    dd.style.pointerEvents = 'none';
                                    // Unlock scroll
                                    if (window.ScrollLock) {{
                                        window.ScrollLock.unlock(200);
                                    }}
                                }}
                            }});

                            // If another dropdown was open, just close it and don't open this one
                            if (otherDropdownOpen) {{
                                return;
                            }}

                            // Normal toggle behavior
                            if (isOpen) {{
                                closeDropdown();
                            }} else {{
                                openDropdown();
                            }}
                        }});

                        // Close when action is clicked
                        const actions = dropdown.querySelectorAll('[data-dropdown-close]');
                        actions.forEach(action => {{
                            action.addEventListener('click', () => {{
                                closeDropdown();
                            }});
                        }});

                        // Handle ESC key to close
                        document.addEventListener('keydown', (e) => {{
                            if (e.key === 'Escape' && isOpen) {{
                                e.preventDefault();
                                closeDropdown();
                            }}
                        }});
                    }};

                    if (document.readyState === 'loading') {{
                        document.addEventListener('DOMContentLoaded', setupDropdown);
                    }} else {{
                        setupDropdown();
                    }}
                }})();
                "#,
                target_id_for_script,
                target_id_for_script,
            )}
        </script>
    }
}

#[component]
pub fn DropdownMenuSub(children: Children) -> impl IntoView {
    // TODO. Find a better way for dropdown__menu_sub_trigger.
    clx! {DropdownMenuSubRoot, li, "dropdown__menu_sub_trigger", " relative inline-flex relative gap-2 items-center py-1.5 px-2 w-full text-sm no-underline rounded-sm transition-colors duration-200 cursor-pointer text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"}

    view! { <DropdownMenuSubRoot>{children()}</DropdownMenuSubRoot> }
}

#[component]
pub fn DropdownMenuSubTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!("flex items-center justify-between w-full", class);

    view! {
        <span attr:data-name="DropdownMenuSubTrigger" class=class>
            <span class="flex gap-2 items-center">{children()}</span>
            <ChevronRight class="opacity-70 size-4" />
        </span>
    }
}

#[component]
pub fn DropdownMenuSubItem(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-3 py-2 text-sm transition-all duration-150 ease text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer hover:translate-x-[2px]",
        class
    );

    view! {
        <li data-name="DropdownMenuSubItem" class=class data-dropdown-close="true">
            {children()}
        </li>
    }
}
