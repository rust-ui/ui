use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoToastVariants() -> impl IntoView {
    let show_success_toast = move |_| {
        show_toast().success("Success!");
    };

    let show_error_toast = move |_| {
        show_toast().error("Error!");
    };

    let show_warning_toast = move |_| {
        show_toast().warning("Warning!");
    };

    view! {
        <div class="flex gap-4">
            <Button variant=ButtonVariant::Success on:click=show_success_toast>
                "Success"
            </Button>
            <Button variant=ButtonVariant::Destructive on:click=show_error_toast>
                "Error"
            </Button>
            <Button variant=ButtonVariant::Warning on:click=show_warning_toast>
                "Warning"
            </Button>
        </div>
    }
}
