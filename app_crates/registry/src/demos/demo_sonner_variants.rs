use leptos::prelude::*;

use crate::ui::sonner::SonnerTrigger;
use crate::ui::sonner::SonnerToaster;
use crate::ui::toast_custom::_data::ToastLevel;

#[component]
pub fn DemoSonnerVariants() -> impl IntoView {
    view! {
        <>
            <div class="flex flex-wrap gap-2 justify-center">
                <SonnerTrigger title="You got a message" description="You toasted me!">
                    "Default"
                </SonnerTrigger>

                <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastLevel::Success>
                    "Success"
                </SonnerTrigger>
                <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastLevel::Error>
                    "Error"
                </SonnerTrigger>
                <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastLevel::Warn>
                    "Warning"
                </SonnerTrigger>
                <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastLevel::Info>
                    "Info"
                </SonnerTrigger>
                <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastLevel::Loading>
                    "Loading"
                </SonnerTrigger>
            </div>
            <SonnerToaster />
        </>
    }
}
