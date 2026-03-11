use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::radio_button_group::{RadioButton, RadioButtonGroup, RadioButtonText};

#[component]
pub fn DemoRadioButtonGroupRtl() -> impl IntoView {
    view! {
        <style>
            {".radio__button[type=\"radio\"] {
            clip: rect(0 0 0 0);
            clip-path: inset(100%);
            height: 1px;
            overflow: hidden;
            position: absolute;
            white-space: nowrap;
            width: 1px;
            }
            
            .radio__button[type=\"radio\"]:checked + span {
            box-shadow: 0 0 0 0.0625em var(--primary);
            background-color: var(--secondary);
            z-index: 1;
            color: var(--primary);
            }"}
        </style>

        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <RadioButtonGroup>
                <RadioButton checked=true>
                    <RadioButtonText>"نساء"</RadioButtonText>
                </RadioButton>
                <RadioButton>
                    <RadioButtonText>"رجال"</RadioButtonText>
                </RadioButton>
                <RadioButton>
                    <RadioButtonText>"الكل"</RadioButtonText>
                </RadioButton>
            </RadioButtonGroup>
        </DirectionProvider>
    }
}
