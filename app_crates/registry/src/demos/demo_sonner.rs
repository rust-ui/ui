use leptos::prelude::*;

use crate::ui::sonner::SonnerTrigger;

#[component]
pub fn DemoSonner() -> impl IntoView {
    view! {
        <SonnerTrigger title="You got a message" description="You toasted me!">
            "Toast Me!"
        </SonnerTrigger>
    }
}
