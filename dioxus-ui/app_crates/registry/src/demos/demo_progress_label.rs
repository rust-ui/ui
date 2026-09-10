use dioxus::prelude::*;

use crate::ui::field::{Field, FieldLabel};
use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressLabel() -> Element {
    rsx! {
        Field { class: "w-full max-w-sm",
            FieldLabel { r#for: "progress-upload",
                span { "Upload progress" }
                span { class: "ml-auto", "66%" }
            }
            Progress { value: 66.0 }
        }
    }
}
