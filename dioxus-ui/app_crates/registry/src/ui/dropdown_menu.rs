use dioxus::prelude::*;
use icons::{Check, ChevronRight};
use tw_merge::tw_merge;

use crate::hooks::use_random::use_random_id_for;
pub use crate::ui::separator::Separator as DropdownMenuSeparator;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuAlign {
    #[default]
    Start,
    StartOuter,
    End,
    EndOuter,
    Center,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuPosition {
    #[default]
    Auto,
    Top,
    Bottom,
}

#[derive(Clone)]
struct DropdownMenuContext {
    target_id: String,
    align: DropdownMenuAlign,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuActionVariant {
    #[default]
    Default,
    Destructive,
}

#[component]
pub fn DropdownMenu(
    #[props(default = DropdownMenuAlign::default())] align: DropdownMenuAlign,
    children: Element,
) -> Element {
    let dropdown_target_id = use_random_id_for("dropdown");
    provide_context(DropdownMenuContext { target_id: dropdown_target_id.clone(), align });

    rsx! {
        div { "data-name": "DropdownMenu",
            style { r#"
                .dropdown__menu_sub_content {{
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
                .dropdown__menu_sub_trigger:hover .dropdown__menu_sub_content {{
                    opacity: 1;
                    visibility: visible;
                    transform: translateX(0);
                    pointer-events: auto;
                }}
            "# }
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] as_child: bool,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let merged = tw_merge!(
        "px-4 py-2 h-9 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus:outline-none focus:ring-1 focus:ring-ring focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 border bg-background border-input hover:bg-accent hover:text-accent-foreground",
        class.as_deref().unwrap_or("")
    );

    if as_child {
        rsx! { span { "data-name": "DropdownMenuTrigger", "data-dropdown-trigger": "{ctx.target_id}", class: "contents", {children} } }
    } else {
        rsx! {
            button {
                type: "button",
                class: "{merged}",
                "data-name": "DropdownMenuTrigger",
                "data-dropdown-trigger": "{ctx.target_id}",
                tabindex: "0",
                {children}
            }
        }
    }
}

#[component]
pub fn DropdownMenuContent(
    #[props(into, optional)] class: Option<String>,
    #[props(default = DropdownMenuPosition::default())] position: DropdownMenuPosition,
    children: Element,
) -> Element {
    let ctx = use_context::<DropdownMenuContext>();
    let width_class = match ctx.align {
        DropdownMenuAlign::Center => "min-w-full",
        _ => "w-[180px]",
    };
    let merged = tw_merge!(
        width_class,
        "z-50 p-1 rounded-md border bg-card shadow-md h-fit fixed transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class.as_deref().unwrap_or("")
    );

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
    let target_id = ctx.target_id.clone();

    let script = format!(
        r#"(function() {{
            const setupDropdown = () => {{
                const dropdown = document.querySelector('#{tid}');
                const trigger = document.querySelector('[data-dropdown-trigger="{tid}"]');
                if (!dropdown || !trigger) {{ setTimeout(setupDropdown, 50); return; }}
                if (dropdown.hasAttribute('data-initialized')) return;
                dropdown.setAttribute('data-initialized', 'true');
                let isOpen = false;
                const updatePosition = () => {{
                    const triggerEl = getComputedStyle(trigger).display === 'contents' ? trigger.firstElementChild : trigger;
                    const triggerRect = triggerEl.getBoundingClientRect();
                    const dropdownRect = dropdown.getBoundingClientRect();
                    const viewportHeight = window.innerHeight;
                    const spaceBelow = viewportHeight - triggerRect.bottom;
                    const spaceAbove = triggerRect.top;
                    const align = dropdown.getAttribute('data-align') || 'start';
                    const position = dropdown.getAttribute('data-position') || 'auto';
                    let shouldPositionAbove = false;
                    if (position === 'top') {{ shouldPositionAbove = true; }}
                    else if (position === 'bottom') {{ shouldPositionAbove = false; }}
                    else {{ shouldPositionAbove = spaceAbove >= dropdownRect.height && spaceBelow < dropdownRect.height; }}
                    switch (align) {{
                        case 'start':
                            dropdown.style.top = shouldPositionAbove ? `${{triggerRect.top - dropdownRect.height - 6}}px` : `${{triggerRect.bottom + 6}}px`;
                            dropdown.style.left = `${{triggerRect.left}}px`;
                            dropdown.style.transformOrigin = shouldPositionAbove ? 'left bottom' : 'left top';
                            break;
                        case 'end':
                            dropdown.style.top = shouldPositionAbove ? `${{triggerRect.top - dropdownRect.height - 6}}px` : `${{triggerRect.bottom + 6}}px`;
                            dropdown.style.left = `${{triggerRect.right - dropdownRect.width}}px`;
                            dropdown.style.transformOrigin = shouldPositionAbove ? 'right bottom' : 'right top';
                            break;
                        case 'start-outer':
                            dropdown.style.top = shouldPositionAbove ? `${{triggerRect.top - dropdownRect.height - 6}}px` : `${{triggerRect.top}}px`;
                            dropdown.style.left = `${{triggerRect.left - dropdownRect.width - 16}}px`;
                            dropdown.style.transformOrigin = shouldPositionAbove ? 'right bottom' : 'right top';
                            break;
                        case 'end-outer':
                            dropdown.style.top = shouldPositionAbove ? `${{triggerRect.top - dropdownRect.height - 6}}px` : `${{triggerRect.top}}px`;
                            dropdown.style.left = `${{triggerRect.right + 8}}px`;
                            dropdown.style.transformOrigin = shouldPositionAbove ? 'left bottom' : 'left top';
                            break;
                        case 'center':
                            dropdown.style.top = shouldPositionAbove ? `${{triggerRect.top - dropdownRect.height - 6}}px` : `${{triggerRect.bottom + 6}}px`;
                            dropdown.style.left = `${{triggerRect.left}}px`;
                            dropdown.style.minWidth = `${{triggerRect.width}}px`;
                            dropdown.style.transformOrigin = shouldPositionAbove ? 'center bottom' : 'center top';
                            break;
                    }}
                }};
                const closeDropdown = () => {{
                    isOpen = false;
                    dropdown.setAttribute('data-state', 'closed');
                    dropdown.style.pointerEvents = 'none';
                    document.removeEventListener('click', handleClickOutside);
                    if (window.ScrollLock) window.ScrollLock.unlock(200);
                }};
                const handleClickOutside = (e) => {{
                    if (!dropdown.contains(e.target) && !trigger.contains(e.target)) {{ closeDropdown(); }}
                }};
                const openDropdown = () => {{
                    isOpen = true;
                    dropdown.setAttribute('data-state', 'open');
                    dropdown.style.visibility = 'hidden';
                    dropdown.style.pointerEvents = 'auto';
                    dropdown.offsetHeight;
                    updatePosition();
                    dropdown.style.visibility = 'visible';
                    if (window.ScrollLock) window.ScrollLock.lock();
                    setTimeout(() => document.addEventListener('click', handleClickOutside), 0);
                }};
                trigger.addEventListener('click', (e) => {{
                    e.stopPropagation();
                    const all = document.querySelectorAll('[data-target="target__dropdown"]');
                    let otherOpen = false;
                    all.forEach(dd => {{
                        if (dd !== dropdown && dd.getAttribute('data-state') === 'open') {{
                            otherOpen = true;
                            dd.setAttribute('data-state', 'closed');
                            dd.style.pointerEvents = 'none';
                            if (window.ScrollLock) window.ScrollLock.unlock(200);
                        }}
                    }});
                    if (otherOpen) return;
                    if (isOpen) closeDropdown(); else openDropdown();
                }});
                dropdown.querySelectorAll('[data-dropdown-close]').forEach(action => action.addEventListener('click', closeDropdown));
                document.addEventListener('keydown', (e) => {{ if (e.key === 'Escape' && isOpen) {{ e.preventDefault(); closeDropdown(); }} }});
            }};
            if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', setupDropdown);
            else setupDropdown();
        }})();"#,
        tid = target_id,
    );

    rsx! {
        div {
            "data-name": "DropdownMenuContent",
            id: "{target_id}",
            class: "{merged}",
            "data-target": "target__dropdown",
            "data-state": "closed",
            "data-align": align_for_script,
            "data-position": position_for_script,
            style: "pointer-events: none;",
            {children}
        }
        script { dangerous_inner_html: "{script}" }
    }
}

#[component]
pub fn DropdownMenuLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("px-2 py-1.5 text-sm font-medium data-inset:pl-8 mb-1", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "DropdownMenuLabel", class: "{merged}", {children} } }
}

#[component]
pub fn DropdownMenuGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("group", class.as_deref().unwrap_or(""));
    rsx! { ul { "data-name": "DropdownMenuGroup", class: "{merged}", {children} } }
}

