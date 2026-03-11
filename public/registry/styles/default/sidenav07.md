---
title: "Sidenav07"
name: "sidenav07"
cargo_dependencies: []
registry_dependencies: ["accordion", "button", "sheet", "sidenav"]
type: "components:blocks"
path: "blocks/sidenav07.rs"
---

# Sidenav07

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add sidenav07
```

## Component Code

```rust
use icons::{BookOpen, Bot, Ellipsis, Frame, PanelLeft, Settings, SquareTerminal};
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::components::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::components::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::components::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::components::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::components::ui::accordion::{AccordionContent, AccordionHeader, AccordionItem, AccordionTitle, AccordionTrigger};
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::components::ui::sidenav::{
    DropdownMenuTriggerEllipsis, Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent,
    SidenavGroupLabel, SidenavHeader, SidenavLink, SidenavMenu, SidenavMenuItem, SidenavMenuSub, SidenavWrapper,
};

#[component(transparent)]
pub fn Sidenav07Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment(SidenavRoutes::view_segment()) view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav07.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav07 /> }
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

// * ---> Layout with [📍 OUTLET] for nested routing (SidenavInset).
#[component]
pub fn SidenavLayout(sidenav_route: SidenavRoutes) -> impl IntoView {
    let location = use_location();

    // Determine current section from URL
    let current_section = Memo::new(move |_| {
        let path = location.pathname.get();
        if path.contains(DocsRoutes::Components.as_ref()) { DocsRoutes::Components } else { DocsRoutes::Hooks }
    });

    view! {
        <div class="bg-background">
            <SidenavWrapper attr:style="--sidenav-width:16rem;">
                // * Desktop Sidenav (hidden on mobile)
                <Sidenav>
                    <Sidenav07Content current_section sidenav_route />
                </Sidenav>

                // * ---> [📍 OUTLET] for nested routing (SidenavInset).
                // * Mobile Sheet is rendered in SidenavInsetRight header
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
fn Sidenav07Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    // Try to get SheetContext - will be Some when inside a Sheet (mobile), None otherwise (desktop)
    let sheet_ctx = use_context::<SheetContext>();

    view! {
        <SidenavHeader>
            <SidenavRoutesSelector current_section sidenav_route />
        </SidenavHeader>

        <SidenavContent>
            {move || {
                let sheet_target_id = sheet_ctx.as_ref().map(|ctx| ctx.target_id.clone());
                let links = match current_section.get() {
                    DocsRoutes::Components => COMPONENT_LINKS,
                    DocsRoutes::Hooks => HOOKS_LINKS,
                };

                view! {
                    <SidenavGroup>
                        <SidenavGroupContent>
                            <SidenavGroupLabel>{move || current_section.get().to_title()}</SidenavGroupLabel>
                            <SidenavMenu>
                                <SidenavMenuItem>
                                    <AccordionItem>
                                        <AccordionTrigger open=true class="p-2 peer-checked:bg-accent hover:bg-accent">
                                            <AccordionHeader>
                                                <BookOpen />
                                                <AccordionTitle>
                                                    {move || current_section.get().to_title()}
                                                </AccordionTitle>
                                            </AccordionHeader>
                                        </AccordionTrigger>
                                        <AccordionContent class="p-0">
                                            <SidenavMenuSub>
                                                {links
                                                    .iter()
                                                    .map(|(href, link_title)| {
                                                        if let Some(ref target_id) = sheet_target_id {
                                                            // Wrap in div with data-sheet-close when inside a Sheet (mobile)
                                                            view! {
                                                                <div data-sheet-close=target_id.clone()>
                                                                    <SidenavLink href=*href>{*link_title}</SidenavLink>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! {
                                                                <SidenavLink href=*href>{*link_title}</SidenavLink>
                                                            }
                                                                .into_any()
                                                        }
                                                    })
                                                    .collect_view()}
                                            </SidenavMenuSub>
                                        </AccordionContent>
                                    </AccordionItem>
                                </SidenavMenuItem>

                                <SidenavMenuItem>
                                    <AccordionItem>
                                        <AccordionTrigger class="p-2 peer-checked:bg-accent hover:bg-accent">
                                            <AccordionHeader>
                                                <Bot />
                                                <AccordionTitle>Models</AccordionTitle>
                                            </AccordionHeader>
                                        </AccordionTrigger>
                                        <AccordionContent class="p-0">
                                            <SidenavMenuSub>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                            </SidenavMenuSub>
                                        </AccordionContent>
                                    </AccordionItem>
                                </SidenavMenuItem>
                                <SidenavMenuItem>
                                    <AccordionItem>
                                        <AccordionTrigger class="p-2 peer-checked:bg-accent hover:bg-accent">
                                            <AccordionHeader>
                                                <SquareTerminal />
                                                <AccordionTitle>Playground</AccordionTitle>
                                            </AccordionHeader>
                                        </AccordionTrigger>
                                        <AccordionContent class="p-0">
                                            <SidenavMenuSub>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                            </SidenavMenuSub>
                                        </AccordionContent>
                                    </AccordionItem>
                                </SidenavMenuItem>
                                <SidenavMenuItem>
                                    <AccordionItem>
                                        <AccordionTrigger class="p-2 peer-checked:bg-accent hover:bg-accent">
                                            <AccordionHeader>
                                                <Settings />
                                                <AccordionTitle>Settings</AccordionTitle>
                                            </AccordionHeader>
                                        </AccordionTrigger>
                                        <AccordionContent class="p-0">
                                            <SidenavMenuSub>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                                <SidenavLink href="#">Link</SidenavLink>
                                            </SidenavMenuSub>
                                        </AccordionContent>
                                    </AccordionItem>
                                </SidenavMenuItem>
                            </SidenavMenu>
                        </SidenavGroupContent>
                    </SidenavGroup>
                }
            }} <SidenavGroup>
                <SidenavGroupLabel>Projects</SidenavGroupLabel>
                <SidenavMenu>
                    <ListExampleProjects />
                    <ListExampleProjects />
                </SidenavMenu>
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

/// Mobile sheet containing sidenav content - uses the proper Sheet component
#[component]
pub fn Sidenav07MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    view! {
        <Sheet>
            // * Trigger visible only on mobile, positioned in header flow
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
                    <Sidenav07Content current_section sidenav_route />
                </div>
            </SheetContent>
        </Sheet>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

// * A simple example with basic links.

const COMPONENT_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav07/docs/components/accordion", "Accordion"),
    ("/view/sidenav07/docs/components/alert", "Alert"),
    ("/view/sidenav07/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav07/docs/components/button", "Button"),
    ("/view/sidenav07/docs/components/card", "Card"),
    ("/view/sidenav07/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav07/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav07/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav07/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav07/docs/hooks/use-random", "Use Random"),
];

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

// Projects
#[component]
fn ListExampleProjects() -> impl IntoView {
    const SHARED_CLASS_PEER_MENU_BUTTON: &str = "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left outline-hidden ring-sidenav-ring transition-[width,height,padding] focus-visible:ring-2 active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 data-[active=true]:bg-sidenav-accent data-[active=true]:font-medium data-[active=true]:text-sidenav-accent-foreground data-[state=open]:hover:bg-sidenav-accent data-[state=open]:hover:text-sidenav-accent-foreground group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 hover:bg-sidenav-accent hover:text-sidenav-accent-foreground h-8 text-sm";

    view! {
        <SidenavMenuItem>
            <a
                href="#"
                data-name="sidenav-menu-button"
                data-sidenav="menu-button"
                data-size="default"
                class=SHARED_CLASS_PEER_MENU_BUTTON
            >
                <Frame />
                <span>Design Engineering</span>
            </a>

            <DropdownMenuTriggerEllipsis attr:aria-haspopup="menu" attr:aria-expanded="false" attr:data-state="closed">
                <Ellipsis />
                <span class="hidden">More</span>
            </DropdownMenuTriggerEllipsis>
        </SidenavMenuItem>
    }
}
```
