use dioxus::prelude::*;
use icons::{Check, ChevronRight};
use tw_merge::tw_merge;

use crate::hooks::use_random::use_random_id_for;
pub use crate::ui::separator::Separator as MenubarSeparator;

/* ========================================================== */
/*                     SIMPLE COMPONENTS                      */
/* ========================================================== */

#[component]
pub fn MenubarGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("group", class.as_deref().unwrap_or(""));
    rsx! { ul { class: "{merged}", {children} } }
}

#[component]
pub fn MenubarLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("px-1.5 py-1 text-sm font-medium data-inset:pl-7", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn MenubarSubContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "menubar__sub_content rounded-md border bg-card shadow-lg p-1 absolute z-[100] min-w-[160px] opacity-0 invisible translate-x-[-8px] transition-all duration-200 ease-out pointer-events-none",
        class.as_deref().unwrap_or("")
    );
    rsx! { ul { class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     MENUBAR SHORTCUT                       */
/* ========================================================== */

#[component]
pub fn MenubarShortcut(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("ml-auto text-xs tracking-widest text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        span { "data-slot": "menubar-shortcut", class: "{merged}", {children} }
    }
}

/* ========================================================== */
/*                     MENUBAR ITEM                            */
/* ========================================================== */

#[component]
pub fn MenubarItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "relative inline-flex gap-1.5 items-center w-full rounded-sm px-1.5 py-1 text-sm cursor-default no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4 data-inset:pl-7",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        li { "data-name": "MenubarItem", class: "{merged}", "data-menubar-close": "true", {children} }
    }
}

/* ========================================================== */
/*                     CHECKBOX ITEM                          */
/* ========================================================== */

#[component]
pub fn MenubarCheckboxItem(
    #[props(into, optional)] class: Option<String>,
    checked: Signal<bool>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "group relative inline-flex gap-1.5 items-center w-full rounded-sm pl-7 pr-1.5 py-1 text-sm cursor-default transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    let mut checked = checked;

    rsx! {
        li {
            "data-name": "MenubarCheckboxItem",
            class: "{merged}",
            role: "menuitemcheckbox",
            "aria-checked": if checked() { "true" } else { "false" },
            onclick: move |_| checked.set(!checked()),
            span { class: "flex absolute left-1.5 justify-center items-center pointer-events-none size-4",
                Check { class: "opacity-0 group-aria-checked:opacity-100 size-3.5" }
            }
            {children}
        }
    }
}

/* ========================================================== */
/*                     RADIO GROUP                            */
/* ========================================================== */

#[derive(Clone)]
struct MenubarRadioContext<T: Clone + PartialEq + Send + Sync + 'static> {
    value_signal: Signal<T>,
}

#[component]
pub fn MenubarRadioGroup<T: Clone + PartialEq + Send + Sync + 'static>(children: Element, value: Signal<T>) -> Element {
    provide_context(MenubarRadioContext { value_signal: value });

    rsx! {
        ul { "data-name": "MenubarRadioGroup", role: "group", class: "group", {children} }
    }
}

#[component]
pub fn MenubarRadioItem<T: Clone + PartialEq + Send + Sync + 'static>(
    children: Element,
    value: T,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let ctx = use_context::<MenubarRadioContext<T>>();
    let mut value_signal = ctx.value_signal;

    let value_for_check = value.clone();
    let value_for_click = value;
    let is_selected = move || value_signal() == value_for_check;

    let merged = tw_merge!(
        "group relative inline-flex gap-1.5 items-center w-full rounded-sm pl-7 pr-1.5 py-1 text-sm cursor-default transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        li {
            "data-name": "MenubarRadioItem",
            class: "{merged}",
            role: "menuitemradio",
            "aria-checked": if is_selected() { "true" } else { "false" },
            onclick: move |_| value_signal.set(value_for_click.clone()),
            span { class: "flex absolute left-1.5 justify-center items-center pointer-events-none size-4",
                Check { class: "opacity-0 group-aria-checked:opacity-100 size-3.5" }
            }
            {children}
        }
    }
}

/* ========================================================== */
/*                     MENUBAR ROOT                           */
/* ========================================================== */

#[derive(Clone)]
struct MenubarContext {
    menubar_id: String,
}

