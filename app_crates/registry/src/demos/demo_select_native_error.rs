use leptos::prelude::*;

use crate::ui::select_native::{LabelNative, SelectNative};

#[component]
pub fn DemoSelectNativeError() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_ERROR";

    view! {
        <div class="space-y-2 [&>_svg]:text-destructive/80 min-w-[300px]">
            <LabelNative r#for=TARGET_ID>"Select with error (native)"</LabelNative>

            <div class="border-destructive/80 text-destructive [&_select]:border-destructive/80 [&_select]:text-destructive [&_select:focus-visible]:border-destructive/80 [&_select:focus-visible]:ring-destructive/20">
                <SelectNative id=TARGET_ID>
                    <option value="1">React</option>
                    <option value="2">Next.js</option>
                    <option value="3">Astro</option>
                    <option value="4">Gatsby</option>
                </SelectNative>
            </div>

            <p class="mt-2 text-xs text-destructive" role="alert" aria-live="polite">
                Selected option is invalid
            </p>
        </div>
    }
}
