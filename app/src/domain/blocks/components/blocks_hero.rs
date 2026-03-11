use app_components::AnnouncementBadge;
use app_routes::BlockRoutes;
use leptos::prelude::*;
use registry::hooks::use_can_scroll::use_can_scroll;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::link::Link;
use strum::IntoEnumIterator;

#[component]
pub fn BlocksHero() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10">
            <BlocksHeader />
            <BlocksNavigation />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn BlocksNavigation() -> impl IntoView {
    let nav_ref = NodeRef::<leptos::html::Nav>::new();
    let (update_fades, show_left_fade, show_right_fade) = use_can_scroll(nav_ref);

    view! {
        <div class="relative lg:max-w-none max-w-[600px]">
            <nav
                node_ref=nav_ref
                attr:data-name="__BlocksNav"
                class="overflow-x-auto relative [-webkit-overflow-scrolling:touch] [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden [touch-action:pan-x]"
                on:scroll=move |_| update_fades()
            >
                <div class="flex gap-4 items-center [&_span]:capitalize">
                    {BlockRoutes::iter()
                        .map(|route| {
                            view! {
                                <Link href=route.to_route() scroll=false class="whitespace-nowrap shrink-0">
                                    {route.to_string()}
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
fn BlocksHeader() -> impl IntoView {
    const HASHTAG_HREF: &str = "#";

    view! {
        <div class="flex flex-col gap-2 items-center py-8 text-center md:py-16 lg:py-20 xl:gap-4">
            <AnnouncementBadge />
            <h1 class="max-w-2xl text-4xl font-semibold tracking-tight lg:font-semibold xl:text-5xl xl:tracking-tighter text-primary leading-tighter text-balance lg:leading-[1.1]">
                "Leptos UI Blocks & Components"
            </h1>
            <p class="max-w-3xl text-base sm:text-lg text-foreground text-balance">
                "Beautiful Rust UI components for Leptos applications. Ready-to-use blocks and components for modern web apps - copy and paste into your project."
            </p>
            <div class="flex gap-2 justify-center items-center pt-2 w-full">
                <Button attr:href=HASHTAG_HREF>Browse Blocks</Button>
                <Button variant=ButtonVariant::Outline attr:href=HASHTAG_HREF>
                    Add a block
                </Button>
            </div>
        </div>
    }
}
