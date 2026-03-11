use leptos::prelude::*;

use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressControlled() -> impl IntoView {
    let value = RwSignal::new(50.0_f64);

    view! {
        <div class="flex overflow-hidden flex-col gap-4 w-60">
            <Progress value=value />
            <input
                type="range"
                min="0"
                max="100"
                step="1"
                prop:value=move || value.get()
                class="w-full accent-primary"
                on:input=move |e| {
                    let val = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                    value.set(val);
                }
            />
        </div>
    }
}
