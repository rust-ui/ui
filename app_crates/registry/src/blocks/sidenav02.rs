use icons::PanelLeft;
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::ui::accordion::{AccordionContent, AccordionHeader, AccordionItem, AccordionTitle, AccordionTrigger};
use crate::ui::button::{ButtonSize, ButtonVariant};
use crate::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::ui::sidenav::{
    Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavGroupLabel, SidenavHeader,
    SidenavLink, SidenavMenu, SidenavMenuItem, SidenavWrapper,
};

/*
 * title: Simple Sidenav with Collapsible menus

*/

#[component(transparent)]
pub fn Sidenav02Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment(SidenavRoutes::view_segment()) view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav02.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav02 /> }
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
                    <Sidenav02Content current_section sidenav_route />
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
fn Sidenav02Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                                                <AccordionTitle>
                                                    {move || current_section.get().to_title()}
                                                </AccordionTitle>
                                            </AccordionHeader>
                                        </AccordionTrigger>
                                        <AccordionContent class="pt-0">
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
                                                        view! { <SidenavLink href=*href>{*link_title}</SidenavLink> }
                                                            .into_any()
                                                    }
                                                })
                                                .collect_view()}
                                        </AccordionContent>
                                    </AccordionItem>
                                </SidenavMenuItem>
                            </SidenavMenu>
                        </SidenavGroupContent>
                    </SidenavGroup>
                }
            }}
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
/// This component renders only the Sheet content, the trigger is in the header
#[component]
pub fn Sidenav02MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                    <Sidenav02Content current_section sidenav_route />
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
    ("/view/sidenav02/docs/components/accordion", "Accordion"),
    ("/view/sidenav02/docs/components/alert", "Alert"),
    ("/view/sidenav02/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav02/docs/components/button", "Button"),
    ("/view/sidenav02/docs/components/card", "Card"),
    ("/view/sidenav02/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav02/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav02/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav02/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav02/docs/hooks/use-random", "Use Random"),
];
