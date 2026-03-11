use icons::Check;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::ui::button::{Button, ButtonVariant};

const TRIGGER_ID_QUALIFIER: &str = "command__trigger";

mod components {
    use super::*;
    clx! {CommandHeader, div, "flex flex-col gap-2 text-center hidden sm:text-left"} // sr-only
    clx! {CommandTitle, h2, "text-lg font-semibold leading-none"}
    clx! {CommandDescription, p, "text-sm text-muted-foreground"}
    clx! {CommandList, div, "overflow-y-auto overflow-x-hidden max-h-[300px] scroll-py-1 no__scrollbar min-h-80 scroll-pt-2 scroll-pb-1.5"}
    clx! {CommandGroup, div, "overflow-hidden p-1 text-foreground"}
    // TODO. This one should turn auto in a tag if we pass href (later, not urgent).
    clx! {CommandItemLink, a, "data-[selected=true]:text-accent-foreground [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default hover:cursor-pointer items-center gap-2 px-2 py-1.5 text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 data-[selected=true]:border-input data-[selected=true]:bg-muted/50 hover:bg-muted h-9 rounded-md border border-transparent font-medium"}
    clx! {CommandGroupLabel, div, "text-muted-foreground px-2 py-1.5 text-xs font-medium"}
    clx! {CommandFooter, footer, "flex gap-4 items-center px-4 h-10 text-xs font-medium rounded-b-xl border-t text-muted-foreground border-t-border bg-muted"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ CONTEXT ✨                          */
/* ========================================================== */

#[derive(Clone)]
struct CommandDialogContext {
    dialog_id: String,
}

#[derive(Clone, Copy)]
struct CommandContext {
    search_query_signal: RwSignal<String>,
    should_filter: bool,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn CommandDialogProvider(children: Children, #[prop(into)] id: String) -> impl IntoView {
    let context = CommandDialogContext { dialog_id: id };

    provide_context(context);

    children()
}

#[component]
pub fn CommandDialogTrigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let context = expect_context::<CommandDialogContext>();
    let trigger_id = format!("{TRIGGER_ID_QUALIFIER}__{}", context.dialog_id);

    view! {
        <Button attr:data-name="CommandDialogTrigger" class=class variant=ButtonVariant::Outline attr:id=trigger_id>
            {children()}
        </Button>
    }
}

#[component]
pub fn CommandDialog(children: ChildrenFn, #[prop(into, optional)] class: String) -> impl IntoView {
    let context = expect_context::<CommandDialogContext>();
    let merged_class = tw_merge!(
        "grid fixed z-100 gap-4 p-2 w-full bg-clip-padding rounded-xl border border-none ring-4 shadow-2xl sm:max-w-lg bg-background top-[50%] left-[50%] max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] ring-neutral-200/80 transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class
    );

    let dialog_id = context.dialog_id;
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

                // Remove old listeners if they exist (for SPA navigation)
                if (window[KEY_HANDLER_KEY]) {{
                    document.removeEventListener('keydown', window[KEY_HANDLER_KEY]);
                }}
                if (window[CLICK_HANDLER_KEY]) {{
                    document.removeEventListener('click', window[CLICK_HANDLER_KEY]);
                }}
                if (window[BACKDROP_HANDLER_KEY]) {{
                    document.removeEventListener('click', window[BACKDROP_HANDLER_KEY]);
                }}

                // Click handler using event delegation (works after SPA navigation)
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

                // Backdrop click handler using event delegation
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

                // Global keyboard listener for Cmd+K, Ctrl+K, or /
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

                // Store handler references and add listeners
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

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <CommandDialogPortal dialog_id=dialog_id backdrop_id=backdrop_id class=merged_class>
            {children()}
        </CommandDialogPortal>

        <script>{script_content}</script>
    }
}

// Renders backdrop and dialog using Leptos Portal to body
#[component]
fn CommandDialogPortal(
    children: ChildrenFn,
    #[prop(into)] dialog_id: String,
    #[prop(into)] backdrop_id: String,
    #[prop(into)] class: String,
) -> impl IntoView {
    let backdrop_id = StoredValue::new(backdrop_id);
    let dialog_id = StoredValue::new(dialog_id);
    let class = StoredValue::new(class);

    view! {
        <Portal>
            <div
                data-name="CommandDialogBackdrop"
                id=backdrop_id.get_value()
                class="fixed inset-0 transition-opacity duration-200 pointer-events-none z-60 bg-black/50 data-[state=closed]:opacity-0 data-[state=open]:opacity-100"
                data-state="closed"
            />

            <div
                data-name="CommandDialog"
                class=class.get_value()
                id=dialog_id.get_value()
                data-state="closed"
                tabindex="-1"
                style="pointer-events: none;"
            >
                {children()}
            </div>
        </Portal>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Command(
    children: Children,
    #[prop(into, optional)] class: String,
    /// When false, disables client-side filtering (use for server-side search).
    /// Default: true (client-side filtering enabled).
    #[prop(default = true)]
    should_filter: bool,
) -> impl IntoView {
    let dialog_context = use_context::<CommandDialogContext>();
    let search_query_signal = RwSignal::new(String::new());
    let command_context = CommandContext { search_query_signal, should_filter };

    provide_context(command_context);

    let merged_class = tw_merge!(
        "flex overflow-hidden flex-col w-full h-full bg-transparent rounded-none text-popover-foreground",
        class
    );

    let script_content = if let Some(ctx) = dialog_context {
        // Dialog version with context
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
                        // Elements not ready yet, try again shortly
                        setTimeout(setupCommandKeyboard, 50);
                        return;
                    }}

                    let index = FIRST_INDEX;

                    // Get visible items only (also skip items inside a hidden group)
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

                    // Filter items based on search query
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

                        // Hide empty groups
                        command_groups.forEach(group => {{
                            const groupItems = group.querySelectorAll('[data-name="CommandItemLink"]');
                            const hasVisibleItems = Array.from(groupItems).some(item => item.style.display !== 'none');
                            group.style.display = hasVisibleItems ? '' : 'none';
                        }});

                        // Reset selection to first visible item
                        index = FIRST_INDEX;
                        select(FIRST_INDEX);
                    }};

