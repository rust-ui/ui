use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::field::{Field, FieldVariant};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldInputInline() -> Element {
    rsx! {
        Field { variant: FieldVariant::Horizontal,
            Input { r#type: crate::ui::input::InputType::Search, placeholder: "Search..." }
            Button { "Search" }
        }
    }
}
