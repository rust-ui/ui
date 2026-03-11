use leptos::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonTable() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 w-full max-w-lg">
            // Header row
            <div class="flex gap-4 pb-2 border-b">
                <Skeleton class="w-24 h-4" />
                <Skeleton class="w-32 h-4" />
                <Skeleton class="w-20 h-4" />
                <Skeleton class="ml-auto w-28 h-4" />
            </div>
            // Data rows
            {(0..5)
                .map(|_| {
                    view! {
                        <div class="flex gap-4 py-1">
                            <Skeleton class="w-24 h-4" />
                            <Skeleton class="w-32 h-4" />
                            <Skeleton class="w-20 h-4" />
                            <Skeleton class="ml-auto w-16 h-4" />
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
