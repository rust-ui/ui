use icons::{Bold, Italic, Underline};
use leptos::prelude::*;

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroup() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <ToggleGroup>
            <ToggleGroupItem title="Bold" pressed=bold on:click=move |_| bold.update(|v| *v = !*v)>
                <Bold />
            </ToggleGroupItem>
            <ToggleGroupItem title="Italic" pressed=italic on:click=move |_| italic.update(|v| *v = !*v)>
                <Italic />
            </ToggleGroupItem>
            <ToggleGroupItem title="Underline" pressed=underline on:click=move |_| underline.update(|v| *v = !*v)>
                <Underline />
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
