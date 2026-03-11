use leptos::prelude::*;

use crate::ui::select_native::{LabelNative, SelectNative};

#[component]
pub fn DemoSelectNativeAutoWidth() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_AUTO_WIDTH";

    view! {
        <div class="space-y-2 min-w-[300px]">
            <LabelNative r#for=TARGET_ID>"Select with auto-width (native)"</LabelNative>

            <div class="w-fit">
                <SelectNative id=TARGET_ID>
                    <option value="1">React</option>
                    <option value="2">Next.js</option>
                    <option value="3">Astro</option>
                    <option value="4">Gatsby</option>
                </SelectNative>
            </div>
        </div>
    }
}
