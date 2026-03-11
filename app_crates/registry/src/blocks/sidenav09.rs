use icons::{Component, Layers, PanelLeft};
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::hooks::use_location;
#[allow(unused_imports)]
use leptos_router::{MatchNestedRoutes, StaticSegment};

use crate::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::blocks::sidenav_routes_selector::SidenavRoutesSelector;
use crate::blocks::sidenav_routes_simplified::SidenavRoutesSimplified;
use crate::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::demos::demo_dropdown_menu_user_icon::DemoDropdownMenuUserIcon;
use crate::ui::accordion::{AccordionContent, AccordionHeader, AccordionItem, AccordionTitle, AccordionTrigger};
use crate::ui::button::{ButtonSize, ButtonVariant};
use crate::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::ui::sidenav::{
    Sidenav, SidenavCollapsible, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavGroupLabel,
    SidenavHeader, SidenavLink, SidenavMenu, SidenavMenuButton, SidenavMenuItem, SidenavWrapper,
};

/*
 * title: Nested Sidenavs with Route-based Navigation

*/

#[component(transparent)]
pub fn Sidenav09Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=StaticSegment(SidenavRoutes::view_segment()) view=|| view! { <Outlet /> }>
            <ParentRoute
                path=StaticSegment(SidenavRoutes::Sidenav09.as_ref())
                view=move || view! { <SidenavLayout sidenav_route=SidenavRoutes::Sidenav09 /> }
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
            <SidenavWrapper attr:style="--sidenav-width:18rem;--sidenav-width-icon:3rem">
                <Sidenav
                    data_collapsible=SidenavCollapsible::Icon
                    class="overflow-hidden *:data-[sidenav=Sidenav]:flex-row"
                >
                    // * 1. Icon-based navigation for DocsRoutes
                    <FirstSidenav data_collapsible=SidenavCollapsible::None sidenav_route=sidenav_route />
                    // * 2. Detailed navigation based on selected route
                    <SecondSidenav data_collapsible=SidenavCollapsible::None current_section=current_section />
                </Sidenav>

                // * ---> [📍 OUTLET] renders SidenavInsetRight via SharedDocsRoutes
                <Outlet />
            </SidenavWrapper>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FirstSidenav(data_collapsible: SidenavCollapsible, sidenav_route: SidenavRoutes) -> impl IntoView {
    // * 🚑 Shortfix to display the data-tooltip. The `Tooltip` component uses `absolute`, which encapsulates
    // * the TooltipContent into the parent div. Find a better solution after.
    const SHARED_TOOLTIP_CLASS: &str = "relative tooltip__shortfix__display group after:content-[attr(data-tooltip)] after:fixed after:left-12 after:py-1 after:px-2 after:text-xs after:whitespace-nowrap after:rounded after:border after:shadow-md after:opacity-0 after:transition-opacity after:duration-200 after:pointer-events-none after:z-[1000000] after:bg-popover after:text-popover-foreground hover:after:opacity-100 focus-visible:after:opacity-100";

    view! {
        <style>
            {r#"
            /* CSS Counter-based Tooltip Positioning System */
            :root {
            --tooltip-header-offset: 32px;
            --tooltip-gap-after-header: 52px;
            --tooltip-item-height: 36px;
            }
            .tooltip___base {
            counter-reset: counter-base;
            }
            .tooltip__shortfix__display {
            counter-increment: counter-base;
            }
            .tooltip__shortfix__display::after {
            top: calc(var(--tooltip-header-offset) + var(--tooltip-gap-after-header) + (counter(counter-base) - 1) * var(--tooltip-item-height)) !important;
            }
            "#}
        </style>

        <Sidenav data_collapsible=data_collapsible class="border-r w-[calc(var(--sidenav-width-icon)+1px)]!">
            <SidenavHeader>
                <SidenavMenu attr:data-sidenav="menu">
                    <SidenavMenuItem attr:data-sidenav="menu-item">
                        <SidenavMenuButton class="md:p-0" attr:href="#" attr:data-sidenav="menu-button">
                            <div class="flex justify-center items-center rounded-lg bg-sidenav-primary text-sidenav-primary-foreground aspect-square size-8 [&_svg:not([class*='size-'])]:size-4">
                                <Component />
                            </div>
                        </SidenavMenuButton>
                    </SidenavMenuItem>
                </SidenavMenu>
            </SidenavHeader>
            <SidenavContent attr:data-sidenav="content">
                <SidenavGroup attr:data-sidenav="group">
                    <SidenavGroupContent attr:data-sidenav="group-content" class="px-1.5 md:px-0">
                        <SidenavMenu attr:data-sidenav="menu" class="tooltip___base">
                            <SidenavMenuItem attr:data-sidenav="menu-item">
                                <SidenavMenuButton
                                    attr:data-sidenav="menu-button"
                                    href=format!(
                                        "/{}/docs/{}",
                                        sidenav_route.to_route(),
                                        DocsRoutes::Components.as_ref(),
                                    )
                                    class=SHARED_TOOLTIP_CLASS
                                    attr:data-tooltip="Components"
                                >
                                    <Component />
                                </SidenavMenuButton>
                            </SidenavMenuItem>
                            <SidenavMenuItem attr:data-sidenav="menu-item">
                                <SidenavMenuButton
                                    attr:data-sidenav="menu-button"
                                    href=format!("/{}/docs/{}", sidenav_route.to_route(), DocsRoutes::Hooks.as_ref())
                                    class=SHARED_TOOLTIP_CLASS
                                    attr:data-tooltip="Hooks"
                                >
                                    <Layers />
                                </SidenavMenuButton>
                            </SidenavMenuItem>
                        </SidenavMenu>
                    </SidenavGroupContent>
                </SidenavGroup>
            </SidenavContent>
            <SidenavFooter attr:data-sidenav="footer">
                <SidenavMenu attr:data-sidenav="menu">
                    <SidenavMenuItem attr:data-sidenav="menu-item">
                        // * @demo
                        <DemoDropdownMenuUserIcon />
                    </SidenavMenuItem>
                </SidenavMenu>
            </SidenavFooter>
        </Sidenav>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Shared sidenav content for mobile Sheet - shows the SecondSidenav navigation
#[component]
fn Sidenav09Content(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                                        <AccordionTrigger open=true class="peer-checked:bg-accent hover:bg-accent">
                                            <AccordionHeader>
                                                <AccordionTitle>
                                                    {move || current_section.get().to_title()}
                                                </AccordionTitle>
                                            </AccordionHeader>
                                        </AccordionTrigger>
                                        <AccordionContent>
                                            <SidenavMenu>
                                                {links
                                                    .iter()
                                                    .map(|(href, link_title)| {
                                                        if let Some(ref target_id) = sheet_target_id {
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
                                            </SidenavMenu>
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

#[component]
pub fn SecondSidenav(data_collapsible: SidenavCollapsible, current_section: Memo<DocsRoutes>) -> impl IntoView {
    view! {
        <Sidenav data_collapsible=data_collapsible class="hidden flex-1 md:flex">
            <SidenavHeader attr:data-sidenav="header" class="p-3 border-b">
                <span class="font-medium">"Routes"</span>
            </SidenavHeader>
            <SidenavContent attr:data-sidenav="content">
                <SidenavGroup attr:data-sidenav="group">
                    <SidenavGroupContent attr:data-sidenav="group-content">
                        <SidenavGroupLabel>{move || current_section.get().to_title()}</SidenavGroupLabel>
                        <SidenavMenu>
                            <SidenavMenuItem>
                                <AccordionItem>
                                    <AccordionTrigger open=true class="peer-checked:bg-accent hover:bg-accent">
                                        <AccordionHeader>
                                            <AccordionTitle>{move || current_section.get().to_title()}</AccordionTitle>
                                        </AccordionHeader>
                                    </AccordionTrigger>
                                    <AccordionContent>
                                        <SidenavMenu>
                                            {move || {
                                                let links = match current_section.get() {
                                                    DocsRoutes::Components => COMPONENT_LINKS,
                                                    DocsRoutes::Hooks => HOOKS_LINKS,
                                                };
                                                links
                                                    .iter()
                                                    .map(|(href, title)| {
                                                        view! { <SidenavLink href=*href>{*title}</SidenavLink> }
                                                    })
                                                    .collect_view()
                                            }}
                                        </SidenavMenu>
                                    </AccordionContent>
                                </AccordionItem>
                            </SidenavMenuItem>
                        </SidenavMenu>
                    </SidenavGroupContent>
                </SidenavGroup>
            </SidenavContent>
        </Sidenav>
    }
}

/* ========================================================== */
/*                     ✨ MOBILE SHEET ✨                     */
/* ========================================================== */

/// Mobile sheet containing sidenav content - uses the proper Sheet component
#[component]
pub fn Sidenav09MobileSheet(current_section: Memo<DocsRoutes>, sidenav_route: SidenavRoutes) -> impl IntoView {
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
                    <Sidenav09Content current_section sidenav_route />
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
    ("/view/sidenav09/docs/components/accordion", "Accordion"),
    ("/view/sidenav09/docs/components/alert", "Alert"),
    ("/view/sidenav09/docs/components/alert-dialog", "Alert Dialog"),
    ("/view/sidenav09/docs/components/button", "Button"),
    ("/view/sidenav09/docs/components/card", "Card"),
    ("/view/sidenav09/docs/components/checkbox", "Checkbox"),
    ("/view/sidenav09/docs/components/dialog", "Dialog"),
];

const HOOKS_LINKS: &[(&str, &str)] = &[
    ("/view/sidenav09/docs/hooks/use-copy-clipboard", "Use Copy Clipboard"),
    ("/view/sidenav09/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll"),
    ("/view/sidenav09/docs/hooks/use-random", "Use Random"),
];
