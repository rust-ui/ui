use dioxus::prelude::*;
use icons::{BookOpenCheck, GitBranch, Search};

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerIcon() -> Element {
    rsx! {
        div { class: "flex flex-col gap-12 py-12 w-full max-w-sm",
            Marker {
                MarkerIcon { GitBranch {} }
                MarkerContent { "Switched to a new branch" }
            }
            Marker { variant: MarkerVariant::Separator,
                MarkerIcon { Search {} }
                MarkerContent { "Explored 4 files" }
            }
            Marker { class: "flex-col",
                MarkerIcon { BookOpenCheck {} }
                MarkerContent { "Syncing completed" }
            }
        }
    }
}
