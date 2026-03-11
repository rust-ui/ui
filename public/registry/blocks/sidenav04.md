


```rust
use icons::PanelLeft;
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::components::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::components::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::components::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::components::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::components::ui::sidenav::{
    Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavGroupLabel, SidenavHeader,
    SidenavLink, SidenavMenu, SidenavVariant, SidenavWrapper,
};

#[component(transparent)]
pub fn Sidenav04Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment(SidenavRoutes::view_segment()) view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav04.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav04 /> }
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
                // * Desktop Sidenav (hidden on mobile) - Floating variant
                <Sidenav variant=SidenavVariant::Floating>
                    <Sidenav04Content current_section sidenav_route />
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
fn Sidenav04Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                        <SidenavGroupLabel>{move || current_section.get().to_title()}</SidenavGroupLabel>
                        <SidenavGroupContent>
                            <SidenavMenu>
                                {links
                                    .iter()
                                    .map(|(href, title)| {
                                        if let Some(ref target_id) = sheet_target_id {
                                            // Wrap in div with data-sheet-close when inside a Sheet (mobile)
                                            view! {
                                                <div data-sheet-close=target_id.clone()>
                                                    <SidenavLink href=*href>{*title}</SidenavLink>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <SidenavLink href=*href>{*title}</SidenavLink> }.into_any()
                                        }
                                    })
                                    .collect_view()}
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
#[component]
pub fn Sidenav04MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                    <Sidenav04Content current_section sidenav_route />
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
    ("/view/sidenav04/docs/components/accordion", "Accordion"),
    ("/view/sidenav04/docs/components/alert", "Alert"),
    ("/view/sidenav04/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav04/docs/components/button", "Button"),
    ("/view/sidenav04/docs/components/card", "Card"),
    ("/view/sidenav04/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav04/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav04/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav04/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav04/docs/hooks/use-random", "Use Random"),
];
```