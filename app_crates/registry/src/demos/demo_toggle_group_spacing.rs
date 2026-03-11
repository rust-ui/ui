use icons::{Bold, Italic, Underline};
use leptos::prelude::*;

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroupSpacing() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <ToggleGroup spacing=0>
            <ToggleGroupItem
                title="Bold"
                pressed=Signal::derive(move || bold.get())
                on:click=move |_| bold.update(|v| *v = !*v)
            >
                <Bold />
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Italic"
                pressed=Signal::derive(move || italic.get())
                on:click=move |_| italic.update(|v| *v = !*v)
            >
                <Italic />
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Underline"
                pressed=Signal::derive(move || underline.get())
                on:click=move |_| underline.update(|v| *v = !*v)
            >
                <Underline />
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
