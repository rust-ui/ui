use icons::{ArrowDown, ArrowUp, CornerDownLeft, Search};
use leptos::prelude::*;
use registry::ui::command::{
    Command, CommandDescription, CommandDialog, CommandDialogProvider, CommandDialogTrigger, CommandFooter,
    CommandGroup, CommandGroupLabel, CommandHeader, CommandInput, CommandItemLink, CommandList, CommandTitle,
};
use registry::ui::input_group::{InputGroup, InputGroupAddon};
use registry::ui::kbd::Kbd;

use crate::__registry__::my_command_bar_constants::{COMPONENTS_ITEMS, CommandCategory, HOOKS_ITEMS, PAGES_ITEMS};

#[component]
pub fn CommandSearchDocs() -> impl IntoView {
    view! {
        <CommandDialogProvider id="command-search-docs">
            <CommandDialogTrigger class="flex-1 justify-start pl-3 h-8 text-sm font-normal shadow-none md:flex-none md:w-48 lg:w-40 xl:w-64">
                <span class="hidden md:inline-flex">Search docs...</span>
                <span class="inline-flex md:hidden">Search documentation...</span>
                <kbd class="flex gap-1 items-center px-1.5 h-5 font-mono font-medium rounded border opacity-100 pointer-events-none select-none bg-muted text-[10px]">
                    <span class="text-xs">"⌘"</span>
                    <span>"K"</span>
                </kbd>
            </CommandDialogTrigger>

            <CommandDialog>
                <CommandHeader>
                    <CommandTitle>Search documentation...</CommandTitle>
                    <CommandDescription>Search for a command to run...</CommandDescription>
                </CommandHeader>
                <Command>
                    <InputGroup class="h-9 bg-input/50">
                        <InputGroupAddon>
                            <Search />
                        </InputGroupAddon>
                        <CommandInput
                            attr:placeholder="Search documentation..."
                            class="flex-1 py-0 h-9 rounded-none border-0 shadow-none"
                        />
                    </InputGroup>
                    <div class="flex gap-1 px-3 pt-1 pb-2 border-b border-border" data-name="CommandTabBar">
                        <button
                            data-tab="all"
                            data-active="true"
                            class="py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground"
                        >
                            "All"
                        </button>
                        <button
                            data-tab="pages"
                            data-active="false"
                            class="py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground"
                        >
                            "Pages"
                        </button>
                        <button
                            data-tab="components"
                            data-active="false"
                            class="py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground"
                        >
                            "Components"
                        </button>
                        <button
                            data-tab="hooks"
                            data-active="false"
                            class="py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground"
                        >
                            "Hooks"
                        </button>
                    </div>

                    <CommandList attr:id="command_demo" attr:tabindex="-1">
                        {[
                            (CommandCategory::Pages, PAGES_ITEMS),
                            (CommandCategory::Components, COMPONENTS_ITEMS),
                            (CommandCategory::Hooks, HOOKS_ITEMS),
                        ]
                            .into_iter()
                            .map(|(category, items)| {
                                let slug = category.to_string().to_lowercase();
                                view! {
                                    <CommandGroup attr:role="presentation" class="p-0" attr:data-category=slug>
                                        <CommandGroupLabel attr:aria-hidden="true" class="p-3">
                                            {category.to_string()}
                                        </CommandGroupLabel>
                                        {items
                                            .iter()
                                            .map(|item| {
                                                let icon = item.to_icon();
                                                let add_cmd = item.add_cmd.unwrap_or("");
                                                view! {
                                                    <CommandItemLink
                                                        class="px-3"
                                                        attr:href=item.href
                                                        attr:data-add-cmd=add_cmd
                                                    >
                                                        {icon}
                                                        <span>{item.label}</span>
                                                    </CommandItemLink>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </CommandGroup>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </CommandList>
                </Command>
                <CommandFooter>
                    <div class="flex gap-2 items-center">
                        <Kbd>
                            <ArrowUp />
                        </Kbd>
                        <Kbd>
                            <ArrowDown />
                        </Kbd>
                        <span>Navigate</span>
                    </div>
                    <div class="flex gap-2 items-center">
                        <Kbd>
                            <CornerDownLeft />
                        </Kbd>
                        <span>Go to Page</span>
                    </div>
                    <div id="cmd-copy-footer" class="flex gap-2 items-center ml-auto" style="display: none;">
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"C"</Kbd>
                        <span id="cmd-copy-label"></span>
                    </div>
                </CommandFooter>
            </CommandDialog>
        </CommandDialogProvider>

        <script>
            r#"
            (function() {
                const setup = () => {
                    const dialog = document.querySelector('#command-search-docs');
                    const copyFooter = document.getElementById('cmd-copy-footer');
                    const copyLabel = document.getElementById('cmd-copy-label');
            
                    if (!dialog || !copyFooter || !copyLabel) {
                        setTimeout(setup, 100);
                        return;
                    }
            
                    // ── Tab switching ────────────────────────────────────────
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
            
                    // ── Copy hint ────────────────────────────────────────────
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
            
                    // ── Reset tab to "All" when dialog opens ─────────────────
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
            "#
        </script>
    }
}
