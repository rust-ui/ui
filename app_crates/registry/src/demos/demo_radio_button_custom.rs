use leptos::prelude::*;
use strum::{IntoStaticStr, VariantArray};

use crate::ui::label::Label;
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};

#[derive(Clone, Copy, Default, VariantArray, IntoStaticStr)]
enum Budget {
    #[strum(serialize = "<$1K")]
    LessThan1K,
    #[strum(serialize = "$1K - $2K")]
    Between1KAnd2K,
    #[default]
    #[strum(serialize = "$2K - $5K")]
    Between2KAnd5K,
    #[strum(serialize = "$5K - $10K")]
    Between5KAnd10K,
    #[strum(serialize = ">$10K")]
    MoreThan10K,
}

#[component]
pub fn DemoRadioButtonCustom() -> impl IntoView {
    let default: &'static str = Budget::default().into();
    let value_signal = RwSignal::new(default.to_string());

    view! {
        <RadioGroup value=value_signal>
            {Budget::VARIANTS
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
