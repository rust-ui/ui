use leptos::prelude::*;
use strum::{IntoStaticStr, VariantArray};

use crate::ui::label::Label;
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};

#[derive(Clone, Copy, Default, VariantArray, IntoStaticStr)]
enum Frequency {
    Daily,
    #[default]
    Weekly,
    Monthly,
}

#[component]
pub fn DemoRadioButton() -> impl IntoView {
    let default: &'static str = Frequency::default().into();
    let value_signal = RwSignal::new(default.to_string());

    view! {
        <RadioGroup value=value_signal>
            {Frequency::VARIANTS
                .iter()
                .map(|variant| {
                    let name: &'static str = (*variant).into();
                    view! {
                        <div class="flex gap-3 items-center">
                            <RadioGroupItem value=name id=name />
                            <Label html_for=name>{name}</Label>
                        </div>
                    }
                })
                .collect_view()}
        </RadioGroup>
    }
}
