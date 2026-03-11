use leptos::prelude::*;

use crate::ui::radio_button_group::{RadioButton, RadioButtonGroup, RadioButtonText};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoRadioButtonGroup() -> impl IntoView {
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

        <RadioButtonGroup>
            <RadioButton checked=true>
                <RadioButtonText>Women</RadioButtonText>
            </RadioButton>
            <RadioButton>
                <RadioButtonText>Men</RadioButtonText>
            </RadioButton>
            <RadioButton>
                <RadioButtonText>Divided</RadioButtonText>
            </RadioButton>
        </RadioButtonGroup>
    }
}
