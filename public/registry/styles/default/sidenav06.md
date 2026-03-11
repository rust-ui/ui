---
title: "Sidenav06"
name: "sidenav06"
cargo_dependencies: []
registry_dependencies: ["button", "dropdown_menu", "sheet", "sidenav"]
type: "components:blocks"
path: "blocks/sidenav06.rs"
---

# Sidenav06

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add sidenav06
```

## Component Code

```rust
use icons::{ChevronRight, LayoutTemplate, PanelLeft, Sparkles};
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::components::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::components::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::components::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::components::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};
use crate::components::ui::sheet::{Sheet, SheetContent, SheetDirection, SheetTrigger};
use crate::components::ui::sidenav::{
    Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavGroupLabel, SidenavHeader,
    SidenavMenu, SidenavMenuItem, SidenavWrapper,
};

#[component(transparent)]
pub fn Sidenav06Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment(SidenavRoutes::view_segment()) view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav06.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav06 /> }
            >
                <SidenavRoutesSimplified />
            </ParentRoute>
        </ParentRoute>
    }
    .into_inner()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn SidenavLayout(sidenav_route: SidenavRoutes) -> impl IntoView {
    let location = use_location();

    let current_section = Memo::new(move |_| {
        let path = location.pathname.get();
        if path.contains(DocsRoutes::Components.as_ref()) { DocsRoutes::Components } else { DocsRoutes::Hooks }
    });

    view! {
        <div class="bg-background">
            <SidenavWrapper attr:style="--sidenav-width:16rem;">
                <Sidenav>
                    <Sidenav06Content current_section sidenav_route />
                </Sidenav>
                <Outlet />
            </SidenavWrapper>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ SHARED CONTENT ✨                   */
/* ========================================================== */

/// Shared sidenav content used by both desktop Sidenav and mobile Sheet
#[component]
fn Sidenav06Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    view! {
        <SidenavHeader>
            <SidenavRoutesSelector current_section sidenav_route />
        </SidenavHeader>

        <SidenavContent>
            <SidenavGroup>
                <SidenavGroupLabel>"Navigation"</SidenavGroupLabel>
                <SidenavGroupContent>
                    <SidenavMenu>
                        // * Components dropdown submenu
                        <SidenavMenuItem>
                            <DropdownMenu align=DropdownMenuAlign::End>
                                <DropdownMenuTrigger class="flex overflow-hidden gap-2 items-center p-2 w-full h-8 text-sm text-left rounded-md focus-visible:ring-2 peer/menu-button ring-sidenav-ring outline-hidden transition-[width,height,padding] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground">
                                    <LayoutTemplate class="size-4 shrink-0" />
                                    <span class="flex-1 truncate">"Components"</span>
                                    <ChevronRight class="opacity-50 size-4 shrink-0" />
                                </DropdownMenuTrigger>
                                <DropdownMenuContent>
                                    <DropdownMenuGroup>
                                        {COMPONENT_LINKS
                                            .iter()
                                            .map(|(href, title)| {
                                                view! {
                                                    <DropdownMenuItem>
                                                        <DropdownMenuAction href=*href>{*title}</DropdownMenuAction>
                                                    </DropdownMenuItem>
                                                }
                                            })
                                            .collect_view()}
                                    </DropdownMenuGroup>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </SidenavMenuItem>

                        // * Hooks dropdown submenu
                        <SidenavMenuItem>
                            <DropdownMenu align=DropdownMenuAlign::End>
                                <DropdownMenuTrigger class="flex overflow-hidden gap-2 items-center p-2 w-full h-8 text-sm text-left rounded-md focus-visible:ring-2 peer/menu-button ring-sidenav-ring outline-hidden transition-[width,height,padding] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground">
                                    <Sparkles class="size-4 shrink-0" />
                                    <span class="flex-1 truncate">"Hooks"</span>
                                    <ChevronRight class="opacity-50 size-4 shrink-0" />
                                </DropdownMenuTrigger>
                                <DropdownMenuContent>
                                    <DropdownMenuGroup>
                                        {HOOKS_LINKS
                                            .iter()
                                            .map(|(href, title)| {
                                                view! {
                                                    <DropdownMenuItem>
                                                        <DropdownMenuAction href=*href>{*title}</DropdownMenuAction>
                                                    </DropdownMenuItem>
                                                }
                                            })
                                            .collect_view()}
                                    </DropdownMenuGroup>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </SidenavMenuItem>
                    </SidenavMenu>
                </SidenavGroupContent>
            </SidenavGroup>
        </SidenavContent>

        <SidenavFooter>
            <DemoDropdownMenuUser />
        </SidenavFooter>
    }
}

/* ========================================================== */
/*                     ✨ MOBILE SHEET ✨                     */
/* ========================================================== */

#[component]
pub fn Sidenav06MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    view! {
        <Sheet>
            <div class="md:hidden">
                <SheetTrigger class="size-7" variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                    <PanelLeft class="size-4" />
                    <span class="hidden">"Toggle Sidenav"</span>
                </SheetTrigger>
            </div>
            <SheetContent
                direction=SheetDirection::Left
                class="p-0 w-[18rem] bg-sidenav text-sidenav-foreground"
                show_close_button=false
            >
                <div class="flex flex-col h-full">
                    <Sidenav06Content current_section sidenav_route />
                </div>
            </SheetContent>
        </Sheet>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const COMPONENT_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav06/docs/components/accordion", "Accordion"),
    ("/view/sidenav06/docs/components/alert", "Alert"),
    ("/view/sidenav06/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav06/docs/components/button", "Button"),
    ("/view/sidenav06/docs/components/card", "Card"),
    ("/view/sidenav06/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav06/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav06/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav06/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav06/docs/hooks/use-random", "Use Random"),
];
```
