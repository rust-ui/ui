use dioxus::prelude::*;

use crate::ui::input_phone::{Country, InputPhone, PhoneNumber};

#[component]
pub fn DemoInputPhoneDisabled() -> Element {
    let phone_signal = use_signal(|| PhoneNumber::new("0612345678", 10));
    let country_signal = use_signal(|| Country::France);

    rsx! {
        div { class: "flex flex-col gap-4 w-full max-w-sm",
            div { class: "space-y-2",
                label { class: "text-sm font-medium", "Disabled" }
                InputPhone { value_signal: phone_signal, country_signal, disabled: true }
            }
        }
    }
}
