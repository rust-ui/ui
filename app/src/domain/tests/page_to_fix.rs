use leptos::prelude::*;
use registry::demos::demo_carousel::DemoCarousel;

#[component]
pub fn PageToFix() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10 p-8">
            <h1 class="mb-4 text-2xl font-bold">"To Fix"</h1>
            <a href="/test" class="text-sm underline text-muted-foreground underline-offset-4 hover:text-foreground">
                "→ Test"
            </a>

            <div>
                <h2 class="mb-4 text-xl font-semibold">"Carousel"</h2>
                <DemoCarousel />
            </div>
        </div>
    }
}
