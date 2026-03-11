use icons::{Check, ChevronRight};
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::hooks::use_random::use_random_id_for;
pub use crate::ui::separator::Separator as MenubarSeparator;

/* ========================================================== */
/*                     SIMPLE CLX COMPONENTS                  */
/* ========================================================== */

mod components {
    use super::*;
    clx! {MenubarGroup, ul, "group"}
    clx! {MenubarLabel, div, "px-1.5 py-1 text-sm font-medium data-inset:pl-7"}
    clx! {MenubarSubContent, ul, "menubar__sub_content", "rounded-md border bg-card shadow-lg p-1 absolute z-[100] min-w-[160px] opacity-0 invisible translate-x-[-8px] transition-all duration-200 ease-out pointer-events-none"}
}

pub use components::*;

/* ========================================================== */
/*                     MENUBAR SHORTCUT                       */
/* ========================================================== */

#[component]
pub fn MenubarShortcut(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!("ml-auto text-xs tracking-widest text-muted-foreground", class);
    view! {
        <span data-slot="menubar-shortcut" class=class>
            {children()}
        </span>
    }
}

/* ========================================================== */
/*                     MENUBAR ITEM                            */
/* ========================================================== */

#[component]
pub fn MenubarItem(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!(
        "relative inline-flex gap-1.5 items-center w-full rounded-sm px-1.5 py-1 text-sm cursor-default no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4 data-inset:pl-7",
        class
    );
    view! {
        <li data-name="MenubarItem" class=class data-menubar-close="true">
            {children()}
        </li>
    }
}

/* ========================================================== */
/*                     CHECKBOX ITEM                          */
/* ========================================================== */

#[component]
pub fn MenubarCheckboxItem(
    children: Children,
    checked: RwSignal<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let class = tw_merge!(
        "group relative inline-flex gap-1.5 items-center w-full rounded-sm pl-7 pr-1.5 py-1 text-sm cursor-default transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class
    );

    view! {
        <li
            data-name="MenubarCheckboxItem"
            class=class
            role="menuitemcheckbox"
            aria-checked=move || checked.get().to_string()
            on:click=move |_| checked.update(|v| *v = !*v)
        >
            <span class="flex absolute left-1.5 justify-center items-center pointer-events-none size-4">
                <Check class="opacity-0 group-aria-checked:opacity-100 size-3.5" />
            </span>
            {children()}
        </li>
    }
}

/* ========================================================== */
/*                     RADIO GROUP                            */
/* ========================================================== */

#[derive(Clone)]
struct MenubarRadioContext<T: Clone + PartialEq + Send + Sync + 'static> {
    value_signal: RwSignal<T>,
}

#[component]
pub fn MenubarRadioGroup<T>(children: Children, value: RwSignal<T>) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let ctx = MenubarRadioContext { value_signal: value };

    view! {
        <Provider value=ctx>
            <ul data-name="MenubarRadioGroup" role="group" class="group">
                {children()}
            </ul>
        </Provider>
    }
}

