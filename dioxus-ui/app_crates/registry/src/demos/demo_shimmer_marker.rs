use dioxus::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoShimmerMarker() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-full max-w-sm",
            Marker { role: "status",
                MarkerIcon { Spinner {} }
                MarkerContent { class: "shimmer", "Thinking..." }
            }
            Marker { variant: MarkerVariant::Separator, role: "status",
                MarkerContent { class: "shimmer", "Reading 4 files" }
            }
        }
    }
}
