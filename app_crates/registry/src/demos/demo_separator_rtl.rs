use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::separator::{Separator, SeparatorOrientation};

#[component]
pub fn DemoSeparatorRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-4">
                <h3 class="text-2xl font-bold text-pretty">"Rust/UI"</h3>
                <p>"مكتبة مكونات واجهة مستخدم مفتوحة المصدر 🦀."</p>

                <Separator />

                <div class="flex gap-4 items-center h-5">
                    <p>"المدونة"</p>
                    <Separator orientation=SeparatorOrientation::Vertical />
                    <p>"التوثيق"</p>
                    <Separator orientation=SeparatorOrientation::Vertical />
                    <p>"المصدر"</p>
                </div>
            </div>
        </DirectionProvider>
    }
}
