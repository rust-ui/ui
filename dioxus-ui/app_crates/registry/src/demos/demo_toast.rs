use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoToast() -> Element {
    let toast_me = move |_| {
        show_toast().info("This is a toast!");
    };

    rsx! {
        Button { onclick: toast_me, "Toast me" }
    }
}
