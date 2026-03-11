use leptos::prelude::*;

use crate::ui::select_native::{OverlappingLabel, SelectNative};

#[component]
pub fn DemoSelectNativeOverlappingLabel() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_OVERLAPPING_LABEL";

    view! {
        <div class="relative group min-w-[300px]">
            <OverlappingLabel r#for=TARGET_ID label="Select with overlapping label (native)" />

            <SelectNative id=TARGET_ID>
                <option value="" disabled selected>
                    Select framework
                </option>
                <option value="1">React</option>
                <option value="2">Next.js</option>
                <option value="3">Astro</option>
                <option value="4">Gatsby</option>
            </SelectNative>
        </div>
    }
}
