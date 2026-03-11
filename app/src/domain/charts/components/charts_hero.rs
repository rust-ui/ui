use app_components::AnnouncementBadge;
use app_routes::ChartRoutes;
use leptos::prelude::*;
use registry::hooks::use_can_scroll::use_can_scroll;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::link::Link;
use strum::IntoEnumIterator;

#[component]
pub fn ChartsHero() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10">
            <ChartsHeader />
            <ChartsNavigation />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChartsNavigation() -> impl IntoView {
    let nav_ref = NodeRef::<leptos::html::Nav>::new();
    let (update_fades, show_left_fade, show_right_fade) = use_can_scroll(nav_ref);

    view! {
        <div class="relative lg:max-w-none max-w-[600px]">
            <nav
                node_ref=nav_ref
                data-name="__ChartsNav"
                class="overflow-x-auto relative [-webkit-overflow-scrolling:touch] [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden [touch-action:pan-x]"
                on:scroll=move |_| update_fades()
            >
                <div class="flex gap-0 items-center">
                    {ChartRoutes::iter()
                        .map(|route| {
                            view! {
                                <Link
                                    href=route.to_route()
                                    scroll=false
                                    class="flex justify-center items-center px-4 h-7 text-base font-medium text-center whitespace-nowrap shrink-0"
                                >
                                    {route.to_title()}
                                </Link>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
            // Left fade indicator
            <div
                class="absolute top-0 left-0 w-8 h-full bg-gradient-to-r to-transparent transition-opacity pointer-events-none lg:hidden from-background"
                class:opacity-0=move || !show_left_fade.get()
            />
            // Right fade indicator
            <div
                class="absolute top-0 right-0 w-8 h-full bg-gradient-to-l to-transparent transition-opacity pointer-events-none lg:hidden from-background"
                class:opacity-0=move || !show_right_fade.get()
            />
        </div>
    }
}

#[component]
fn ChartsHeader() -> impl IntoView {
    const HASHTAG_HREF: &str = "#";

    view! {
        <div class="flex flex-col gap-2 items-center py-8 text-center md:py-16 lg:pt-20 xl:gap-4">
            <AnnouncementBadge />
            <h1 class="max-w-2xl text-4xl font-semibold tracking-tight lg:font-semibold xl:text-5xl xl:tracking-tighter text-primary leading-tighter text-balance lg:leading-[1.1]">
                "Leptos Rust UI Charts & Graphs"
            </h1>
            <p class="max-w-3xl text-base sm:text-lg text-foreground text-balance">
                "Beautiful Leptos Rust UI chart components. Ready-to-use charts and graphs for modern web apps - copy and paste into your Leptos project."
            </p>
            <div class="flex gap-2 justify-center items-center pt-2 w-full">
                <Button attr:href=HASHTAG_HREF>Browse Charts</Button>
                <Button variant=ButtonVariant::Outline attr:href=HASHTAG_HREF>
                    Documentation
                </Button>
            </div>
        </div>
    }
}