#[component]
pub fn MenubarRadioItem<T>(children: Children, value: T, #[prop(optional, into)] class: String) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let ctx = expect_context::<MenubarRadioContext<T>>();

    let value_for_check = value.clone();
    let value_for_click = value;
    let is_selected = move || ctx.value_signal.get() == value_for_check;

    let class = tw_merge!(
        "group relative inline-flex gap-1.5 items-center w-full rounded-sm pl-7 pr-1.5 py-1 text-sm cursor-default transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class
    );

    view! {
        <li
            data-name="MenubarRadioItem"
            class=class
            role="menuitemradio"
            aria-checked=move || is_selected().to_string()
            on:click=move |_| ctx.value_signal.set(value_for_click.clone())
        >
            <span class="flex absolute left-1.5 justify-center items-center pointer-events-none size-4">
                <Check class="opacity-0 group-aria-checked:opacity-100 size-3.5" />
            </span>
            {children()}
        </li>
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
pub fn Menubar(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let menubar_id = use_random_id_for("menubar");
    let ctx = MenubarContext { menubar_id: menubar_id.clone() };

    let class = tw_merge!("flex h-8 items-center gap-0.5 rounded-lg border bg-background p-[3px]", class);

    view! {
        <Provider value=ctx>
            <style>
                "
                .menubar__sub_content {
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
                
                .menubar__sub_trigger:hover .menubar__sub_content {
                    opacity: 1;
                    visibility: visible;
                    transform: translateX(0);
                    pointer-events: auto;
                }
                "
            </style>

            <div data-name="Menubar" data-menubar-id=menubar_id class=class>
                {children()}
            </div>
        </Provider>
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
pub fn MenubarMenu(children: Children) -> impl IntoView {
    let menubar_ctx = expect_context::<MenubarContext>();
    let menu_id = use_random_id_for("menubarmenu");

    let ctx = MenubarMenuContext { menu_id, menubar_id: menubar_ctx.menubar_id };

    view! {
        <Provider value=ctx>
            <div data-name="MenubarMenu" class="relative">
                {children()}
            </div>
        </Provider>
    }
}

/* ========================================================== */
/*                     MENUBAR TRIGGER                        */
/* ========================================================== */

#[component]
pub fn MenubarTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<MenubarMenuContext>();
    let class = tw_merge!(
        "flex items-center rounded-sm px-2 py-[2px] text-sm font-medium outline-none select-none cursor-default transition-colors hover:bg-muted aria-expanded:bg-muted",
        class
    );

    view! {
        <button
            type="button"
            data-name="MenubarTrigger"
            data-menubar-trigger=ctx.menu_id
            data-menubar-id=ctx.menubar_id
            class=class
            aria-expanded="false"
        >
            {children()}
        </button>
    }
}

/* ========================================================== */
/*                     MENUBAR CONTENT                        */
/* ========================================================== */

#[component]
pub fn MenubarContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<MenubarMenuContext>();

    let base_classes = "z-50 p-1 min-w-36 rounded-md border bg-card shadow-md fixed transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100";
    let class = tw_merge!(base_classes, class);

    let menu_id = ctx.menu_id.clone();
    let menubar_id = ctx.menubar_id;

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <ul
            data-name="MenubarContent"
            data-menubar-content=""
            class=class
            id=menu_id.clone()
            data-state="closed"
            style="pointer-events: none;"
        >
            {children()}
        </ul>

        <script>
            {format!(
                r#"
                (function() {{
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
                            // Close other menus in this menubar
                            menubarRoot.querySelectorAll('[data-menubar-content]').forEach(m => {{
                                if (m !== menu && m.getAttribute('data-state') === 'open') {{
                                    m.setAttribute('data-state', 'closed');
                                    m.style.pointerEvents = 'none';
                                    // Update aria-expanded on the other trigger
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
                                // Close all menus in this menubar
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

                        // Click trigger: toggle
                        trigger.addEventListener('click', (e) => {{
                            e.stopPropagation();
                            if (isOpen()) {{
                                closeMenu();
                            }} else {{
                                openMenu();
                            }}
                        }});

                        // Hover trigger: switch between menus when bar is active
                        trigger.addEventListener('mouseenter', () => {{
                            if (menubarRoot.hasAttribute('data-active') && !isOpen()) {{
                                openMenu();
                            }}
                        }});

                        // Close when item with data-menubar-close is clicked (event delegation)
                        menu.addEventListener('click', (e) => {{
                            if (e.target.closest('[data-menubar-close]')) {{
                                closeMenu();
                            }}
                        }});

                        // ESC key
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
                }})();
                "#,
                menu_id,
                menubar_id,
            )}
        </script>
    }
}

/* ========================================================== */
/*                     SUBMENU                                */
/* ========================================================== */

#[component]
pub fn MenubarSub(children: Children) -> impl IntoView {
    clx! {MenubarSubRoot, li, "menubar__sub_trigger", "relative inline-flex gap-1.5 items-center py-1 px-1.5 w-full text-sm no-underline rounded-sm transition-colors duration-200 cursor-default text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"}

    view! { <MenubarSubRoot>{children()}</MenubarSubRoot> }
}

#[component]
pub fn MenubarSubTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!("flex items-center justify-between w-full", class);

    view! {
        <span data-name="MenubarSubTrigger" class=class>
            <span class="flex gap-1.5 items-center">{children()}</span>
            <ChevronRight class="opacity-70 size-4" />
        </span>
    }
}

#[component]
pub fn MenubarSubItem(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!(
        "inline-flex gap-1.5 items-center w-full rounded-sm px-3 py-2 text-sm transition-all duration-150 ease text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-default hover:translate-x-[2px]",
        class
    );

    view! {
        <li data-name="MenubarSubItem" class=class data-menubar-close="true">
            {children()}
        </li>
    }
}
