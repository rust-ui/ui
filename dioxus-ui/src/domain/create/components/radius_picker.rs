use dioxus::prelude::*;

const RADII: &[f32] = &[0.0, 0.3, 0.5, 0.75, 1.0];

#[component]
pub fn RadiusPicker(mut radius: Signal<f32>) -> Element {
    rsx! {
        div { class: "space-y-2",
            small { class: "text-sm font-medium leading-none", "Radius" }
            div { class: "flex gap-1 items-center",
                {RADII.iter().map(|&r| {
                    let is_active = move || (radius() - r).abs() < f32::EPSILON;
                    rsx! {
                        button {
                            key: "{r}",
                            class: "inline-flex justify-center items-center px-2.5 h-8 text-xs font-medium rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none ring-offset-background hover:bg-primary hover:text-primary-foreground focus-visible:ring-ring",
                            class: if is_active() { "bg-primary text-primary-foreground" },
                            onclick: move |_| radius.set(r),
                            "{r}"
                        }
                    }
                })}
            }
        }
    }
}
