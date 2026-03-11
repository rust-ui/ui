use leptos::prelude::*;
use registry::demos::demo_data_table_filters::DemoDataTableFilters;

#[component]
pub fn PageTest() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10 p-8">
            <h1 class="mb-4 text-2xl font-bold">"Test Page"</h1>
            <a href="/to-fix" class="text-sm underline text-muted-foreground underline-offset-4 hover:text-foreground">
                "→ To Fix"
            </a>

            <DemoDataTableFilters />

        // Keep: intentional hydration bug for testing
        // <HydrationBugTest />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Intentional Leptos warning: calling .get() outside reactive context.
#[component]
fn HydrationBugTest() -> impl IntoView {
    let test_signal = RwSignal::new(42);

    // This triggers "reading a signal outside reactive context" warning
    let value = test_signal.get();

    view! {
        <div class="p-4 bg-yellow-100 rounded dark:bg-yellow-900">
            <p class="text-sm">"[TEST] Value: " {value}</p>
        </div>
    }
}
