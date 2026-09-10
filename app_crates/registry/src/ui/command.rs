use dioxus::prelude::*;
use icons::Check;
use tw_merge::tw_merge;

use crate::ui::button::{Button, ButtonVariant};

const TRIGGER_ID_QUALIFIER: &str = "command__trigger";

/* ========================================================== */
/*                     ✨ CONTEXT ✨                          */
/* ========================================================== */

#[derive(Clone)]
struct CommandDialogContext {
    dialog_id: String,
}

#[derive(Clone, Copy)]
struct CommandContext {
    search_query_signal: Signal<String>,
    should_filter: bool,
}

/* ========================================================== */
/*                     ✨ CLX COMPONENTS ✨                   */
/* ========================================================== */

#[component]
pub fn CommandHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex flex-col gap-2 text-center hidden sm:text-left", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "CommandHeader", class: "{merged}", {children} } }
}

#[component]
pub fn CommandTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-lg font-semibold leading-none", class.as_deref().unwrap_or(""));
    rsx! { h2 { "data-name": "CommandTitle", class: "{merged}", {children} } }
}

#[component]
pub fn CommandDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "CommandDescription", class: "{merged}", {children} } }
}

#[component]
pub fn CommandList(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(into, optional)] tabindex: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "overflow-y-auto overflow-x-hidden max-h-[300px] scroll-py-1 no__scrollbar min-h-80 scroll-pt-2 scroll-pb-1.5",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": "CommandList",
            class: "{merged}",
            id: id.as_deref(),
            tabindex: tabindex.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn CommandGroup(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] role: Option<String>,
    #[props(into, optional)] data_category: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("overflow-hidden p-1 text-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        div {
            "data-name": "CommandGroup",
            class: "{merged}",
            role: role.as_deref(),
            "data-category": data_category.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn CommandItemLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    #[props(into, optional)] target: Option<String>,
    #[props(into, optional)] rel: Option<String>,
    #[props(into, optional)] data_add_cmd: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "data-[selected=true]:text-accent-foreground [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default hover:cursor-pointer items-center gap-2 px-2 py-1.5 text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 data-[selected=true]:border-input data-[selected=true]:bg-muted/50 hover:bg-muted h-9 rounded-md border border-transparent font-medium",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        a {
            "data-name": "CommandItemLink",
            class: "{merged}",
            href: href.as_deref(),
            target: target.as_deref(),
            rel: rel.as_deref(),
            "data-add-cmd": data_add_cmd.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn CommandGroupLabel(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_hidden: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-muted-foreground px-2 py-1.5 text-xs font-medium", class.as_deref().unwrap_or(""));
    rsx! {
        div {
            "data-name": "CommandGroupLabel",
            class: "{merged}",
            "aria-hidden": aria_hidden.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn CommandFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex gap-4 items-center px-4 h-10 text-xs font-medium rounded-b-xl border-t text-muted-foreground border-t-border bg-muted",
        class.as_deref().unwrap_or("")
    );
    rsx! { footer { "data-name": "CommandFooter", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ DIALOG PROVIDER ✨                  */
/* ========================================================== */

#[component]
pub fn CommandDialogProvider(children: Element, #[props(into)] id: String) -> Element {
    let context = CommandDialogContext { dialog_id: id };
    use_context_provider(|| context);
    rsx! { {children} }
}

#[component]
pub fn CommandDialogTrigger(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let context = use_context::<CommandDialogContext>();
    let trigger_id = format!("{TRIGGER_ID_QUALIFIER}__{}", context.dialog_id);

    rsx! {
        Button {
            class: class.unwrap_or_default(),
            variant: ButtonVariant::Outline,
            id: "{trigger_id}",
            {children}
        }
    }
}

#[component]
pub fn CommandDialog(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let context = use_context::<CommandDialogContext>();
    let merged_class = tw_merge!(
        "grid fixed z-100 gap-4 p-2 w-full bg-clip-padding rounded-xl border border-none ring-4 shadow-2xl sm:max-w-lg bg-background top-[50%] left-[50%] max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] ring-neutral-200/80 transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class.as_deref().unwrap_or("")
    );

    let dialog_id = context.dialog_id.clone();
    let backdrop_id = format!("{dialog_id}__{TRIGGER_ID_QUALIFIER}");
    let trigger_id = format!("{TRIGGER_ID_QUALIFIER}__{dialog_id}");

    let script_content = format!(
        r#"
        (function() {{
            const KEY_HANDLER_KEY = '__commandDialog_{dialog_id}_keyHandler';
            const CLICK_HANDLER_KEY = '__commandDialog_{dialog_id}_clickHandler';
            const BACKDROP_HANDLER_KEY = '__commandDialog_{dialog_id}_backdropHandler';

            const setupDialog = () => {{
                const dialog = document.querySelector('#{dialog_id}');
                const backdrop = document.querySelector('#{backdrop_id}');

                if (!dialog || !backdrop) {{
                    setTimeout(setupDialog, 50);
                    return;
                }}

                if (window[KEY_HANDLER_KEY]) {{
                    document.removeEventListener('keydown', window[KEY_HANDLER_KEY]);
                }}
                if (window[CLICK_HANDLER_KEY]) {{
                    document.removeEventListener('click', window[CLICK_HANDLER_KEY]);
                }}
                if (window[BACKDROP_HANDLER_KEY]) {{
                    document.removeEventListener('click', window[BACKDROP_HANDLER_KEY]);
                }}

                const clickHandler = (e) => {{
                    const openBtn = e.target.closest('#{trigger_id}');
                    if (!openBtn) return;

                    const currentDialog = document.querySelector('#{dialog_id}');
                    const currentBackdrop = document.querySelector('#{backdrop_id}');
                    if (!currentDialog || !currentBackdrop) return;

                    window.ScrollLock.lock();
                    currentDialog.setAttribute('data-state', 'open');
                    currentBackdrop.setAttribute('data-state', 'open');
                    currentDialog.style.pointerEvents = 'auto';
                    currentBackdrop.style.pointerEvents = 'auto';
                }};

                const backdropHandler = (e) => {{
                    const clickedBackdrop = e.target.closest('#{backdrop_id}');
                    if (!clickedBackdrop) return;

                    const currentDialog = document.querySelector('#{dialog_id}');
                    const currentBackdrop = document.querySelector('#{backdrop_id}');
                    if (!currentDialog || !currentBackdrop) return;

                    currentDialog.setAttribute('data-state', 'closed');
                    currentBackdrop.setAttribute('data-state', 'closed');
                    currentDialog.style.pointerEvents = 'none';
                    currentBackdrop.style.pointerEvents = 'none';
                    window.ScrollLock.unlock(100);
                }};

                const keyHandler = (e) => {{
                    const currentDialog = document.querySelector('#{dialog_id}');
                    const currentBackdrop = document.querySelector('#{backdrop_id}');
                    if (!currentDialog || !currentBackdrop) return;

                    const openDialog = () => {{
                        window.ScrollLock.lock();
                        currentDialog.setAttribute('data-state', 'open');
                        currentBackdrop.setAttribute('data-state', 'open');
                        currentDialog.style.pointerEvents = 'auto';
                        currentBackdrop.style.pointerEvents = 'auto';
                    }};

                    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {{
                        e.preventDefault();
                        if (currentDialog.getAttribute('data-state') !== 'open') openDialog();
                    }}
                    else if (e.key === '/' && currentDialog.getAttribute('data-state') !== 'open') {{
                        const el = document.activeElement;
                        const tag = el?.tagName;
                        if (tag === 'INPUT' || tag === 'TEXTAREA' || el?.isContentEditable) return;
                        e.preventDefault();
                        openDialog();
                    }}
                    else if (e.key === 'Escape' && currentDialog.getAttribute('data-state') === 'open') {{
                        e.preventDefault();
                        currentDialog.setAttribute('data-state', 'closed');
                        currentBackdrop.setAttribute('data-state', 'closed');
                        currentDialog.style.pointerEvents = 'none';
                        currentBackdrop.style.pointerEvents = 'none';
                        window.ScrollLock.unlock(100);
                    }}
                }};

                window[KEY_HANDLER_KEY] = keyHandler;
                window[CLICK_HANDLER_KEY] = clickHandler;
                window[BACKDROP_HANDLER_KEY] = backdropHandler;
                document.addEventListener('keydown', keyHandler);
                document.addEventListener('click', clickHandler);
                document.addEventListener('click', backdropHandler);
            }};

            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', setupDialog);
            }} else {{
                setupDialog();
            }}
        }})();
        "#
    );

    rsx! {
        // Backdrop
        div {
            "data-name": "CommandDialogBackdrop",
            id: "{backdrop_id}",
            class: "fixed inset-0 transition-opacity duration-200 pointer-events-none z-60 bg-black/50 data-[state=closed]:opacity-0 data-[state=open]:opacity-100",
            "data-state": "closed",
        }
        // Dialog
        div {
            "data-name": "CommandDialog",
            class: "{merged_class}",
            id: "{dialog_id}",
            "data-state": "closed",
            tabindex: "-1",
            style: "pointer-events: none;",
            {children}
        }
        script { dangerous_inner_html: "{script_content}" }
    }
}

/* ========================================================== */
/*                     ✨ COMMAND ✨                          */
/* ========================================================== */

#[component]
pub fn Command(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    /// When false, disables client-side filtering (use for server-side search).
    /// Default: true (client-side filtering enabled).
    #[props(default = true)]
    should_filter: bool,
) -> Element {
    let dialog_context = try_use_context::<CommandDialogContext>();
    let search_query_signal = use_signal(String::new);
    let command_context = CommandContext { search_query_signal, should_filter };

    use_context_provider(|| command_context);

    let merged_class = tw_merge!(
        "flex overflow-hidden flex-col w-full h-full bg-transparent rounded-none text-popover-foreground",
        class.as_deref().unwrap_or("")
    );

    let script_content = if let Some(ctx) = dialog_context {
        let dialog_id = ctx.dialog_id;
        let backdrop_id = format!("{dialog_id}__{TRIGGER_ID_QUALIFIER}");
        format!(
            r#"
            (function() {{
                const setupCommandKeyboard = () => {{
                    const FIRST_INDEX = 0;
                    const dialog = document.querySelector('#{dialog_id}');
                    const backdrop = document.querySelector('#{backdrop_id}');
                    const command_list = dialog?.querySelector('[data-name="CommandList"]');
                    const command_input = dialog?.querySelector('[data-name="CommandInput"]');
                    const command_items = command_list?.querySelectorAll('[data-name="CommandItemLink"]');
                    const command_groups = command_list?.querySelectorAll('[data-name="CommandGroup"]');

                    if (!command_items || command_items.length === 0 || !command_input) {{
                        setTimeout(setupCommandKeyboard, 50);
                        return;
                    }}

                    let index = FIRST_INDEX;

                    const getVisibleItems = () => {{
                        return Array.from(command_items).filter(item => {{
                            if (item.style.display === 'none') return false;
                            const group = item.closest('[data-name="CommandGroup"]');
                            return !group || group.style.display !== 'none';
                        }});
                    }};

                    const select = (i) => {{
                        const visibleItems = getVisibleItems();
                        if (visibleItems.length === 0) return;

                        command_items.forEach(item => item.setAttribute('aria-selected', 'false'));
                        if (visibleItems[i]) {{
                            visibleItems[i].setAttribute('aria-selected', 'true');
                            visibleItems[i].scrollIntoView({{ block: 'nearest', behavior: 'smooth' }});
                        }}
                    }};

                    const filterItems = (query) => {{
                        const searchQuery = query.toLowerCase().trim();

                        command_items.forEach(item => {{
                            const text = item.textContent.toLowerCase();
                            if (searchQuery === '' || text.includes(searchQuery)) {{
                                item.style.display = '';
                            }} else {{
                                item.style.display = 'none';
                            }}
                        }});

                        command_groups.forEach(group => {{
                            const groupItems = group.querySelectorAll('[data-name="CommandItemLink"]');
                            const hasVisibleItems = Array.from(groupItems).some(item => item.style.display !== 'none');
                            group.style.display = hasVisibleItems ? '' : 'none';
                        }});

                        index = FIRST_INDEX;
                        select(FIRST_INDEX);
                    }};

                    command_input.addEventListener('input', (e) => {{
                        filterItems(e.target.value);
                    }});

                    const closeDialog = () => {{
                        backdrop.click();
                    }};

                    command_items.forEach((item) => {{
                        item.addEventListener('click', () => {{
                            closeDialog();
                        }});
                    }});

                    document.addEventListener('keydown', (e) => {{
                        if (dialog?.getAttribute('data-state') !== 'open') return;

                        const visibleItems = getVisibleItems();
                        if (visibleItems.length === 0) return;

                        if (e.key === 'ArrowDown') {{
                            e.preventDefault();
                            if (index < visibleItems.length - 1) select(++index);
                        }} else if (e.key === 'ArrowUp') {{
                            e.preventDefault();
                            if (index > FIRST_INDEX) select(--index);
                            else command_list.scrollTo({{ top: 0, behavior: 'smooth' }});
                        }} else if (e.key === 'Enter') {{
                            e.preventDefault();
                            visibleItems[index]?.click();
                        }}
                    }});

                    const observer = new MutationObserver((mutations) => {{
                        mutations.forEach((mutation) => {{
                            if (mutation.attributeName === 'data-state') {{
                                if (dialog.getAttribute('data-state') === 'open') {{
                                    command_input.value = '';
                                    filterItems('');
                                    index = FIRST_INDEX;
                                    select(FIRST_INDEX);
                                    setTimeout(() => command_input.focus(), 0);
                                }}
                            }}
                        }});
                    }});

                    observer.observe(dialog, {{ attributes: true }});
                }};

                if (document.readyState === 'loading') {{
                    document.addEventListener('DOMContentLoaded', setupCommandKeyboard);
                }} else {{
                    setupCommandKeyboard();
                }}
            }})();
            "#
        )
    } else {
        r#"
        (function() {
            const setupCommand = () => {
                const FIRST_INDEX = 0;
                const command_list = document.querySelector('[data-name="CommandList"]');
                const command_input = document.querySelector('[data-name="CommandInput"]');
                const command_items = command_list?.querySelectorAll('[data-name="CommandItemLink"]');
                const command_groups = command_list?.querySelectorAll('[data-name="CommandGroup"]');

                if (!command_items || command_items.length === 0) {
                    setTimeout(setupCommand, 50);
                    return;
                }

                let index = FIRST_INDEX;

                const getVisibleItems = () => {
                    return Array.from(command_items).filter(item => item.style.display !== 'none');
                };

                const select = (i) => {
                    const visibleItems = getVisibleItems();
                    if (visibleItems.length === 0) return;

                    command_items.forEach(item => item.setAttribute('aria-selected', 'false'));
                    if (visibleItems[i]) {
                        visibleItems[i].setAttribute('aria-selected', 'true');
                        visibleItems[i].scrollIntoView({ block: 'nearest', behavior: 'smooth' });
                    }
                };

                const filterItems = (query) => {
                    const searchQuery = query.toLowerCase().trim();

                    command_items.forEach(item => {
                        const text = item.textContent.toLowerCase();
                        if (searchQuery === '' || text.includes(searchQuery)) {
                            item.style.display = '';
                        } else {
                            item.style.display = 'none';
                        }
                    });

                    if (command_groups) {
                        command_groups.forEach(group => {
                            const groupItems = group.querySelectorAll('[data-name="CommandItemLink"]');
                            const hasVisibleItems = Array.from(groupItems).some(item => item.style.display !== 'none');
                            group.style.display = hasVisibleItems ? '' : 'none';
                        });
                    }

                    index = FIRST_INDEX;
                    select(FIRST_INDEX);
                };

                if (command_input) {
                    command_input.addEventListener('input', (e) => {
                        filterItems(e.target.value);
                    });
                }

                select(FIRST_INDEX);

                document.addEventListener('keydown', (e) => {
                    const visibleItems = getVisibleItems();
                    if (visibleItems.length === 0) return;

                    if (e.key === 'ArrowDown') {
                        e.preventDefault();
                        if (index < visibleItems.length - 1) select(++index);
                    } else if (e.key === 'ArrowUp') {
                        e.preventDefault();
                        if (index > FIRST_INDEX) select(--index);
                        else command_list.scrollTo({ top: 0, behavior: 'smooth' });
                    } else if (e.key === 'Enter') {
                        e.preventDefault();
                        visibleItems[index]?.click();
                    }
                });
            };

            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', setupCommand);
            } else {
                setupCommand();
            }
        })();
        "#
        .to_string()
    };

    rsx! {
        style {
            r#"
            /* Command component - aria-selected styling */
            [data-name="CommandItemLink"][aria-selected="true"],
            [data-name="CommandItem"][aria-selected="true"] {{
                background-color: var(--color-muted);
            }}
            /* Hide CommandEmpty when there are visible items */
            [data-name="CommandList"]:has([data-name="CommandItem"][style*="flex"]) [data-name="CommandEmpty"] {{
                display: none;
            }}
            "#
        }
        div { "data-name": "Command", class: "{merged_class}", tabindex: "-1",
            {children}
        }
        script { dangerous_inner_html: "{script_content}" }
    }
}

#[component]
pub fn CommandInput(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] placeholder: Option<String>,
    /// Callback fired when search input changes. Use for server-side search.
    #[props(optional)]
    on_search_change: Option<EventHandler<String>>,
) -> Element {
    let command_context = use_context::<CommandContext>();
    let merged_class = tw_merge!(
        "flex py-3 w-full h-10 text-sm bg-transparent rounded-md disabled:opacity-50 disabled:cursor-not-allowed placeholder:text-muted-foreground outline-hidden",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        input {
            "data-name": "CommandInput",
            class: "{merged_class}",
            placeholder: placeholder.as_deref(),
            autocomplete: "off",
            spellcheck: false,
            "aria-autocomplete": "list",
            role: "combobox",
            "aria-expanded": "true",
            "aria-controls": "command_demo",
            "aria-label": "Search documentation",
            r#type: "text",
            value: "{(command_context.search_query_signal)()}",
            oninput: move |ev| {
                let value = ev.value();
                let mut search_query_signal = command_context.search_query_signal;
                search_query_signal.set(value.clone());
                if let Some(cb) = on_search_change {
                    cb.call(value);
                }
            },
            autofocus: true,
            "data-1p-ignore": "true",
            "data-bwignore": "true",
            "data-lpignore": "true",
        }
    }
}

