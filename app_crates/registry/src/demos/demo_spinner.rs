use leptos::prelude::*;

use crate::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoSpinner() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Spinner />
            <SpinnerCircle />
        </div>
    }
}
