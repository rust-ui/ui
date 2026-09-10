use dioxus::prelude::*;
use icons::{Anchor, ArrowDown, ArrowRight, ArrowUp, CircleDashed, CornerDownLeft, Search};
use registry::ui::command::{
    Command, CommandDescription, CommandDialog, CommandDialogProvider, CommandDialogTrigger, CommandFooter,
    CommandGroup, CommandGroupLabel, CommandHeader, CommandInput, CommandItemLink, CommandList, CommandTitle,
};
use registry::ui::input_group::{InputGroup, InputGroupAddon};
use registry::ui::kbd::Kbd;

use crate::__registry__::my_command_bar_constants::{COMPONENTS_ITEMS, CommandCategory, HOOKS_ITEMS, PAGES_ITEMS};

const DIALOG_ID: &str = "command-search-docs";

const TAB_CLASS: &str = "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground";

/// Trigger only. Rendered in every header. Opens the single global dialog
/// (`CommandSearchDocsDialog`) purely through DOM ids + delegated listeners,
/// so it needs no shared Dioxus state with the dialog.
#[component]
pub fn CommandSearchDocs() -> Element {
    rsx! {
        CommandDialogProvider { id: DIALOG_ID,
            CommandDialogTrigger { class: "flex-1 justify-start pl-3 h-8 text-sm font-normal shadow-none md:flex-none",
                span { class: "hidden md:inline-flex", "Search..." }
                span { class: "inline-flex md:hidden", "Search documentation..." }
                kbd {
                    class: "flex gap-1 items-center px-1.5 h-5 font-mono font-medium rounded border opacity-100 pointer-events-none select-none bg-muted text-[10px]",
                    span { class: "text-xs", "⌘" }
                    span { "K" }
                }
            }
        }
    }
}

