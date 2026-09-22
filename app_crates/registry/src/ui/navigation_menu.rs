use dioxus::prelude::*;
use icons::ChevronDown;
use tw_merge::tw_merge;

use crate::hooks::use_random::use_random_id_for;

/* ========================================================== */
/*                    TRIGGER STYLE HELPER                     */
/* ========================================================== */

#[must_use]
pub const fn navigation_menu_trigger_style() -> &'static str {
    "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50"
}

/* ========================================================== */
/*                         CONTEXTS                            */
/* ========================================================== */

#[derive(Clone)]
struct NavigationMenuContext {
    menu_id: String,
}

#[derive(Clone)]
struct NavigationMenuItemContext {
    item_id: String,
}

/* ========================================================== */
/*                       NAVIGATION MENU                       */
/* ========================================================== */

/// Root navigation wrapper. All content panels are anchored to this element's
/// rect (computed via JS, `position: fixed`), creating a shared viewport effect
/// without portaling, and staying correct under sticky/backdrop-blur ancestors
/// where plain CSS `position: absolute` breaks.
#[component]
pub fn NavigationMenu(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let menu_id = use_random_id_for("navmenu");
    provide_context(NavigationMenuContext { menu_id: menu_id.clone() });
    let merged =
        tw_merge!("relative z-10 flex max-w-max flex-1 items-center justify-center", class.as_deref().unwrap_or(""));

    // Large inline JS template with trailing named args; inlining every `{menu_id}`
    // into the format string would not improve readability.
    #[allow(clippy::uninlined_format_args)]
    let script = format!(
        r#"(function() {{
            const setup = () => {{
                const menuRoot = document.querySelector('[data-nav-menu="{menu_id}"]');
                if (!menuRoot) {{ setTimeout(setup, 50); return; }}
                if (menuRoot.hasAttribute('data-js-initialized')) return;
                menuRoot.setAttribute('data-js-initialized', 'true');

                const triggers = [...menuRoot.querySelectorAll('[data-nav-trigger]')];
                const getContent = (id) => menuRoot.querySelector('[data-nav-content="' + id + '"]');

                // `position: fixed` anchors to the nearest ancestor with a transform/filter/
                // backdrop-filter/etc (it creates a new containing block, same as `transform`).
                // Sticky headers commonly use `backdrop-blur`, so we can't assume the viewport.
                const getFixedContainingBlock = (el) => {{
                    let node = el.parentElement;
                    while (node && node !== document.body) {{
                        const cs = getComputedStyle(node);
                        if (
                            cs.transform !== 'none' ||
                            cs.filter !== 'none' ||
                            cs.backdropFilter !== 'none' ||
                            cs.perspective !== 'none' ||
                            cs.willChange === 'transform' ||
                            cs.willChange === 'filter' ||
                            (cs.contain && /layout|paint|strict|content/.test(cs.contain))
                        ) {{
                            return node;
                        }}
                        node = node.parentElement;
                    }}
                    return null;
                }};

                const positionContents = () => {{
                    const rect = menuRoot.getBoundingClientRect();
                    menuRoot.querySelectorAll('[data-nav-content]').forEach(content => {{
                        const containingBlock = getFixedContainingBlock(content);
                        const originTop = containingBlock ? containingBlock.getBoundingClientRect().top : 0;
                        const originLeft = containingBlock ? containingBlock.getBoundingClientRect().left : 0;
                        content.style.top = (rect.bottom - originTop + 6) + 'px';
                        content.style.left = (rect.left - originLeft) + 'px';
                    }});
                }};
                positionContents();
                window.addEventListener('resize', positionContents);
                window.addEventListener('scroll', positionContents, true);

                let activeItemId = null;
                let activeIndex  = -1;
                let hideTimer;

                const openItem = (trigger, idx) => {{
                    clearTimeout(hideTimer);
                    const itemId  = trigger.getAttribute('data-nav-trigger');
                    const content = getContent(itemId);
                    if (!content || activeItemId === itemId) return;
                    positionContents();

                    if (activeItemId) {{
                        const prevContent = getContent(activeItemId);
                        if (prevContent) {{
                            prevContent.setAttribute('data-motion', idx > activeIndex ? 'to-start' : 'to-end');
                            const prev = prevContent;
                            setTimeout(() => {{
                                prev.setAttribute('data-state', 'closed');
                                prev.removeAttribute('data-motion');
                            }}, 200);
                        }}
                        const prevTrigger = triggers[activeIndex];
                        if (prevTrigger) prevTrigger.setAttribute('data-state', 'closed');
                    }}

                    if (activeItemId) {{
                        content.setAttribute('data-motion', idx > activeIndex ? 'from-end' : 'from-start');
                    }} else {{
                        content.removeAttribute('data-motion');
                    }}
                    content.setAttribute('data-state', 'open');
                    trigger.setAttribute('data-state', 'open');
                    activeItemId = itemId;
                    activeIndex  = idx;
                }};

                const closeAll = (delay) => {{
                    hideTimer = setTimeout(() => {{
                        triggers.forEach(t => t.setAttribute('data-state', 'closed'));
                        if (activeItemId) {{
                            const content = getContent(activeItemId);
                            if (content) {{
                                content.setAttribute('data-motion', 'to-fade');
                                setTimeout(() => {{
                                    content.setAttribute('data-state', 'closed');
                                    content.removeAttribute('data-motion');
                                }}, 150);
                            }}
                        }}
                        activeItemId = null;
                        activeIndex  = -1;
                    }}, delay);
                }};

                triggers.forEach((trigger, idx) => {{
                    trigger.addEventListener('mouseenter', () => openItem(trigger, idx));
                    trigger.addEventListener('mouseleave', () => closeAll(150));
                }});

                menuRoot.querySelectorAll('[data-nav-content]').forEach(content => {{
                    content.addEventListener('mouseenter', () => clearTimeout(hideTimer));
                    content.addEventListener('mouseleave', () => closeAll(150));
                }});

                document.addEventListener('click', (e) => {{
                    if (!menuRoot.contains(e.target)) closeAll(0);
                }});

                document.addEventListener('keydown', (e) => {{
                    if (e.key === 'Escape') closeAll(0);
                }});
            }};

            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', setup);
            }} else {{
                setup();
            }}
        }})();"#,
        menu_id = menu_id,
    );

    rsx! {
        // Directional slide animations for content panels
        style { r#"
            @keyframes navFromStart {{
                from {{ opacity: 0; transform: translateX(-16px); }}
                to   {{ opacity: 1; transform: translateX(0); }}
            }}
            @keyframes navFromEnd {{
                from {{ opacity: 0; transform: translateX(16px); }}
                to   {{ opacity: 1; transform: translateX(0); }}
            }}
            @keyframes navToStart {{
                from {{ opacity: 1; transform: translateX(0); }}
                to   {{ opacity: 0; transform: translateX(-16px); }}
            }}
            @keyframes navToEnd {{
                from {{ opacity: 1; transform: translateX(0); }}
                to   {{ opacity: 0; transform: translateX(16px); }}
            }}
            @keyframes navFadeIn {{
                from {{ opacity: 0; transform: scale(0.96) translateY(-4px); }}
                to   {{ opacity: 1; transform: scale(1) translateY(0); }}
            }}
            @keyframes navFadeOut {{
                from {{ opacity: 1; transform: scale(1) translateY(0); }}
                to   {{ opacity: 0; transform: scale(0.96) translateY(-4px); }}
            }}
            [data-nav-content][data-motion='from-start'] {{ animation: navFromStart 200ms ease-out; }}
            [data-nav-content][data-motion='from-end']   {{ animation: navFromEnd 200ms ease-out; }}
            [data-nav-content][data-motion='to-start']   {{ animation: navToStart 200ms ease-out forwards; }}
            [data-nav-content][data-motion='to-end']     {{ animation: navToEnd 200ms ease-out forwards; }}
            [data-nav-content][data-motion='to-fade']    {{ animation: navFadeOut 150ms ease-out forwards; }}
            [data-nav-content][data-state='open']:not([data-motion]) {{ animation: navFadeIn 200ms ease-out; }}
        "# }

        nav {
            "data-name": "NavigationMenu",
            "data-nav-menu": "{menu_id}",
            class: "{merged}",
            {children}
        }

        // Single JS coordinator for all hover interactions and directional animation
        script { dangerous_inner_html: "{script}" }
    }
}

