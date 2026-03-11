use leptos::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeleton() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-3">
            <Skeleton class="rounded-xl h-[125px] w-[250px]" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px]" />
                <Skeleton class="h-4 w-[200px]" />
            </div>
        </div>
    }
}
