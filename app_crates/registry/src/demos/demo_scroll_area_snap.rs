use leptos::prelude::*;

use crate::ui::scroll_area::{SnapItem, SnapScrollArea};

#[component]
pub fn DemoScrollAreaSnap() -> impl IntoView {
    let images = (1..=6).map(|i| format!("Image {}", i)).collect::<Vec<_>>();

    view! {
        <div class="overflow-hidden relative">
            <div class="flex justify-start items-end pt-10 mb-6 ml-[50%]">
                <div class="px-1.5 ml-2 font-mono leading-6 text-indigo-600 bg-indigo-50 rounded ring-1 ring-inset ring-indigo-600 dark:text-white dark:bg-indigo-500 dark:ring-0 text-[0.625rem] dark:highlight-white/10">
                    snap point
                </div>
                <div class="absolute top-0 bottom-0 left-1/2 border-l border-indigo-500"></div>
            </div>

            <SnapScrollArea class="flex relative gap-6 pb-14 w-full px-[calc(50%-10rem)]">
                {images
                    .into_iter()
                    .map(|label| {
                        view! {
                            <SnapItem>
                                <div class="flex justify-center items-center w-80 h-40 text-sm rounded-md bg-muted text-muted-foreground">
                                    {label}
                                </div>
                            </SnapItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </SnapScrollArea>
        </div>
    }
}
