use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::{Input, InputType};

#[component]
pub fn DemoInputRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="space-y-4 w-full max-w-sm">
                <Input placeholder="أدخل النص هنا" />
                <Input r#type=InputType::Email placeholder="البريد الإلكتروني" />
                <Input r#type=InputType::Password placeholder="كلمة المرور" />
            </div>
        </DirectionProvider>
    }
}
