use dioxus::prelude::*;

use crate::ui::slider::Slider;

#[component]
pub fn DemoSliderMultiple() -> Element {
    rsx! {
        div { class: "flex flex-col gap-5 w-80",
            div { class: "flex flex-col gap-1.5",
                div { class: "flex justify-between text-sm",
                    span { class: "text-muted-foreground", "Low" }
                    span { class: "font-medium tabular-nums", "10" }
                }
                Slider { value: 10.0 }
            }
            div { class: "flex flex-col gap-1.5",
                div { class: "flex justify-between text-sm",
                    span { class: "text-muted-foreground", "Mid" }
                    span { class: "font-medium tabular-nums", "20" }
                }
                Slider { value: 20.0 }
            }
            div { class: "flex flex-col gap-1.5",
                div { class: "flex justify-between text-sm",
                    span { class: "text-muted-foreground", "High" }
                    span { class: "font-medium tabular-nums", "70" }
                }
                Slider { value: 70.0 }
            }
        }
    }
}
