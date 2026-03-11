use app_config::{SeoMeta, SiteConfig};
use leptos::prelude::*;

use crate::components::navigation::header_docs::HeaderDocs;
use crate::components::newsletter_signup::NewsletterSignup;

#[component]
pub fn PageCharts() -> impl IntoView {
    let title = format!("Leptos Charts & Graphs · Rust UI Components | {}", SiteConfig::TITLE);
    let description = "Beautiful Rust UI components for Leptos applications. Professional chart components and graphs for modern web apps - coming soon. Sign up to get notified!".to_string();
    let canonical_url = format!("{}/charts", SiteConfig::BASE_URL);
    let image_url = format!("{}/og-image-charts.png", SiteConfig::BASE_URL);

    view! {
        <SeoMeta title=title description=description canonical_url=canonical_url image_url=image_url />

        <HeaderDocs />

        <div class="flex flex-col gap-8 justify-center items-center px-4 my-20 mx-auto max-w-4xl">
            <div class="flex flex-col gap-4 items-center text-center">
                <h1 class="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl">"Coming Soon"</h1>
                <p class="max-w-2xl text-lg text-muted-foreground">
                    "Chart components are currently under development. Sign up to get notified when they're ready!"
                </p>
            </div>

            <div class="w-full max-w-2xl">
                <NewsletterSignup />
            </div>
        </div>
    }
}
