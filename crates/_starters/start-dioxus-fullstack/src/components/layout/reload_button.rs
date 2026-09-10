use dioxus::prelude::*;
use icons::RefreshCw;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

/// Visible only on iOS dev via supports-[-webkit-touch-callout:none].
#[component]
pub fn ReloadButton() -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            class: "hidden transition-transform active:scale-95 supports-[-webkit-touch-callout:none]:block",
            onclick: move |_| {
                let _ = dioxus::document::eval("window.location.reload()");
            },
            RefreshCw { class: "size-5 text-muted-foreground" }
        }
    }
}
