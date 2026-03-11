use icons::ChevronRight;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;
use wasm_bindgen::JsCast;

use crate::hooks::use_random::use_random_id_for;

/// Programmatically close any open context menu.
pub fn close_context_menu() {
    let Some(document) = window().document() else {
        return;
    };
    let Some(menu) = document.query_selector("[data-target='target__context'][data-state='open']").ok().flatten()
    else {
        return;
    };
    let _ = menu.set_attribute("data-state", "closed");
    if let Some(el) = menu.dyn_ref::<web_sys::HtmlElement>() {
        let _ = el.style().set_property("pointer-events", "none");
    }
}

mod components {
    use super::*;
    clx! {ContextMenuLabel, span, "px-2 py-1.5 text-sm font-medium data-inset:pl-8", "mb-1"}
    clx! {ContextMenuGroup, ul, "group"}
    clx! {ContextMenuItem, li, "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4"}
    clx! {ContextMenuSubContent, ul, "context__menu_sub_content", "rounded-md border bg-card shadow-lg p-1 absolute z-[100] min-w-[160px] opacity-0 invisible translate-x-[-8px] transition-all duration-200 ease-out pointer-events-none"}
    clx! {ContextMenuLink, a, "w-full inline-flex gap-2 items-center"}
}

pub use components::*;

