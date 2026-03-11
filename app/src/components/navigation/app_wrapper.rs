use leptos::prelude::*;

#[component]
pub fn AppWrapper(children: Children) -> impl IntoView {
    view! {
        // Empty touchstart enables CSS :active on iOS
        <div class="flex flex-col h-full bottom__safe" on:touchstart=|_| {}>
            {children()}
        </div>
    }
}
