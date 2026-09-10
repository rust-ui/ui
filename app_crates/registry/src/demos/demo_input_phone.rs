use dioxus::prelude::*;

use crate::ui::input_phone::{Country, InputPhone, PhoneNumber};

#[component]
pub fn DemoInputPhone() -> Element {
    let phone_signal = use_signal(PhoneNumber::default);
    let country_signal = use_signal(|| Country::UnitedStatesOfAmerica);

    rsx! {
        div { class: "flex flex-col gap-2 w-full max-w-sm",
            InputPhone { value_signal: phone_signal, country_signal }
            p { class: "text-sm text-muted-foreground",
                {
                    let phone = phone_signal();
                    let country = country_signal();
                    if phone.is_empty() {
                        "Enter a phone number".to_string()
                    } else {
                        phone.format_international(country)
                    }
                }
            }
        }
    }
}
