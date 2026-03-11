use leptos::prelude::*;

use crate::ui::scroll_area::ScrollArea;
use crate::ui::separator::Separator;

#[component]
pub fn DemoScrollArea() -> impl IntoView {
    let tags = (0..=50).rev().map(|i| format!("v1.2.0-beta.{}", i)).collect::<Vec<_>>();

    view! {
        <ScrollArea class="w-48 h-72 rounded-md border">
            <div class="p-4">
                <h4 class="mb-4 text-sm font-medium leading-none">Tags</h4>
                {tags
                    .into_iter()
                    .map(|tag| {
                        view! {
                            <>
                                <div class="text-sm">{tag}</div>
                                <Separator class="my-2" />
                            </>
                        }
                    })
                    .collect_view()}
            </div>
        </ScrollArea>
    }
}
