use dioxus::prelude::*;

use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressControlled() -> Element {
    let mut value = use_signal(|| 50.0_f64);

    rsx! {
        div { class: "flex overflow-hidden flex-col gap-4 w-60",
            Progress { value: value() }
            input {
                r#type: "range",
                min: "0",
                max: "100",
                step: "1",
                value: "{value}",
                class: "w-full accent-primary",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f64>() {
                        value.set(v);
                    }
                },
            }
        }
    }
}
