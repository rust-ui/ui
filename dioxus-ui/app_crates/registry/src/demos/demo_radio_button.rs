use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};

const FREQUENCIES: &[&str] = &["Daily", "Weekly", "Monthly"];

#[component]
pub fn DemoRadioButton() -> Element {
    let value = use_signal(|| "Weekly".to_string());

    rsx! {
        RadioGroup { value,
            for frequency in FREQUENCIES.iter() {
                div { class: "flex gap-3 items-center",
                    RadioGroupItem { value: *frequency, id: *frequency }
                    Label { html_for: *frequency, {*frequency} }
                }
            }
        }
    }
}
