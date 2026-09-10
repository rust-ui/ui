use dioxus::prelude::*;
use icons::ChevronRight;
use tw_merge::tw_merge;

use crate::hooks::use_random::use_random_id_for;

/// Programmatically close any open context menu.
pub fn close_context_menu() {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
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
}

/* ========================================================== */
/*                     ✨ CONTEXT ✨                          */
/* ========================================================== */

#[derive(Clone)]
struct ContextMenuContext {
    target_id: String,
}

/* ========================================================== */
/*                     ✨ CLX COMPONENTS ✨                   */
/* ========================================================== */

#[component]
pub fn ContextMenuLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("px-2 py-1.5 text-sm font-medium data-inset:pl-8 mb-1", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "ContextMenuLabel", class: "{merged}", {children} } }
}

#[component]
pub fn ContextMenuGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("group", class.as_deref().unwrap_or(""));
    rsx! { ul { "data-name": "ContextMenuGroup", class: "{merged}", {children} } }
}

#[component]
pub fn ContextMenuItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    rsx! { li { "data-name": "ContextMenuItem", class: "{merged}", {children} } }
}

#[component]
pub fn ContextMenuSubContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "context__menu_sub_content rounded-md border bg-card shadow-lg p-1 absolute z-[100] min-w-[160px] opacity-0 invisible translate-x-[-8px] transition-all duration-200 ease-out pointer-events-none",
        class.as_deref().unwrap_or("")
    );
    rsx! { ul { "data-name": "ContextMenuSubContent", class: "{merged}", {children} } }
}

#[component]
pub fn ContextMenuLink(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("w-full inline-flex gap-2 items-center", class.as_deref().unwrap_or(""));
    rsx! { a { "data-name": "ContextMenuLink", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ ACTION ✨                           */
/* ========================================================== */

#[component]
pub fn ContextMenuAction(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<()>>,
) -> Element {
    let _ctx = use_context::<ContextMenuContext>();

    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full text-sm text-left transition-colors duration-200 focus:outline-none focus-visible:outline-none text-popover-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );

    if let Some(href) = href {
        rsx! {
            a {
                "data-name": "ContextMenuAction",
                class: "{merged}",
                href: "{href}",
                "data-context-close": "true",
                onclick: move |_| {
                    if let Some(cb) = &onclick {
                        cb.call(());
                    }
                },
                {children}
            }
        }
    } else {
        rsx! {
            button {
                r#type: "button",
                "data-name": "ContextMenuAction",
                class: "{merged}",
                "data-context-close": "true",
                onclick: move |_| {
                    if let Some(cb) = &onclick {
                        cb.call(());
                    }
                },
                {children}
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ CONTEXT MENU ✨                     */
/* ========================================================== */

#[component]
pub fn ContextMenu(children: Element) -> Element {
    let context_target_id = use_random_id_for("context");

    let ctx = ContextMenuContext { target_id: context_target_id };
    provide_context(ctx);

    rsx! {
        style {
            r#"
            /* Submenu Styles */
            .context__menu_sub_content {{
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
            }}

            .context__menu_sub_trigger:hover .context__menu_sub_content {{
                opacity: 1;
                visibility: visible;
                transform: translateX(0);
                pointer-events: auto;
            }}
            "#
        }
        div { "data-name": "ContextMenu", class: "contents",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ TRIGGER ✨                          */
/* ========================================================== */

#[component]
pub fn ContextMenuTrigger(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] on_open: Option<EventHandler<()>>,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();
    let trigger_class = tw_merge!("contents", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{trigger_class}",
            "data-name": "ContextMenuTrigger",
            "data-context-trigger": "{ctx.target_id}",
            oncontextmenu: move |_| {
                if let Some(cb) = on_open {
                    cb.call(());
                }
            },
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ CONTENT ✨                          */
/* ========================================================== */

#[component]
pub fn ContextMenuContent(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
) -> Element {
    let ctx = use_context::<ContextMenuContext>();

    let base_classes = "z-50 p-1 rounded-md border bg-card shadow-md w-[200px] fixed transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100";
    let merged = tw_merge!(base_classes, class.as_deref().unwrap_or(""));

    let target_id_for_script = ctx.target_id.clone();
    let script = format!(
        r#"
        (function() {{
            const setupContextMenu = () => {{
                const menu = document.querySelector('#{target_id_for_script}');
                const trigger = document.querySelector('[data-context-trigger="{target_id_for_script}"]');

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

                    let left = x;
                    let top = y;

                    if (x + menuRect.width > viewportWidth) {{
                        left = x - menuRect.width;
                    }}

                    if (y + menuRect.height > viewportHeight) {{
                        top = y - menuRect.height;
                    }}

                    menu.style.left = `${{left}}px`;
                    menu.style.top = `${{top}}px`;
                    menu.style.transformOrigin = 'top left';
                }};

                const openMenu = (x, y) => {{
                    isOpen = true;

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

                    menu.offsetHeight;

                    updatePosition(x, y);
                    menu.style.visibility = 'visible';

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

                trigger.addEventListener('contextmenu', (e) => {{
                    e.preventDefault();
                    e.stopPropagation();

                    if (isOpen) {{
                        closeMenu();
                    }}
                    openMenu(e.clientX, e.clientY);
                }});

                const actions = menu.querySelectorAll('[data-context-close]');
                actions.forEach(action => {{
                    action.addEventListener('click', () => {{
                        closeMenu();
                    }});
                }});

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
        "#
    );

    rsx! {
        div {
            "data-name": "ContextMenuContent",
            class: "{merged}",
            id: "{ctx.target_id}",
            "data-target": "target__context",
            "data-state": "closed",
            style: "pointer-events: none;",
            {children}
        }
        script { dangerous_inner_html: "{script}" }
    }
}

/* ========================================================== */
/*                     ✨ SUB MENU ✨                         */
/* ========================================================== */

#[component]
pub fn ContextMenuSub(children: Element) -> Element {
    rsx! {
        li {
            class: "context__menu_sub_trigger relative inline-flex gap-2 items-center py-1.5 px-2 w-full text-sm no-underline rounded-sm transition-colors duration-200 cursor-pointer text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground",
            {children}
        }
    }
}

#[component]
pub fn ContextMenuSubTrigger(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!("flex items-center justify-between w-full", class.as_deref().unwrap_or(""));
    rsx! {
        span { "data-name": "ContextMenuSubTrigger", class: "{merged}",
            span { class: "flex gap-2 items-center", {children} }
            ChevronRight { class: "opacity-70 size-4" }
        }
    }
}

#[component]
pub fn ContextMenuSubItem(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-3 py-2 text-sm transition-all duration-150 ease text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer hover:translate-x-[2px]",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        li { "data-name": "ContextMenuSubItem", class: "{merged}", "data-context-close": "true",
            {children}
        }
    }
}
