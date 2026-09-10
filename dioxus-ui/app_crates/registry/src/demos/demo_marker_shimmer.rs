use dioxus::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerShimmer() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Marker { role: "status", MarkerContent { class: "shimmer", "Thinking..." } }
            Marker { variant: MarkerVariant::Separator, role: "status",
                MarkerContent { class: "shimmer", "Reading 4 files" }
            }
        }
    }
}