#[component]
pub fn DropdownMenuItem(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        li {
            "data-name": "DropdownMenuItem",
            class: "{merged}",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuLink(
    #[props(into, optional)] href: Option<String>,
    #[props(into, optional)] target: Option<String>,
    #[props(into, optional)] rel: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        a {
            "data-name": "DropdownMenuLink",
            class: "w-full inline-flex gap-2 items-center",
            href: "{href.as_deref().unwrap_or(\"#\")}",
            target: target,
            rel: rel,
            {children}
        }
    }
}

#[component]
pub fn DropdownMenuAction(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    #[props(default = DropdownMenuActionVariant::default())] variant: DropdownMenuActionVariant,
    children: Element,
) -> Element {
    let variant_class = match variant {
        DropdownMenuActionVariant::Default => "text-popover-foreground hover:bg-accent hover:text-accent-foreground",
        DropdownMenuActionVariant::Destructive => "text-destructive hover:bg-destructive/10 hover:text-destructive",
    };
    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full text-sm text-left transition-colors duration-200 focus:outline-none focus-visible:outline-none [&_svg:not([class*='size-'])]:size-4",
        variant_class,
        class.as_deref().unwrap_or("")
    );
    if let Some(href) = href {
        rsx! { a { "data-name": "DropdownMenuAction", class: "{merged}", href: "{href}", "data-dropdown-close": "true", {children} } }
    } else {
        rsx! { button { "data-name": "DropdownMenuAction", class: "{merged}", type: "button", "data-dropdown-close": "true", {children} } }
    }
}

