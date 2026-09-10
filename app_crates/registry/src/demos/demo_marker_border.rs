use dioxus::prelude::*;
use icons::{FileText, GitBranch, Search};

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerBorder() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 py-12 w-full max-w-sm",
            Marker { variant: MarkerVariant::Border,
                MarkerIcon { GitBranch {} }
                MarkerContent { "Switched to release-candidate" }
            }
            Marker { variant: MarkerVariant::Border,
                MarkerIcon { Search {} }
                MarkerContent { "Reviewed 8 related files" }
            }
            Marker { variant: MarkerVariant::Border,
                MarkerIcon { FileText {} }
                MarkerContent { "Opened implementation notes" }
            }
        }
    }
}