#[component]
pub fn ContextMenuAction(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] aria_selected: Option<Signal<bool>>,
    #[prop(optional, into)] href: Option<String>,
) -> impl IntoView {
    let _ctx = expect_context::<ContextMenuContext>();

    let class = tw_merge!(
        "inline-flex gap-2 items-center w-full text-sm text-left transition-colors duration-200 focus:outline-none focus-visible:outline-none text-popover-foreground [&_svg:not([class*='size-'])]:size-4",
        class
    );

    let aria_selected_attr = move || aria_selected.map(|s| s.get()).unwrap_or(false).to_string();

    if let Some(href) = href {
        view! {
            <a
                data-name="ContextMenuAction"
                class=class
                href=href
                aria-selected=aria_selected_attr
                data-context-close="true"
            >
                {children()}
            </a>
        }
        .into_any()
    } else {
        view! {
            <button
                type="button"
                data-name="ContextMenuAction"
                class=class
                data-context-close="true"
                aria-selected=aria_selected_attr
            >
                {children()}
            </button>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct ContextMenuContext {
    target_id: String,
}

#[component]
pub fn ContextMenu(children: Children) -> impl IntoView {
    let context_target_id = use_random_id_for("context");

    let ctx = ContextMenuContext { target_id: context_target_id };

    view! {
        <Provider value=ctx>
            <style>
                "
                /* Submenu Styles */
                .context__menu_sub_content {
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
                
                .context__menu_sub_trigger:hover .context__menu_sub_content {
                    opacity: 1;
                    visibility: visible;
                    transform: translateX(0);
                    pointer-events: auto;
                }
                "
            </style>

            <div data-name="ContextMenu" class="contents">
                {children()}
            </div>
        </Provider>
    }
}

/// Wrapper that triggers the context menu on right-click.
/// The `on_open` callback is triggered when the context menu opens (right-click).
#[component]
pub fn ContextMenuTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] on_open: Option<Callback<()>>,
) -> impl IntoView {
    let ctx = expect_context::<ContextMenuContext>();
    let trigger_class = tw_merge!("contents", class);

    view! {
        <div
            class=trigger_class
            data-name="ContextMenuTrigger"
            data-context-trigger=ctx.target_id
            on:contextmenu=move |_| {
                if let Some(cb) = on_open {
                    cb.run(());
                }
            }
        >
            {children()}
        </div>
    }
}

/// Content of the context menu that appears on right-click.
/// The `on_close` callback is triggered when the menu closes (click outside, ESC key, or action click).
#[component]
pub fn ContextMenuContent(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let ctx = expect_context::<ContextMenuContext>();

    let base_classes = "z-50 p-1 rounded-md border bg-card shadow-md w-[200px] fixed transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100";

    let class = tw_merge!(base_classes, class);

    let target_id_for_script = ctx.target_id.clone();

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name="ContextMenuContent"
            class=class
            // Listen for custom 'contextmenuclose' event dispatched by JS when menu closes
            on:contextmenuclose=move |_: web_sys::CustomEvent| {
                if let Some(cb) = on_close {
                    cb.run(());
                }
            }
            id=ctx.target_id
            data-target="target__context"
            data-state="closed"
            style="pointer-events: none;"
        >
            {children()}
        </div>

        <script>
            {format!(
                r#"
                (function() {{
                    const setupContextMenu = () => {{
                        const menu = document.querySelector('#{}');
                        const trigger = document.querySelector('[data-context-trigger="{}"]');

                        if (!menu || !trigger) {{
                            setTimeout(setupContextMenu, 50);
                            return;
                        }}

                        if (menu.hasAttribute('data-initialized')) {{
                            return;
                        }}
                        menu.setAttribute('data-initialized', 'true');

                        let isOpen = false;

                        const updatePosition = (x, y) => {{
                            const menuRect = menu.getBoundingClientRect();
                            const viewportHeight = window.innerHeight;
                            const viewportWidth = window.innerWidth;

                            // Calculate position, ensuring menu stays within viewport
                            let left = x;
                            let top = y;

                            // Adjust if menu would go off right edge
                            if (x + menuRect.width > viewportWidth) {{
                                left = x - menuRect.width;
                            }}

                            // Adjust if menu would go off bottom edge
                            if (y + menuRect.height > viewportHeight) {{
                                top = y - menuRect.height;
                            }}

                            menu.style.left = `${{left}}px`;
                            menu.style.top = `${{top}}px`;
                            menu.style.transformOrigin = 'top left';
                        }};

                        const openMenu = (x, y) => {{
                            isOpen = true;

                            // Close any other open context menus
                            const allMenus = document.querySelectorAll('[data-target="target__context"]');
                            allMenus.forEach(m => {{
                                if (m !== menu && m.getAttribute('data-state') === 'open') {{
                                    m.setAttribute('data-state', 'closed');
                                    m.style.pointerEvents = 'none';
                                }}
                            }});

                            menu.setAttribute('data-state', 'open');
                            menu.style.visibility = 'hidden';
                            menu.style.pointerEvents = 'auto';

                            // Force reflow
                            menu.offsetHeight;

                            updatePosition(x, y);
                            menu.style.visibility = 'visible';

                            // Lock scroll
                            if (window.ScrollLock) {{
                                window.ScrollLock.lock();
                            }}

                            setTimeout(() => {{
                                document.addEventListener('click', handleClickOutside);
                                document.addEventListener('contextmenu', handleContextOutside);
                            }}, 0);
                        }};

                        const closeMenu = () => {{
                            isOpen = false;
                            menu.setAttribute('data-state', 'closed');
                            menu.style.pointerEvents = 'none';
                            document.removeEventListener('click', handleClickOutside);
                            document.removeEventListener('contextmenu', handleContextOutside);

                            // Dispatch custom event for Leptos to listen to
                            menu.dispatchEvent(new CustomEvent('contextmenuclose', {{ bubbles: false }}));

                            if (window.ScrollLock) {{
                                window.ScrollLock.unlock(200);
                            }}
                        }};

                        const handleClickOutside = (e) => {{
                            if (!menu.contains(e.target)) {{
                                closeMenu();
                            }}
                        }};

                        const handleContextOutside = (e) => {{
                            if (!trigger.contains(e.target)) {{
                                closeMenu();
                            }}
                        }};

                        // Right-click on trigger
                        trigger.addEventListener('contextmenu', (e) => {{
                            e.preventDefault();
                            e.stopPropagation();

                            if (isOpen) {{
                                closeMenu();
                            }}
                            openMenu(e.clientX, e.clientY);
                        }});

                        // Close when action is clicked
                        const actions = menu.querySelectorAll('[data-context-close]');
                        actions.forEach(action => {{
                            action.addEventListener('click', () => {{
                                closeMenu();
                            }});
                        }});

                        // Handle ESC key
                        document.addEventListener('keydown', (e) => {{
                            if (e.key === 'Escape' && isOpen) {{
                                e.preventDefault();
                                closeMenu();
                            }}
                        }});
                    }};

                    if (document.readyState === 'loading') {{
                        document.addEventListener('DOMContentLoaded', setupContextMenu);
                    }} else {{
                        setupContextMenu();
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
pub fn ContextMenuSub(children: Children) -> impl IntoView {
    clx! {ContextMenuSubRoot, li, "context__menu_sub_trigger", " relative inline-flex relative gap-2 items-center py-1.5 px-2 w-full text-sm no-underline rounded-sm transition-colors duration-200 cursor-pointer text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"}

    view! { <ContextMenuSubRoot>{children()}</ContextMenuSubRoot> }
}

#[component]
pub fn ContextMenuSubTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!("flex items-center justify-between w-full", class);

    view! {
        <span data-name="ContextMenuSubTrigger" class=class>
            <span class="flex gap-2 items-center">{children()}</span>
            <ChevronRight class="opacity-70 size-4" />
        </span>
    }
}

#[component]
pub fn ContextMenuSubItem(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-3 py-2 text-sm transition-all duration-150 ease text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer hover:translate-x-[2px]",
        class
    );

    view! {
        <li data-name="ContextMenuSubItem" class=class data-context-close="true">
            {children()}
        </li>
    }
}
