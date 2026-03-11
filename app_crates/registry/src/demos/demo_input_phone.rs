use leptos::prelude::*;

use crate::ui::input_phone::InputPhone;
use crate::utils::country::Country;
use crate::utils::phone_number::PhoneNumber;

#[component]
pub fn DemoInputPhone() -> impl IntoView {
    let phone_signal = RwSignal::new(PhoneNumber::default());
    let country_signal = RwSignal::new(Country::UnitedStatesOfAmerica);

    view! {
        <div class="flex flex-col gap-2 w-full max-w-sm">
            <InputPhone value_signal=phone_signal country_signal=country_signal />
            <p class="text-sm text-muted-foreground">
                {move || {
                    let phone = phone_signal.get();
                    let country = country_signal.get();
                    if phone.is_empty() {
                        "Enter a phone number".to_string()
                    } else {
                        phone.format_international(country)
                    }
                }}
            </p>
        </div>
    }
}
