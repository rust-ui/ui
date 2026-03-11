use icons::{Monitor, Smartphone};
use leptos::prelude::*;

use crate::hooks::use_is_mobile::use_is_mobile;

#[component]
pub fn DemoUseIsMobile() -> impl IntoView {
    let is_mobile = use_is_mobile();

    view! {
        <div class="flex flex-col gap-3 items-center">
            <div class="flex gap-2 items-center text-sm font-medium">
                {move || {
                    if is_mobile.get() {
                        view! {
                            <Smartphone class="size-4" />
                            <span>"Mobile"</span>
                        }
                            .into_any()
                    } else {
                        view! {
                            <Monitor class="size-4" />
                            <span>"Desktop"</span>
                        }
                            .into_any()
                    }
                }}
            </div>
            <p class="text-xs text-muted-foreground">"Resize the window below 768px to toggle."</p>
        </div>
    }
}
