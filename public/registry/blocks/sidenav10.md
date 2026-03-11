


```rust
use icons::{PanelLeft, Search};
use leptos::ev::Event;
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
#[allow(unused_imports)]
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::components::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::components::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::components::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::components::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::label::Label;
use crate::components::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::components::ui::sidenav::{
    Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavGroupLabel, SidenavHeader,
    SidenavInput, SidenavLink, SidenavMenu, SidenavWrapper,
};

#[component(transparent)]
pub fn Sidenav10Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment("view") view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav10.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav10 /> }
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
                    <Sidenav10Content current_section sidenav_route />
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
fn Sidenav10Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
    // Try to get SheetContext - will be Some when inside a Sheet (mobile), None otherwise (desktop)
    let sheet_ctx = use_context::<SheetContext>();

    // Search state
    let search_query_signal = RwSignal::new(String::new());

    let handle_search = move |ev: Event| {
        let value = event_target_value(&ev);
        search_query_signal.set(value);
    };

    view! {
        <SidenavHeader>
            <SidenavRoutesSelector current_section sidenav_route />
            <form>
                <SidenavGroup attr:data-sidenav="group">
                    <SidenavGroupContent attr:data-sidenav="group-content" class="relative">
                        <Label html_for="search" class="hidden">
                            Search
                        </Label>
                        <SidenavInput
                            class="pl-8"
                            attr:id="search"
                            attr:placeholder="Search the docs..."
                            attr:value=move || search_query_signal.get()
                            on:input=handle_search
                        />
                        <Search class="absolute left-2 top-1/2 opacity-50 -translate-y-1/2 pointer-events-none select-none size-4" />
                    </SidenavGroupContent>
                </SidenavGroup>
            </form>
        </SidenavHeader>

        <SidenavContent>
            {move || {
                let sheet_target_id = sheet_ctx.as_ref().map(|ctx| ctx.target_id.clone());
                let query = search_query_signal.get().to_lowercase();
                let links = match current_section.get() {
                    DocsRoutes::Components => COMPONENT_LINKS,
                    DocsRoutes::Hooks => HOOKS_LINKS,
                };
                let filtered: Vec<_> = if query.is_empty() {
                    links.to_vec()
                } else {
                    links.iter().filter(|(_, title)| title.to_lowercase().contains(&query)).copied().collect()
                };

                view! {
                    <SidenavGroup>
                        <SidenavGroupLabel>{move || current_section.get().to_title()}</SidenavGroupLabel>
                        <SidenavGroupContent>
                            <SidenavMenu>
                                {if filtered.is_empty() && !query.is_empty() {
                                    view! {
                                        <div class="py-4 px-2 text-sm text-muted-foreground">No results found</div>
                                    }
                                        .into_any()
                                } else {
                                    filtered
                                        .into_iter()
                                        .map(|(href, link_title)| {
                                            if let Some(ref target_id) = sheet_target_id {
                                                view! {
                                                    <div data-sheet-close=target_id.clone()>
                                                        <SidenavLink href=href>{link_title}</SidenavLink>
                                                    </div>
                                                }
                                                    .into_any()
                                            } else {
                                                view! { <SidenavLink href=href>{link_title}</SidenavLink> }.into_any()
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }}
                            </SidenavMenu>
                        </SidenavGroupContent>
                    </SidenavGroup>
                }
            }}
        </SidenavContent>

        <SidenavFooter>
            // * @demo
            <DemoDropdownMenuUser />
        </SidenavFooter>
    }
}

/* ========================================================== */
/*                     ✨ MOBILE SHEET ✨                     */
/* ========================================================== */

/// Mobile sheet containing sidenav content - uses the proper Sheet component
#[component]
pub fn Sidenav10MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                    <Sidenav10Content current_section sidenav_route />
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
    ("/view/sidenav10/docs/components/accordion", "Accordion"),
    ("/view/sidenav10/docs/components/alert", "Alert"),
    ("/view/sidenav10/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav10/docs/components/button", "Button"),
    ("/view/sidenav10/docs/components/card", "Card"),
    ("/view/sidenav10/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav10/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav10/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav10/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav10/docs/hooks/use-random", "Use Random"),
];
```