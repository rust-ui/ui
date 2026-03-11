use app_routes::{ComponentsRoutes, HooksRoutes};
use icons::{Anchor, Component, House};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::components::navigation::header_docs::HeaderDocs;

// TODO: Find a better solution for full-page 404.
// Currently using `fixed inset-0` to overlay the entire viewport because when a route like
// `/docs/components/invalid` is accessed, it matches the `:name` param segment and renders
// inside `DocsLayout`, showing the sidebar. This overlay approach hides the layout beneath.
#[component]
pub fn PageNotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let url = use_context::<http::request::Parts>().map(|req| req.uri.to_string());
        crate::domain::bug_report::bug_reports::report_not_found(url);
    }

    view! {
        <div class="fixed inset-0 z-50 bg-background">
            <Title text="Rust/UI · 404 Not Found" />

            <HeaderDocs />

            <div class="flex flex-col justify-center items-center px-4 mt-10">
                <div class="flex flex-col items-center space-y-8 max-w-3xl text-center">
                    <h1 class="text-9xl font-bold tracking-tight">"404"</h1>

                    <h2 class="text-3xl font-bold">"Oops! Page Not Found"</h2>

                    <p class="max-w-2xl text-lg text-muted-foreground">
                        "Looks like this page took a creative detour! Don't worry, even the best designs sometimes need a redirect. Let's get you back on track to discover amazing web experiences."
                    </p>

                    <div class="flex gap-4">
                        <A href="/">
                            <Button size=ButtonSize::Lg>
                                <House class="mr-2" />
                                "Return Home"
                            </Button>
                        </A>
                        <A href=ComponentsRoutes::Button.to_route()>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                                "Browse Components"
                            </Button>
                        </A>
                    </div>

                    // Explore Section
                    <div class="pt-8 w-full">
                        <h3 class="mb-6 text-xl font-semibold">"Explore Our Registry"</h3>
                        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
                            <A href=ComponentsRoutes::base_url()>
                                <Card class="transition-all hover:shadow-lg hover:scale-[1.02]">
                                    <CardHeader>
                                        <div class="flex gap-3 items-center">
                                            <div class="flex justify-center items-center w-10 h-10 rounded-lg bg-primary/10">
                                                <Component class="w-5 h-5 text-primary" />
                                            </div>
                                            <CardTitle>"Components"</CardTitle>
                                        </div>
                                    </CardHeader>
                                    <CardContent>
                                        <CardDescription>
                                            "Discover our collection of reusable UI components."
                                        </CardDescription>
                                    </CardContent>
                                </Card>
                            </A>

                            <A href=HooksRoutes::base_url()>
                                <Card class="transition-all hover:shadow-lg hover:scale-[1.02]">
                                    <CardHeader>
                                        <div class="flex gap-3 items-center">
                                            <div class="flex justify-center items-center w-10 h-10 rounded-lg bg-primary/10">
                                                <Anchor class="w-5 h-5 text-primary" />
                                            </div>
                                            <CardTitle>"Hooks"</CardTitle>
                                        </div>
                                    </CardHeader>
                                    <CardContent>
                                        <CardDescription>
                                            "Learn about powerful hooks for enhanced functionality."
                                        </CardDescription>
                                    </CardContent>
                                </Card>
                            </A>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
