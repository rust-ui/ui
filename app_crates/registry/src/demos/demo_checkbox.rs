use leptos::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::label::Label;

#[component]
pub fn DemoCheckbox() -> impl IntoView {
    let item1_signal = RwSignal::new(true);
    let item2_signal = RwSignal::new(false);

    view! {
        <div class="flex flex-col gap-4">
            <div class="flex gap-2 items-center">
                <Checkbox
                    attr:id="item1"
                    checked=item1_signal
                    on_checked_change=Callback::new(move |checked| {
                        item1_signal.set(checked);
                    })
                />
                <Label html_for="item1">Item 1</Label>
            </div>
            <div class="flex gap-2 items-center">
                <Checkbox
                    attr:id="item2"
                    checked=item2_signal
                    on_checked_change=Callback::new(move |checked| {
                        item2_signal.set(checked);
                    })
                />
                <Label html_for="item2">Item 2</Label>
            </div>
            <div class="flex gap-2 items-center">
                <Checkbox attr:id="disabled1" checked=true disabled=true />
                <Label html_for="disabled1">Disabled</Label>
            </div>
            <div class="flex gap-2 items-center">
                <Checkbox attr:id="disabled2" disabled=true />
                <Label html_for="disabled2">Disabled</Label>
            </div>
        </div>
    }
}
