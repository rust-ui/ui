use leptos::prelude::*;

use crate::ui::textarea::Textarea;

#[component]
pub fn DemoTextarea() -> impl IntoView {
    view! {
        <div>
            <Textarea attr:placeholder="Type your message here." />
        </div>
    }
}
