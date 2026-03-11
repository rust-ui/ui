use leptos::prelude::*;

#[component]
pub fn SuggestComponents() -> impl IntoView {
    view! {
        <div class="py-16 text-center border-t">
            <h2 class="mb-6 text-2xl font-semibold text-foreground">"Didn't find what you were looking for?"</h2>
            <a
                attr:href="https://github.com/rust-ui/labs/discussions/categories/suggestions"
                attr:target="_blank"
                attr:rel="noopener noreferrer"
                class="inline-flex justify-center items-center py-2 px-6 text-sm font-medium rounded-md shadow transition-colors focus-visible:ring-1 focus-visible:outline-none bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:ring-ring"
            >
                "Suggest component"
            </a>
        </div>
    }
}
