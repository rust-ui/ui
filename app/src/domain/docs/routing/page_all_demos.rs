use app_components::SuggestComponents;
use app_config::{SeoMeta, SiteConfig};
use app_routes::{ComponentsRoutes, HooksRoutes};
use leptos::prelude::*;
use leptos_router::components::A;
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::__registry__::demos_sidenav::{ALL_SIDENAV_COMPONENTS, ALL_SIDENAV_HOOKS};

#[component]
pub fn PageAllDemos(segment: &'static str) -> impl IntoView {
    let (title, description, demos, base_url, page_title) = match segment {
        segment if segment == ComponentsRoutes::segment() => (
            "Leptos Components",
            "Beautiful Rust UI components for Leptos applications. Copy-and-paste components to quickly build modern fullstack web apps.",
            ALL_SIDENAV_COMPONENTS,
            ComponentsRoutes::base_url(),
            format!("Leptos Components · Rust UI Components | {}", SiteConfig::TITLE),
        ),
        segment if segment == HooksRoutes::segment() => (
            "Leptos Hooks",
            "Reusable Rust UI hooks for Leptos applications. A collection of custom hooks for building fullstack web apps.",
            ALL_SIDENAV_HOOKS,
            HooksRoutes::base_url(),
            format!("Leptos Hooks · Rust UI Components | {}", SiteConfig::TITLE),
        ),
        _ => (
            "Unknown demo type",
            "Please check the URL.",
            ALL_SIDENAV_COMPONENTS,
            "/docs",
            format!("Unknown · {}", SiteConfig::TITLE),
        ),
    };

    let all_demos: Vec<_> = demos.iter().collect();
    let canonical_url = format!("{}{}", SiteConfig::BASE_URL, base_url);

    view! {
        <SeoMeta title=page_title description=description.to_string() canonical_url=canonical_url />

        <div class="flex flex-col gap-6 py-6 w-full sm:px-4 lg:max-w-[1100px]">
            <div class="max-w-3xl max-sm:text-center text-pretty">
                <h1 class="text-3xl font-bold tracking-tight lg:text-4xl text-pretty">{title}</h1>
                <p class="mt-2 mb-8 text-lg text-muted-foreground text-balance">{description}</p>
            </div>

            <div data-name="Grid3" class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <For
                    each=move || all_demos.clone()
                    key=|demo| demo.path_url
                    children=move |demo| {
                        let href = format!("{}/{}", base_url, demo.path_url);
                        view! {
                            <A href=href>
                                <Card class="transition-all duration-200 hover:shadow-lg hover:scale-101">
                                    <CardContent>
                                        <img src=demo.image alt=demo.title class="w-full rounded-md dark:hidden" />
                                        <img
                                            src=demo.image_dark
                                            alt=demo.title
                                            class="hidden w-full rounded-md dark:block"
                                        />

                                        <CardHeader class="px-0">
                                            <CardTitle class="text-lg">{demo.title}</CardTitle>
                                        </CardHeader>

                                        <CardDescription class="line-clamp-2">{demo.description}</CardDescription>
                                    </CardContent>
                                </Card>
                            </A>
                        }
                    }
                />
            </div>

            <SuggestComponents />
        </div>
    }
}