/// Portal-equivalent for Dioxus 0.7 (which has no `Portal` primitive, unlike
/// the leptos site's `leptos::portal::Portal`). Mounted once from `AppLayout`,
/// outside the blurred `<header>` whose `backdrop-filter` would otherwise
/// become the containing block for the dialog's `position: fixed` and trap it
/// near the top of the header instead of centering it in the viewport.
#[component]
pub fn CommandSearchDocsDialog() -> Element {
    rsx! {
        CommandDialogProvider { id: DIALOG_ID,
            CommandDialog {
                CommandHeader {
                    CommandTitle { "Search documentation..." }
                    CommandDescription { "Search for a command to run..." }
                }
                Command {
                    InputGroup { class: "h-9 bg-input/50",
                        InputGroupAddon {
                            Search {}
                        }
                        CommandInput {
                            placeholder: "Search documentation...",
                            class: "flex-1 py-0 h-9 rounded-none border-0 shadow-none",
                        }
                    }
                    div { class: "flex gap-1 px-3 pt-1 pb-2 border-b border-border", "data-name": "CommandTabBar",
                        button { "data-tab": "all", "data-active": "true", class: TAB_CLASS, "All" }
                        button { "data-tab": "pages", "data-active": "false", class: TAB_CLASS, "Pages" }
                        button { "data-tab": "components", "data-active": "false", class: TAB_CLASS, "Components" }
                        button { "data-tab": "hooks", "data-active": "false", class: TAB_CLASS, "Hooks" }
                    }

                    CommandList { id: "command_demo", tabindex: "-1",
                        for (category , items) in [
                            (CommandCategory::Pages, PAGES_ITEMS),
                            (CommandCategory::Components, COMPONENTS_ITEMS),
                            (CommandCategory::Hooks, HOOKS_ITEMS),
                        ] {
                            CommandGroup {
                                role: "presentation",
                                class: "p-0",
                                data_category: category.slug(),
                                CommandGroupLabel { aria_hidden: "true", class: "p-3", "{category.as_str()}" }
                                for item in items.iter().copied() {
                                    CommandItemLink {
                                        class: "px-3",
                                        href: item.href,
                                        data_add_cmd: item.add_cmd.unwrap_or(""),
                                        CategoryIcon { category: item.category }
                                        span { "{item.label}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CommandFooter {
                    div { class: "flex gap-2 items-center",
                        Kbd { ArrowUp {} }
                        Kbd { ArrowDown {} }
                        span { "Navigate" }
                    }
                    div { class: "flex gap-2 items-center",
                        Kbd { CornerDownLeft {} }
                        span { "Go to Page" }
                    }
                    div { id: "cmd-copy-footer", class: "flex gap-2 items-center ml-auto", style: "display: none;",
                        Kbd { "⌘" }
                        Kbd { "C" }
                        span { id: "cmd-copy-label" }
                    }
                }
            }
        }

        script { dangerous_inner_html: PAGE_SCRIPT }
    }
}

#[component]
fn CategoryIcon(category: CommandCategory) -> Element {
    rsx! {
        match category {
            CommandCategory::Pages => rsx! { ArrowRight {} },
            CommandCategory::Components => rsx! { CircleDashed {} },
            CommandCategory::Hooks => rsx! { Anchor {} },
        }
    }
}

const PAGE_SCRIPT: &str = r#"
(function() {
    const setup = () => {
        const dialog = document.querySelector('#command-search-docs');
        const copyFooter = document.getElementById('cmd-copy-footer');
        const copyLabel = document.getElementById('cmd-copy-label');

        if (!dialog || !copyFooter || !copyLabel) {
            setTimeout(setup, 100);
            return;
        }

        // Tab switching
        const tabButtons = dialog.querySelectorAll('[data-name="CommandTabBar"] [data-tab]');
        const groups = dialog.querySelectorAll('[data-name="CommandGroup"][data-category]');

        const applyTab = (tab) => {
            tabButtons.forEach(btn => {
                btn.setAttribute('data-active', btn.dataset.tab === tab ? 'true' : 'false');
            });
            groups.forEach(group => {
                group.style.display = (tab === 'all' || group.dataset.category === tab) ? '' : 'none';
            });
            // Reset search so filterItems re-runs for the new visible set
            const input = dialog.querySelector('[data-name="CommandInput"]');
            if (input && input.value !== '') {
                input.value = '';
                input.dispatchEvent(new Event('input'));
            }
        };

        tabButtons.forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.preventDefault();
                applyTab(btn.dataset.tab);
            });
        });

        // Copy hint
        const updateCopyHint = () => {
            const selected = dialog.querySelector('[data-name="CommandItemLink"][aria-selected="true"]');
            const slug = selected?.getAttribute('data-add-cmd');
            if (slug) {
                copyLabel.textContent = 'ui add ' + slug;
                copyFooter.style.display = '';
            } else {
                copyFooter.style.display = 'none';
            }
        };

        const ariaObserver = new MutationObserver((mutations) => {
            for (const m of mutations) {
                if (m.attributeName === 'aria-selected') {
                    updateCopyHint();
                    break;
                }
            }
        });

        const items = dialog.querySelectorAll('[data-name="CommandItemLink"]');
        items.forEach(item => {
            ariaObserver.observe(item, { attributes: true, attributeFilter: ['aria-selected'] });

            item.addEventListener('mouseenter', () => {
                const slug = item.getAttribute('data-add-cmd');
                if (slug) {
                    copyLabel.textContent = 'ui add ' + slug;
                    copyFooter.style.display = '';
                } else {
                    copyFooter.style.display = 'none';
                }
            });

            item.addEventListener('mouseleave', () => {
                updateCopyHint();
            });
        });

        updateCopyHint();

        document.addEventListener('keydown', (e) => {
            if (dialog.getAttribute('data-state') !== 'open') return;
            if ((e.metaKey || e.ctrlKey) && e.key === 'c') {
                const selected = dialog.querySelector('[data-name="CommandItemLink"][aria-selected="true"]');
                const slug = selected?.getAttribute('data-add-cmd');
                if (slug) {
                    e.preventDefault();
                    const cmd = 'ui add ' + slug;
                    navigator.clipboard.writeText(cmd).catch(() => {});
                    const orig = copyLabel.textContent;
                    copyLabel.textContent = 'Copied!';
                    setTimeout(() => { copyLabel.textContent = orig; }, 1500);
                }
            }
        });

        // Reset tab to "All" when dialog opens
        const dialogObserver = new MutationObserver((mutations) => {
            for (const m of mutations) {
                if (m.attributeName === 'data-state' && dialog.getAttribute('data-state') === 'open') {
                    applyTab('all');
                    break;
                }
            }
        });
        dialogObserver.observe(dialog, { attributes: true, attributeFilter: ['data-state'] });
    };

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setup);
    } else {
        setup();
    }
})();
"#;
