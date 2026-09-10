use dioxus::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerSeparator() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Marker { variant: MarkerVariant::Separator, MarkerContent { "Today" } }
            Marker { variant: MarkerVariant::Separator, MarkerContent { "Worked for 42s" } }
            Marker { variant: MarkerVariant::Separator, MarkerContent { "Conversation compacted" } }
        }
    }
}
