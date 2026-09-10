use dioxus::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoMarkerStatus() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Marker { role: "status",
                MarkerIcon { Spinner {} }
                MarkerContent { "Compacting conversation" }
            }
            Marker { variant: MarkerVariant::Separator, role: "status",
                MarkerIcon { Spinner {} }
                MarkerContent { "Running tests" }
            }
        }
    }
}
