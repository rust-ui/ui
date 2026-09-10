use dioxus::prelude::*;

use crate::hooks::use_lock_body_scroll::use_lock_body_scroll;
use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoUseLockBodyScroll() -> Element {
    let mut scroll_locked = use_lock_body_scroll(false);

    rsx! {
        Card { class: "mx-auto w-full max-w-md",
            CardHeader {
                CardTitle { "Body Scroll Lock Demo" }
                CardDescription {
                    "Try scrolling the page. Click the button to lock or unlock body scrolling."
                }
            }
            CardContent { class: "space-y-4",
                div { class: "flex justify-between items-center",
                    span { class: "text-sm font-medium", "Scroll Status:" }
                    span { class: "py-1 px-2 text-sm rounded-full",
                        if scroll_locked() { "Locked" } else { "Unlocked" }
                    }
                }

                Button {
                    class: "w-full",
                    onclick: move |_| scroll_locked.toggle(),
                    if scroll_locked() {
                        "Unlock Body Scroll"
                    } else {
                        "Lock Body Scroll"
                    }
                }
            }
        }
    }
}