/* ========================================================== */
/*                    NAVIGATION MENU LIST                     */
/* ========================================================== */

#[component]
pub fn NavigationMenuList(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged =
        tw_merge!("group flex flex-1 list-none items-center justify-center gap-1", class.as_deref().unwrap_or(""));

    rsx! {
        ul { "data-name": "NavigationMenuList", class: "{merged}", {children} }
    }
}

/* ========================================================== */
/*                    NAVIGATION MENU ITEM                     */
/* ========================================================== */

/// NOTE: intentionally has NO `position: relative` so that `NavigationMenuContent`
/// (with `position: absolute`) escapes to the <nav> root, making all panels
/// appear at the same position → shared viewport effect.
#[component]
pub fn NavigationMenuItem(children: Element) -> Element {
    let item_id = use_random_id_for("navitem");
    provide_context(NavigationMenuItemContext { item_id });

    rsx! {
        li { "data-name": "NavigationMenuItem", {children} }
    }
}

/* ========================================================== */
/*                   NAVIGATION MENU TRIGGER                   */
/* ========================================================== */

#[component]
pub fn NavigationMenuTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let item_ctx = use_context::<NavigationMenuItemContext>();
    let menu_ctx = use_context::<NavigationMenuContext>();

    let merged = tw_merge!(
        "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50 cursor-default select-none",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            r#type: "button",
            "data-name": "NavigationMenuTrigger",
            "data-nav-trigger": "{item_ctx.item_id}",
            "data-nav-menu": "{menu_ctx.menu_id}",
            class: "{merged}",
            "data-state": "closed",
            {children}
            ChevronDown { class: "relative ml-1 transition duration-300 top-[1px] size-3 group-data-[state=open]:rotate-180" }
        }
    }
}

/* ========================================================== */
/*                  NAVIGATION MENU CONTENT                    */
/* ========================================================== */

/// Fixed-positioned (top/left set via JS from `NavigationMenu`'s rect), so all
/// content panels share the same anchor point below the menu bar regardless of
/// ancestor stacking context (sticky headers, backdrop-blur, etc).
#[component]
pub fn NavigationMenuContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<NavigationMenuItemContext>();

    let merged = tw_merge!(
        "fixed z-50 w-full rounded-md border bg-popover p-4 shadow-md data-[state=closed]:hidden md:w-auto",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "NavigationMenuContent",
            "data-nav-content": "{ctx.item_id}",
            class: "{merged}",
            "data-state": "closed",
            {children}
        }
    }
}

/* ========================================================== */
/*                    NAVIGATION MENU LINK                     */
/* ========================================================== */

#[component]
pub fn NavigationMenuLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    #[props(into, optional)] target: Option<String>,
    #[props(into, optional)] rel: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center rounded-sm text-sm font-medium transition-colors hover:text-foreground text-foreground/70 focus:outline-none",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        a {
            "data-name": "NavigationMenuLink",
            href: "{href.as_deref().unwrap_or(\"#\")}",
            target,
            rel,
            class: "{merged}",
            {children}
        }
    }
}
