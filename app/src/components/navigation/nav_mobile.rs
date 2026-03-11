use app_components::LogoHomeLink;
use app_domain::constants::RoutePaths;
use app_routes::ComponentsRoutes;
use leptos::portal::Portal;
use leptos::prelude::*;
use registry::ui::link::{Link, PathMatchType};
use registry::ui::scroll_area::ScrollArea;
use registry::ui::sheet::{
    Sheet, SheetBody, SheetContent, SheetContext, SheetDirection, SheetSize, SheetTrigger, SheetVariant,
};

use crate::__registry__::demos_sidenav::ALL_SIDENAV_COMPONENTS;

#[component]
pub fn NavMobile() -> impl IntoView {
    const HIDDEN_FOR_DESKTOP: &str = "md:hidden";

    view! {
        <Sheet class=HIDDEN_FOR_DESKTOP>
            <SheetTrigger variant=SheetVariant::Ghost size=SheetSize::Icon class=HIDDEN_FOR_DESKTOP>
                <svg
                    stroke-width="1.5"
                    viewBox="0 0 24 24"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-5"
                >
                    <path
                        d="M3 5H11"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                    <path
                        d="M3 12H16"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                    <path
                        d="M3 19H21"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </svg>
            </SheetTrigger>

            <Portal>
                <SheetContent direction=SheetDirection::Left class="pb-4 w-[60%] max-w-[350px]">
                    <SheetBody class="overflow-hidden h-full !gap-0 supports-[-webkit-touch-callout:none]:pt-8">
                        <ScrollArea class="pr-2 h-full">
                            <div class="flex flex-col gap-4">
                                <LogoHomeLink />
                                <GetStartedLinks />
                                <AllDemoComponents />
                            </div>
                        </ScrollArea>
                    </SheetBody>
                </SheetContent>
            </Portal>
        </Sheet>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn GetStartedLinks() -> impl IntoView {
    let ctx = expect_context::<SheetContext>();

    let docs_links = [
        (RoutePaths::DOCS_INTRODUCTION, "Introduction"),
        (RoutePaths::DOCS_INSTALLATION, "Installation"),
        (RoutePaths::DOCS_CLI, "CLI"),
        (RoutePaths::ICONS, "Icons"),
        (RoutePaths::BLOCKS, "Blocks"),
        (RoutePaths::CHARTS, "Charts"),
        (RoutePaths::CREATE, "Create"),
    ];

    view! {
        <div class="flex flex-col gap-1">
            <h2 class="text-xl">Get Started</h2>

            {docs_links
                .iter()
                .map(|(href, name)| {
                    let href_owned = href.to_owned();
                    let name_owned = name.to_owned();
                    let target_id = ctx.target_id.clone();
                    view! {
                        <div data-sheet-close=target_id class="cursor-pointer">
                            <Link href=href_owned match_type=PathMatchType::Exact>
                                {name_owned}
                            </Link>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn AllDemoComponents() -> impl IntoView {
    let ctx = expect_context::<SheetContext>();

    view! {
        <div class="flex flex-col gap-1">
            <h2 class="text-xl">Components</h2>

            {ALL_SIDENAV_COMPONENTS
                .iter()
                .map(|demo| {
                    let href_owned = format!("{}/{}", ComponentsRoutes::base_url(), demo.path_url);
                    let name_owned = demo.title.to_owned();
                    let target_id = ctx.target_id.clone();
                    view! {
                        <div data-sheet-close=target_id class="cursor-pointer">
                            <Link href=href_owned match_type=PathMatchType::Exact>
                                {name_owned}
                            </Link>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
