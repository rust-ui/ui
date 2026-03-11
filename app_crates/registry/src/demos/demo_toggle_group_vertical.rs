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
                pressed=Memo::new(move |_| align.get() == "left")
                on:click=move |_| align.set("left")
            >
                <AlignLeft />
                "Left"
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Align Center"
                pressed=Memo::new(move |_| align.get() == "center")
                on:click=move |_| align.set("center")
            >
                <AlignCenter />
                "Center"
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Align Right"
                pressed=Memo::new(move |_| align.get() == "right")
                on:click=move |_| align.set("right")
            >
                <AlignRight />
                "Right"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
