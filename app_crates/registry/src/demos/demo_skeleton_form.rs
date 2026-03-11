use leptos::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonForm() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 max-w-sm">
            <div class="flex flex-col gap-2">
                <Skeleton class="w-20 h-4" />
                <Skeleton class="w-full h-9 rounded-md" />
            </div>
            <div class="flex flex-col gap-2">
                <Skeleton class="w-16 h-4" />
                <Skeleton class="w-full h-9 rounded-md" />
            </div>
            <div class="flex flex-col gap-2">
                <Skeleton class="w-24 h-4" />
                <Skeleton class="w-full h-9 rounded-md" />
            </div>
            <Skeleton class="w-full h-9 rounded-md" />
        </div>
    }
}
