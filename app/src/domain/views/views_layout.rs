use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_router::components::Outlet;

#[component]
pub fn ViewsLayout() -> impl IntoView {
    view! {
        <Meta name="robots" content="noindex, nofollow" />
        <Outlet />
    }
}