#[component]
pub fn CommandEmpty(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("py-6 text-sm text-center", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "CommandEmpty", class: "{merged_class}", {children} } }
}

#[component]
pub fn CommandItem(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] value: Option<String>,
    #[props(optional)] on_select: Option<EventHandler<()>>,
    #[props(default = false)] selected: bool,
    /// Reserve space for check icon even when not selected (for alignment)
    #[props(default = false)]
    reserve_check_space: bool,
) -> Element {
    let command_context = use_context::<CommandContext>();
    let value_for_filter = value.clone().unwrap_or_default();

    let merged_class = tw_merge!(
        "group relative flex gap-2 items-center px-2 py-1.5 text-sm rounded-sm cursor-default select-none outline-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 hover:bg-accent hover:text-accent-foreground",
        class.as_deref().unwrap_or("")
    );

    let is_visible = use_memo(move || {
        if !command_context.should_filter {
            return true;
        }
        let search = (command_context.search_query_signal)().to_lowercase();
        if search.is_empty() {
            return true;
        }
        value_for_filter.to_lowercase().contains(&search)
    });

    let check_class = if reserve_check_space {
        "ml-auto size-4 text-muted-foreground opacity-0 group-aria-selected:opacity-100"
    } else {
        "ml-auto size-4 text-muted-foreground hidden group-aria-selected:block"
    };

    rsx! {
        div {
            "data-name": "CommandItem",
            class: "{merged_class}",
            role: "option",
            tabindex: "0",
            "aria-selected": if selected { "true" } else { "false" },
            style: if is_visible() { "display: flex;" } else { "display: none;" },
            onclick: move |_| {
                if let Some(cb) = on_select {
                    cb.call(());
                }
            },
            {children}
            Check { class: check_class }
        }
    }
}
