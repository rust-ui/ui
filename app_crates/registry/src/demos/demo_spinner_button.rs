use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::spinner::SpinnerCircle;

#[component]
pub fn DemoSpinnerButton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center">
            <Button attr:disabled=true>
                <SpinnerCircle />
                <span>"Loading..."</span>
            </Button>
            <Button variant=ButtonVariant::Outline attr:disabled=true>
                <SpinnerCircle />
                <span>"Please wait"</span>
            </Button>
            <Button variant=ButtonVariant::Secondary attr:disabled=true>
                <SpinnerCircle />
                <span>"Processing"</span>
            </Button>
        </div>
    }
}
