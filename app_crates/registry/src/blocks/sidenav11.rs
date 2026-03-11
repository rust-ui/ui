use icons::PanelLeft;
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
#[allow(unused_imports)]
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::ui::button::{ButtonSize, ButtonVariant};
use crate::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::ui::sidenav::{
    Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavGroupLabel, SidenavHeader,
    SidenavLink, SidenavMenu, SidenavSide, SidenavWrapper,
};

/*
 * title: Right-Side Sidenav

*/

#[component(transparent)]
pub fn Sidenav11Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment(SidenavRoutes::view_segment()) view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav11.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav11 /> }
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

// * Content (SidenavInset) renders first → appears on left.
// * Sidenav renders second with data_side=Right → fixed to the right edge.
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
                // * ---> [📍 OUTLET] for nested routing (SidenavInset) — comes first = renders on left.
                <Outlet />
                // * Desktop Sidenav fixed to right edge
                <Sidenav data_side=SidenavSide::Right>
                    <Sidenav11Content current_section sidenav_route />
                </Sidenav>
            </SidenavWrapper>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ SHARED CONTENT ✨                   */
/* ========================================================== */

/// Shared sidenav content used by both desktop Sidenav and mobile Sheet
#[component]
fn Sidenav11Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    let sheet_ctx = use_context::<SheetContext>();

    view! {
        <SidenavHeader>
            <SidenavRoutesSelector current_section sidenav_route />
        </SidenavHeader>

        <SidenavContent>
            <SidenavGroup>
                <SidenavGroupLabel>{move || current_section.get().to_title()}</SidenavGroupLabel>
                <SidenavGroupContent>
                    <SidenavMenu>
                        {move || {
                            let sheet_target_id = sheet_ctx.as_ref().map(|ctx| ctx.target_id.clone());
                            match current_section.get() {
                                DocsRoutes::Components => COMPONENT_LINKS,
                                DocsRoutes::Hooks => HOOKS_LINKS,
                            }
                                .iter()
                                .map(|(href, title)| {
                                    if let Some(ref target_id) = sheet_target_id {
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
                                .collect_view()
                        }}
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

/// Mobile sheet slides from the right to match the right-side sidenav
#[component]
pub fn Sidenav11MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    view! {
        <Sheet>
            <div class="md:hidden">
                <SheetTrigger class="size-7" variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                    <PanelLeft class="rotate-180 size-4" />
                    <span class="hidden">"Toggle Sidenav"</span>
                </SheetTrigger>
            </div>
            <SheetContent
                direction=SheetDirection::Right
                class="p-0 w-[18rem] bg-sidenav text-sidenav-foreground"
                show_close_button=false
            >
                <div class="flex flex-col h-full">
                    <Sidenav11Content current_section sidenav_route />
                </div>
            </SheetContent>
        </Sheet>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const COMPONENT_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav11/docs/components/accordion", "Accordion"),
    ("/view/sidenav11/docs/components/alert", "Alert"),
    ("/view/sidenav11/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav11/docs/components/button", "Button"),
    ("/view/sidenav11/docs/components/card", "Card"),
    ("/view/sidenav11/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav11/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav11/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav11/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav11/docs/hooks/use-random", "Use Random"),
];
