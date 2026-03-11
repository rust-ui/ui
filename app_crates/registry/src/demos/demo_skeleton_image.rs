use icons::Image;
use leptos::prelude::*;

use crate::ui::skeleton::Skeleton;

// TODO UI.

#[component]
pub fn DemoSkeletonImage() -> impl IntoView {
    view! {
        <div class="m-4 space-y-8 w-full @md:flex @md:items-center @md:space-y-0 @md:space-x-8 rtl:space-x-reverse">

            // TODO UI. Skeleton should be able to receive children (or not).
            <div class="flex justify-center items-center w-full h-48 rounded-lg animate-pulse @sm:w-96 bg-muted">
                <Image class="text-muted-foreground size-10" />
            </div>

            <div class="space-y-2 w-full">
                <Skeleton class="h-4" />
                <Skeleton class="h-4 w-[80%]" />
                <Skeleton class="h-4 w-[80%]" />
                <Skeleton class="h-4 w-[80%]" />
            </div>
            <span class="hidden">Loading...</span>
        </div>
    }
}
