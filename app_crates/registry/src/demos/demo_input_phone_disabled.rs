use leptos::prelude::*;

use crate::ui::input_phone::InputPhone;
use crate::utils::country::Country;
use crate::utils::phone_number::PhoneNumber;

#[component]
pub fn DemoInputPhoneDisabled() -> impl IntoView {
    let phone_signal = RwSignal::new(PhoneNumber::new("0612345678", 10));
    let country_signal = RwSignal::new(Country::France);

    view! {
        <div class="flex flex-col gap-4 w-full max-w-sm">
            <div class="space-y-2">
                <label class="text-sm font-medium">"Disabled"</label>
                <InputPhone value_signal=phone_signal country_signal=country_signal disabled=true />
            </div>
        </div>
    }
}
