---
title: "Sidenav Routes Selector"
name: "sidenav_routes_selector"
cargo_dependencies: []
registry_dependencies: ["dropdown_menu"]
type: "components:blocks"
path: "blocks/sidenav_routes_selector.rs"
---

# Sidenav Routes Selector

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add sidenav_routes_selector
```

## Component Code

```rust
use icons::{ChevronsUpDown, LayoutTemplate, Sparkles};
use leptos::prelude::*;

use super::sidenav_routes::{ComponentsRoutes, DocsRoutes, HooksRoutes, SidenavRoutes};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};

#[component]
pub fn SidenavRoutesSelector(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    let docs_routes = [DocsRoutes::Components, DocsRoutes::Hooks];

    view! {
        <DropdownMenu align=DropdownMenuAlign::Center>
            <DropdownMenuTrigger class="flex justify-between px-2 w-full h-12 bg-transparent border-0">
                <div class="flex gap-2 items-center">
                    <div class="flex justify-center items-center rounded-lg bg-primary text-primary-foreground aspect-square size-8">
                        {move || match current_section.get() {
                            DocsRoutes::Components => view! { <LayoutTemplate /> }.into_any(),
                            DocsRoutes::Hooks => view! { <Sparkles /> }.into_any(),
                        }}
                    </div>

                    <div class="grid flex-1 text-sm leading-tight text-left">
                        <span class="font-medium">"Docs"</span>
                        <span class="text-xs">{move || current_section.get().to_title()}</span>
                    </div>
                </div>

                <ChevronsUpDown />
            </DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuGroup>
                    {docs_routes
                        .into_iter()
                        .map(|doc_route| {
                            view! {
                                <DropdownMenuItem>
                                    <DropdownMenuAction href=match doc_route {
                                        DocsRoutes::Components => ComponentsRoutes::base_url_with_sidenav(sidenav_route),
                                        DocsRoutes::Hooks => HooksRoutes::base_url_with_sidenav(sidenav_route),
                                    }>{doc_route.to_title()}</DropdownMenuAction>
                                </DropdownMenuItem>
                            }
                        })
                        .collect_view()}
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
```
