use icons::PanelLeft;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use super::sidenav01::Sidenav01MobileSheet;
use super::sidenav02::Sidenav02MobileSheet;
use super::sidenav03::Sidenav03MobileSheet;
use super::sidenav04::Sidenav04MobileSheet;
use super::sidenav05::Sidenav05MobileSheet;
use super::sidenav06::Sidenav06MobileSheet;
use super::sidenav07::Sidenav07MobileSheet;
use super::sidenav08::Sidenav08MobileSheet;
use super::sidenav09::Sidenav09MobileSheet;
use super::sidenav10::Sidenav10MobileSheet;
use super::sidenav11::Sidenav11MobileSheet;
use crate::hooks::use_breadcrumb::use_breadcrumb_from_segment;
use crate::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::ui::separator::{Separator, SeparatorOrientation};
use crate::ui::sidenav::{SidenavInset, SidenavTrigger, SidenavVariant};

// * Sidenav Demo Layout with dynamic breadcrumb. Always render the SidenavInset with dynamice Breadcrumb.
#[component]
pub fn SidenavInsetRight(#[prop(into, optional)] data_variant: Option<SidenavVariant>) -> impl IntoView {
    let breadcrumb_items = use_breadcrumb_from_segment(DocsRoutes::base_segment());

    // For mobile sheet - determine current section and sidenav route from URL
    let location = use_location();
    let current_section = Memo::new(move |_| {
        let path = location.pathname.get();
        if path.contains(DocsRoutes::Components.as_ref()) { DocsRoutes::Components } else { DocsRoutes::Hooks }
    });

    // Detect which sidenav route is active based on URL
    let sidenav_route = Memo::new(move |_| SidenavRoutes::from_path(&location.pathname.get()));

    view! {
        <SidenavInset attr:data-variant=data_variant.map(|v| v.to_string())>
            <header class="flex gap-2 items-center h-16 ease-linear shrink-0 transition-[width,height] group-has-data-[collapsible=icon]/sidenav-wrapper:h-12">
                <div class="flex gap-2 items-center px-4">
                    // * Mobile Sheet trigger - visible only on mobile
                    // Render appropriate mobile sheet based on sidenav route
                    {move || match sidenav_route.get() {
                        SidenavRoutes::Sidenav02 => {
                            view! { <Sidenav02MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav02 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav03 => {
                            view! { <Sidenav03MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav03 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav04 => {
                            view! { <Sidenav04MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav04 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav05 => {
                            view! { <Sidenav05MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav05 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav06 => {
                            view! { <Sidenav06MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav06 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav07 => {
                            view! { <Sidenav07MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav07 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav08 => {
                            view! { <Sidenav08MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav08 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav09 => {
                            view! { <Sidenav09MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav09 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav10 => {
                            view! { <Sidenav10MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav10 /> }
                                .into_any()
                        }
                        SidenavRoutes::Sidenav11 => {
                            view! { <Sidenav11MobileSheet current_section sidenav_route=SidenavRoutes::Sidenav11 /> }
                                .into_any()
                        }
                        route => {
                            // Default to Sidenav01 mobile sheet for all other routes
                            view! { <Sidenav01MobileSheet current_section sidenav_route=route /> }
                                .into_any()
                        }
                    }} // * Desktop trigger (toggles sidenav collapse) - hidden on mobile
                    <div class="hidden md:block">
                        <SidenavTrigger>
                            <PanelLeft />
                            <span class="hidden">Toggle Sidenav</span>
                        </SidenavTrigger>
                    </div> <Separator orientation=SeparatorOrientation::Vertical class="-ml-1 h-4" />
                    // Dynamic Breadcrumb based on URL
                    <Breadcrumb>
                        <BreadcrumbList>
                            {move || {
                                breadcrumb_items
                                    .get()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, (name, href, is_last))| {
                                        const FIRST_ITEM_INDEX: usize = 0;
                                        let needs_separator = idx > FIRST_ITEM_INDEX;

                                        view! {
                                            <>
                                                {needs_separator.then(|| view! { <BreadcrumbSeparator /> })}
                                                <BreadcrumbItem>
                                                    {if is_last {
                                                        view! { <BreadcrumbPage>{name}</BreadcrumbPage> }.into_any()
                                                    } else {
                                                        view! { <BreadcrumbLink attr:href=href>{name}</BreadcrumbLink> }
                                                            .into_any()
                                                    }}
                                                </BreadcrumbItem>
                                            </>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </BreadcrumbList>
                    </Breadcrumb>
                </div>
            </header>

            <div class="flex flex-col flex-1 gap-4 p-4 pt-0">
                <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                    <div class="rounded-xl bg-muted/50 aspect-video"></div>
                    <div class="rounded-xl bg-muted/50 aspect-video"></div>
                    <div class="rounded-xl bg-muted/50 aspect-video"></div>
                </div>
                <div class="flex-1 rounded-xl bg-muted/50 min-h-[100vh] md:min-h-min"></div>
            </div>
        </SidenavInset>
    }
}
