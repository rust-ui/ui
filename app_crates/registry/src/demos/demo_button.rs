use leptos::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButton() -> impl IntoView {
    view! { <Button>"Button"</Button> }
}
