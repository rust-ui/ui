use leptos::prelude::*;

const RADII: &[f32] = &[0.0, 0.3, 0.5, 0.75, 1.0];

#[component]
pub fn RadiusPicker(radius: RwSignal<f32>) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <small class="text-sm font-medium leading-none">"Radius"</small>
            <div class="flex gap-1 items-center">
                {RADII
                    .iter()
                    .map(|&r| {
                        let is_active = move || (radius.get() - r).abs() < f32::EPSILON;
                        view! {
                            <button
                                class="inline-flex justify-center items-center px-2.5 h-8 text-xs font-medium rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none ring-offset-background hover:bg-primary hover:text-primary-foreground focus-visible:ring-ring"
                                class:bg-primary=is_active
                                class:text-primary-foreground=is_active
                                on:click=move |_| radius.set(r)
                            >
                                {format!("{r}")}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
