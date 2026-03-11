use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const OPTIONS: [(&str, &str); 3] = [("components", "المكونات"), ("extensions", "الإضافات"), ("icons", "الأيقونات")];

#[component]
pub fn DemoSelectRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Select>
                <SelectTrigger class="w-[180px]">
                    <SelectValue placeholder="اختر خياراً" />
                </SelectTrigger>

                <SelectContent>
                    <SelectGroup>
                        {OPTIONS
                            .into_iter()
                            .map(|(value, label)| {
                                view! { <SelectOption value=value>{label}</SelectOption> }
                            })
                            .collect_view()}
                    </SelectGroup>
                </SelectContent>
            </Select>
        </DirectionProvider>
    }
}
