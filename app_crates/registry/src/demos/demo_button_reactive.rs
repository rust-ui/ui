use dioxus::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButtonReactive() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Button { onclick: move |_| count += 1, "Click Me: {count}" }
    }
}
