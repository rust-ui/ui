use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};

const BUDGETS: &[&str] = &["<$1K", "$1K - $2K", "$2K - $5K", "$5K - $10K", ">$10K"];

#[component]
pub fn DemoRadioButtonCustom() -> Element {
    let mut selected = use_signal(|| "$2K - $5K".to_string());

    rsx! {
        RadioGroup { value: selected,
            for budget in BUDGETS.iter() {
                div { class: "flex gap-3 items-center",
                    RadioGroupItem { value: *budget, id: *budget }
                    Label { html_for: *budget, {*budget} }
                }
            }
        }
    }
}
