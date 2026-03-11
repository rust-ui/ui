use icons::{AlignCenter, AlignLeft, AlignRight};
use leptos::prelude::*;

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation};

#[component]
pub fn DemoToggleGroupVertical() -> impl IntoView {
    let align = RwSignal::new("left");

    view! {
        <ToggleGroup orientation=ToggleGroupOrientation::Vertical>
            <ToggleGroupItem
                title="Align Left"
                pressed=Signal::derive(move || align.get() == "left")
                on:click=move |_| align.set("left")
            >
                <AlignLeft />
                "Left"
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Align Center"
                pressed=Signal::derive(move || align.get() == "center")
                on:click=move |_| align.set("center")
            >
                <AlignCenter />
                "Center"
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Align Right"
                pressed=Signal::derive(move || align.get() == "right")
                on:click=move |_| align.set("right")
            >
                <AlignRight />
                "Right"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
