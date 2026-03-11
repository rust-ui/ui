use leptos::prelude::*;

use crate::ui::sonner::{SonnerTrigger, ToastType};

#[component]
pub fn DemoSonnerVariants() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2 justify-center">
            <SonnerTrigger title="You got a message" description="You toasted me!">
                "Default"
            </SonnerTrigger>

            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Success>
                "Success"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Error>
                "Error"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Warning>
                "Warning"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Info>
                "Info"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Loading>
                "Loading"
            </SonnerTrigger>
        </div>
    }
}