#[component]
pub fn DropdownMenuSub(children: Element) -> Element {
    rsx! { li { "data-name": "DropdownMenuSub", class: "dropdown__menu_sub_trigger relative inline-flex relative gap-2 items-center py-1.5 px-2 w-full text-sm no-underline rounded-sm transition-colors duration-200 cursor-pointer text-popover-foreground [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground", {children} } }
}

#[component]
pub fn DropdownMenuSubTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex items-center justify-between w-full", class.as_deref().unwrap_or(""));
    rsx! {
        span { "data-name": "DropdownMenuSubTrigger", class: "{merged}",
            span { class: "flex gap-2 items-center", {children} }
            ChevronRight { class: "opacity-70 size-4" }
        }
    }
}

#[component]
pub fn DropdownMenuSubContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "dropdown__menu_sub_content rounded-md border bg-card shadow-lg p-1 absolute z-[100] min-w-[160px] opacity-0 invisible translate-x-[-8px] transition-all duration-200 ease-out pointer-events-none",
        class.as_deref().unwrap_or("")
    );
    rsx! { ul { "data-name": "DropdownMenuSubContent", class: "{merged}", {children} } }
}

#[component]
pub fn DropdownMenuSubItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-2 items-center w-full rounded-sm px-3 py-2 text-sm transition-all duration-150 ease text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer hover:translate-x-[2px]",
        class.as_deref().unwrap_or("")
    );
    rsx! { li { "data-name": "DropdownMenuSubItem", class: "{merged}", "data-dropdown-close": "true", {children} } }
}

#[component]
pub fn DropdownMenuRadioGroup<T: Clone + PartialEq + Send + Sync + 'static>(
    children: Element,
    value: Signal<T>,
) -> Element {
    provide_context(value);
    rsx! { ul { "data-name": "DropdownMenuRadioGroup", role: "group", class: "group", {children} } }
}

#[component]
pub fn DropdownMenuRadioItem<T: Clone + PartialEq + Send + Sync + 'static>(
    children: Element,
    value: T,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let mut selected = use_context::<Signal<T>>();
    let value_for_check = value.clone();
    let value_for_click = value;
    let class = tw_merge!(
        "group inline-flex gap-2 items-center w-full rounded-sm pl-2 pr-2 py-1.5 text-sm cursor-pointer no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    let is_selected = move || selected() == value_for_check;
    rsx! {
        li {
            "data-name": "DropdownMenuRadioItem",
            class: "{class}",
            role: "menuitemradio",
            "aria-checked": if is_selected() { "true" } else { "false" },
            "data-dropdown-close": "true",
            onclick: move |_| selected.set(value_for_click.clone()),
            {children}
            Check { class: "ml-auto opacity-0 size-4 text-muted-foreground group-aria-checked:opacity-100" }
        }
    }
}
