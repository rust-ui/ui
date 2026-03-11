use leptos::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButtonReactive() -> impl IntoView {
    let count_signal = RwSignal::new(0);
    let increment = move |_| *count_signal.write() += 1;

    view! { <Button on:click=increment>"Click Me: " {count_signal}</Button> }
}
