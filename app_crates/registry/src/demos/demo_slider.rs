use dioxus::prelude::*;

use crate::ui::slider::Slider;

#[component]
pub fn DemoSlider() -> Element {
    let mut value = use_signal(|| 50.0_f64);

    rsx! {
        div { class: "flex flex-col gap-4 w-64",
            Slider {
                value: value(),
                oninput: move |e: FormEvent| {
                    if let Ok(v) = e.value().parse::<f64>() {
                        value.set(v);
                    }
                },
            }
            p { class: "text-sm text-muted-foreground text-center", "Value: {value():.0}" }
        }
    }
}
