use leptos::prelude::*;

/// Simple wrapper for md_docs components - just renders the preview without tabs or resizable features
#[component]
pub fn StaticMdDocsWrapper(children: Children) -> impl IntoView {
    view! { <div class="p-2 w-full rounded-md border">{children()}</div> }
}
