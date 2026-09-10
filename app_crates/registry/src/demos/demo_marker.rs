use dioxus::prelude::*;
use icons::{GitBranch, Search};

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoMarker() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Marker {
                MarkerIcon { GitBranch {} }
                MarkerContent { "Switched to a new branch" }
            }
            Marker { role: "status",
                MarkerIcon { Spinner {} }
                MarkerContent { class: "shimmer", "Thinking..." }
            }
            Marker { variant: MarkerVariant::Separator,
                MarkerContent { "Conversation compacted" }
            }
            Marker {
                MarkerIcon { Search {} }
                MarkerContent { "Explored 4 files" }
            }
        }
    }
}
