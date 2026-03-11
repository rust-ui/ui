use leptos::prelude::*;

use crate::hooks::use_random::use_random_id;

#[component]
pub fn DemoUseRandom() -> impl IntoView {
    let checkbox_id = use_random_id();

    view! {
        <div class="p-4 mx-auto space-y-4 max-w-md">
            <h3 class="font-semibold">"Random ID Hook"</h3>

            <p class="text-sm">
                "Generated ID: " <code class="py-1 px-2 text-xs rounded bg-muted">{checkbox_id.clone()}</code>
            </p>

            <div class="flex items-center space-x-2">
                <input type="checkbox" id=checkbox_id />
                <label class="text-sm">"Checkbox with unique ID"</label>
            </div>
        </div>
    }
}