                    // Listen to input changes
                    command_input.addEventListener('input', (e) => {{
                        filterItems(e.target.value);
                    }});

                    // Close dialog function
                    // NOTE: We DON'T handle body scroll restoration here.
                    // That's handled by CommandDialog's closeDialog function.
                    // We just trigger the dialog close, and the backdrop click handler will do the rest.
                    const closeDialog = () => {{
                        backdrop.click();
                    }};

                    // Add click handlers to all command items to close dialog
                    command_items.forEach((item) => {{
                        item.addEventListener('click', () => {{
                            closeDialog();
                        }});
                    }});

                    document.addEventListener('keydown', (e) => {{
                        // Only handle keyboard navigation if dialog is open
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

                    // Initialize selection only when dialog opens
                    const observer = new MutationObserver((mutations) => {{
                        mutations.forEach((mutation) => {{
                            if (mutation.attributeName === 'data-state') {{
                                if (dialog.getAttribute('data-state') === 'open') {{
                                    // Reset search input and filter
                                    command_input.value = '';
                                    filterItems('');
                                    index = FIRST_INDEX;
                                    select(FIRST_INDEX);
                                    // Focus the input when dialog opens (needed only on click the Trigger)
                                    setTimeout(() => command_input.focus(), 0);
                                }}
                            }}
                        }});
                    }});

                    observer.observe(dialog, {{ attributes: true }});
                }};

                // Try to setup immediately, or wait for DOMContentLoaded
                if (document.readyState === 'loading') {{
                    document.addEventListener('DOMContentLoaded', setupCommandKeyboard);
                }} else {{
                    setupCommandKeyboard();
                }}
            }})();
            "#
        )
    } else {
        // Standalone version without dialog context - always active keyboard navigation
        r#"
        (function() {
            const setupCommand = () => {
                const FIRST_INDEX = 0;
                const command_list = document.querySelector('[data-name="CommandList"]');
                const command_input = document.querySelector('[data-name="CommandInput"]');
                const command_items = command_list?.querySelectorAll('[data-name="CommandItemLink"]');
                const command_groups = command_list?.querySelectorAll('[data-name="CommandGroup"]');

                if (!command_items || command_items.length === 0) {
                    // Elements not ready yet, try again shortly
                    setTimeout(setupCommand, 50);
                    return;
                }

                let index = FIRST_INDEX;

                // Get visible items only
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

                // Filter items based on search query
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

                    // Hide empty groups
                    if (command_groups) {
                        command_groups.forEach(group => {
                            const groupItems = group.querySelectorAll('[data-name="CommandItemLink"]');
                            const hasVisibleItems = Array.from(groupItems).some(item => item.style.display !== 'none');
                            group.style.display = hasVisibleItems ? '' : 'none';
                        });
                    }

                    // Reset selection to first visible item
                    index = FIRST_INDEX;
                    select(FIRST_INDEX);
                };

                // Listen to input changes if input exists
                if (command_input) {
                    command_input.addEventListener('input', (e) => {
                        filterItems(e.target.value);
                    });
                }

                // Initialize first item as selected
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

            // Try to setup immediately, or wait for DOMContentLoaded
            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', setupCommand);
            } else {
                setupCommand();
            }
        })();
        "#
        .to_string()
    };

    view! {
        <style>
            r#"
            /* Command component - aria-selected styling */
            [data-name="CommandItemLink"][aria-selected="true"],
            [data-name="CommandItem"][aria-selected="true"] {
                background-color: var(--color-muted);
            }
            /* Hide CommandEmpty when there are visible items */
            [data-name="CommandList"]:has([data-name="CommandItem"][style*="flex"]) [data-name="CommandEmpty"] {
                display: none;
            }
            "#
        </style>

        <div data-name="Command" class=merged_class tabindex="-1">
            // <label style="position: absolute; width: 1px; height: 1px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0px, 0px, 0px, 0px); white-space: nowrap; border-width: 0px;"></label>
            {children()}
        </div>

        <script>{script_content}</script>
    }
}

