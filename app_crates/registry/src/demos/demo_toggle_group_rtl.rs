use icons::{Bold, Italic, Underline};
use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroupRtl() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <ToggleGroup>
                <ToggleGroupItem
                    title="غامق"
                    pressed=Signal::derive(move || bold.get())
                    on:click=move |_| bold.update(|v| *v = !*v)
                >
                    <Bold />
                </ToggleGroupItem>
                <ToggleGroupItem
                    title="مائل"
                    pressed=Signal::derive(move || italic.get())
                    on:click=move |_| italic.update(|v| *v = !*v)
                >
                    <Italic />
                </ToggleGroupItem>
                <ToggleGroupItem
                    title="تسطير"
                    pressed=Signal::derive(move || underline.get())
                    on:click=move |_| underline.update(|v| *v = !*v)
                >
                    <Underline />
                </ToggleGroupItem>
            </ToggleGroup>
        </DirectionProvider>
    }
}
