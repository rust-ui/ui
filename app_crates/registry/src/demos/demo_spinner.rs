use dioxus::prelude::*;

use crate::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoSpinner() -> Element {
    rsx! {
        div { class: "flex gap-4",
            Spinner {}
            SpinnerCircle {}
        }
    }
}