#[component]
pub fn CommandInput(
    #[prop(into, optional)] class: String,
    /// Callback fired when search input changes. Use for server-side search.
    #[prop(optional)]
    on_search_change: Option<Callback<String>>,
) -> impl IntoView {
    let command_context = expect_context::<CommandContext>();
    let merged_class = tw_merge!(
        "flex py-3 w-full h-10 text-sm bg-transparent rounded-md disabled:opacity-50 disabled:cursor-not-allowed placeholder:text-muted-foreground outline-hidden",
        class
    );

    view! {
        <input
            data-name="CommandInput"
            class=merged_class
            autocomplete="off"
            // TODO. Leptos does not seem to have autocorrect in keys.rs
            // autocorrect="off"
            spellcheck="false"
            aria-autocomplete="list"
            role="combobox"
            aria-expanded="true"
            aria-controls="command_demo"
            aria-label="Search documentation"
            type="text"
            prop:value=move || command_context.search_query_signal.get()
            on:input=move |ev| {
                let value = event_target_value(&ev);
                command_context.search_query_signal.set(value.clone());
                if let Some(callback) = on_search_change {
                    callback.run(value);
                }
            }
            autofocus="true"
            // Prevent password managers from showing up
            data-1p-ignore="true"
            data-bwignore="true"
            data-lpignore="true"
        />
    }
}

#[component]
pub fn CommandEmpty(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("py-6 text-sm text-center", class);

    view! {
        <div data-name="CommandEmpty" class=merged_class>
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItem(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] value: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    #[prop(default = false.into(), into)] selected: Signal<bool>,
    /// Reserve space for check icon even when not selected (for alignment)
    #[prop(default = false)]
    reserve_check_space: bool,
) -> impl IntoView {
    let command_context = expect_context::<CommandContext>();
    let value_for_filter = value;

    let merged_class = tw_merge!(
        "group relative flex gap-2 items-center px-2 py-1.5 text-sm rounded-sm cursor-default select-none outline-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 hover:bg-accent hover:text-accent-foreground",
        class
    );

    let is_visible = Memo::new(move |_| {
        // Skip client-side filtering when should_filter is false (server-side search)
        if !command_context.should_filter {
            return true;
        }

        let search = command_context.search_query_signal.get().to_lowercase();
        if search.is_empty() {
            return true;
        }
        value_for_filter.to_lowercase().contains(&search)
    });

    // Check icon class: always visible space when reserve_check_space, otherwise hidden when not selected
    let check_class = if reserve_check_space {
        "ml-auto size-4 text-muted-foreground opacity-0 group-aria-selected:opacity-100"
    } else {
        "ml-auto size-4 text-muted-foreground hidden group-aria-selected:block"
    };

    view! {
        <div
            data-name="CommandItem"
            class=merged_class
            role="option"
            tabindex="0"
            aria-selected=move || selected.get().to_string()
            style:display=move || if is_visible.get() { "flex" } else { "none" }
            on:click=move |_| {
                if let Some(callback) = on_select {
                    callback.run(());
                }
            }
        >
            {children()}
            <Check class=check_class />
        </div>
    }
}