#[component]
pub fn Menubar(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let menubar_id = use_random_id_for("menubar");
    provide_context(MenubarContext { menubar_id: menubar_id.clone() });

    let merged = tw_merge!(
        "flex h-8 items-center gap-0.5 rounded-lg border bg-background p-[3px]",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        style { r#"
            .menubar__sub_content {{
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

            .menubar__sub_trigger:hover .menubar__sub_content {{
                opacity: 1;
                visibility: visible;
                transform: translateX(0);
                pointer-events: auto;
            }}
        "# }

        div {
            "data-name": "Menubar",
            "data-menubar-id": "{menubar_id}",
            class: "{merged}",
            {children}
        }
    }
}

/* ========================================================== */
/*                     MENUBAR MENU                           */
/* ========================================================== */

#[derive(Clone)]
struct MenubarMenuContext {
    menu_id: String,
    menubar_id: String,
}

#[component]
pub fn MenubarMenu(children: Element) -> Element {
    let menubar_ctx = use_context::<MenubarContext>();
    let menu_id = use_random_id_for("menubarmenu");

    provide_context(MenubarMenuContext { menu_id, menubar_id: menubar_ctx.menubar_id });

    rsx! {
        div { "data-name": "MenubarMenu", class: "relative", {children} }
    }
}

/* ========================================================== */
/*                     MENUBAR TRIGGER                        */
/* ========================================================== */

#[component]
pub fn MenubarTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<MenubarMenuContext>();
    let merged = tw_merge!(
        "flex items-center rounded-sm px-2 py-[2px] text-sm font-medium outline-none select-none cursor-default transition-colors hover:bg-muted aria-expanded:bg-muted",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            r#type: "button",
            "data-name": "MenubarTrigger",
            "data-menubar-trigger": "{ctx.menu_id}",
            "data-menubar-id": "{ctx.menubar_id}",
            class: "{merged}",
            "aria-expanded": "false",
            {children}
        }
    }
}

/* ========================================================== */
/*                     MENUBAR CONTENT                        */
/* ========================================================== */

#[component]
pub fn MenubarContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<MenubarMenuContext>();

    let merged = tw_merge!(
        "z-50 p-1 min-w-36 rounded-md border bg-card shadow-md fixed transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class.as_deref().unwrap_or("")
    );

    let menu_id = ctx.menu_id.clone();
    let menubar_id = ctx.menubar_id.clone();

    let script = format!(
        r#"(function() {{
            const setupMenu = () => {{
                const menu = document.querySelector('#{0}');
                const trigger = document.querySelector('[data-menubar-trigger="{0}"]');
                const menubarRoot = document.querySelector('[data-menubar-id="{1}"]');

                if (!menu || !trigger || !menubarRoot) {{
                    setTimeout(setupMenu, 50);
                    return;
                }}

                if (menu.hasAttribute('data-initialized')) return;
                menu.setAttribute('data-initialized', 'true');

                const isOpen = () => menu.getAttribute('data-state') === 'open';

                const updatePosition = () => {{
                    const triggerRect = trigger.getBoundingClientRect();
                    const menuRect = menu.getBoundingClientRect();
                    const viewportHeight = window.innerHeight;
                    const spaceBelow = viewportHeight - triggerRect.bottom;
                    const spaceAbove = triggerRect.top;

                    const shouldPositionAbove = spaceAbove >= menuRect.height && spaceBelow < menuRect.height;

                    if (shouldPositionAbove) {{
                        menu.style.top = `${{triggerRect.top - menuRect.height - 4}}px`;
                        menu.style.transformOrigin = 'left bottom';
                    }} else {{
                        menu.style.top = `${{triggerRect.bottom + 4}}px`;
                        menu.style.transformOrigin = 'left top';
                    }}
                    menu.style.left = `${{triggerRect.left}}px`;
                }};

                const openMenu = () => {{
                    menubarRoot.querySelectorAll('[data-menubar-content]').forEach(m => {{
                        if (m !== menu && m.getAttribute('data-state') === 'open') {{
                            m.setAttribute('data-state', 'closed');
                            m.style.pointerEvents = 'none';
                            const otherId = m.id;
                            const otherTrigger = menubarRoot.querySelector(`[data-menubar-trigger="${{otherId}}"]`);
                            if (otherTrigger) otherTrigger.setAttribute('aria-expanded', 'false');
                        }}
                    }});

                    menubarRoot.setAttribute('data-active', 'true');
                    trigger.setAttribute('aria-expanded', 'true');

                    menu.setAttribute('data-state', 'open');
                    menu.style.visibility = 'hidden';
                    menu.style.pointerEvents = 'auto';
                    menu.offsetHeight;
                    updatePosition();
                    menu.style.visibility = 'visible';

                    window.ScrollLock?.lock();

                    setTimeout(() => {{
                        document.addEventListener('click', handleClickOutside);
                    }}, 0);
                }};

                const closeMenu = () => {{
                    menu.setAttribute('data-state', 'closed');
                    menu.style.pointerEvents = 'none';
                    trigger.setAttribute('aria-expanded', 'false');
                    document.removeEventListener('click', handleClickOutside);

                    const anyOpen = [...menubarRoot.querySelectorAll('[data-menubar-content]')]
                        .some(m => m.getAttribute('data-state') === 'open');
                    if (!anyOpen) {{
                        menubarRoot.removeAttribute('data-active');
                        window.ScrollLock?.unlock(200);
                    }}
                }};

                const handleClickOutside = (e) => {{
                    if (!menubarRoot.contains(e.target)) {{
                        menubarRoot.querySelectorAll('[data-menubar-content]').forEach(m => {{
                            m.setAttribute('data-state', 'closed');
                            m.style.pointerEvents = 'none';
                        }});
                        menubarRoot.querySelectorAll('[data-menubar-trigger]').forEach(t => {{
                            t.setAttribute('aria-expanded', 'false');
                        }});
                        menubarRoot.removeAttribute('data-active');
                        window.ScrollLock?.unlock(200);
                        document.removeEventListener('click', handleClickOutside);
                    }}
                }};

                trigger.addEventListener('click', (e) => {{
                    e.stopPropagation();
                    if (isOpen()) {{
                        closeMenu();
                    }} else {{
                        openMenu();
                    }}
                }});

                trigger.addEventListener('mouseenter', () => {{
                    if (menubarRoot.hasAttribute('data-active') && !isOpen()) {{
                        openMenu();
                    }}
                }});

                menu.addEventListener('click', (e) => {{
                    if (e.target.closest('[data-menubar-close]')) {{
                        closeMenu();
                    }}
                }});

                document.addEventListener('keydown', (e) => {{
                    if (e.key === 'Escape' && isOpen()) {{
                        e.preventDefault();
                        closeMenu();
                    }}
                }});
            }};

            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', setupMenu);
            }} else {{
                setupMenu();
            }}
        }})();"#,
        menu_id, menubar_id,
    );

    rsx! {
        ul {
            "data-name": "MenubarContent",
            "data-menubar-content": "",
            class: "{merged}",
            id: "{menu_id}",
            "data-state": "closed",
            style: "pointer-events: none;",
            {children}
        }
        script { dangerous_inner_html: "{script}" }
    }
}

/* ========================================================== */
/*                     SUBMENU                                */
/* ========================================================== */

#[component]
pub fn MenubarSub(children: Element) -> Element {
    rsx! {
        li {
            "data-name": "MenubarSub",
            class: "menubar__sub_trigger relative inline-flex gap-1.5 items-center py-1 px-1.5 w-full text-sm no-underline rounded-sm transition-colors duration-200 cursor-default text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground",
            {children}
        }
    }
}

#[component]
pub fn MenubarSubTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex items-center justify-between w-full", class.as_deref().unwrap_or(""));

    rsx! {
        span { "data-name": "MenubarSubTrigger", class: "{merged}",
            span { class: "flex gap-1.5 items-center", {children} }
            ChevronRight { class: "opacity-70 size-4" }
        }
    }
}

#[component]
pub fn MenubarSubItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-1.5 items-center w-full rounded-sm px-3 py-2 text-sm transition-all duration-150 ease text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-default hover:translate-x-[2px]",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        li { "data-name": "MenubarSubItem", class: "{merged}", "data-menubar-close": "true", {children} }
    }
}
