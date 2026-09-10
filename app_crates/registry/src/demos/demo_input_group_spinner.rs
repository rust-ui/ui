use dioxus::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};
use crate::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoInputGroupSpinner() -> Element {
    rsx! {
        div { class: "grid gap-4 w-full max-w-sm",
            InputGroup {
                InputGroupInput { placeholder: "Searching..." }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    Spinner {}
                }
            }

            InputGroup {
                InputGroupAddon {
                    SpinnerCircle {}
                }
                InputGroupInput { placeholder: "Processing..." }
            }

            InputGroup {
                InputGroupInput { placeholder: "Saving changes..." }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    InputGroupText { "Saving..." }
                    Spinner {}
                }
            }
        }
    }
}
