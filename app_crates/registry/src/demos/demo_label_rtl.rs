use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoLabelRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-2">
                <Label html_for="rtl-label-input">"اسم المستخدم"</Label>
                <Input attr:id="rtl-label-input" placeholder="أدخل اسم المستخدم" />
            </div>
        </DirectionProvider>
    }
}
