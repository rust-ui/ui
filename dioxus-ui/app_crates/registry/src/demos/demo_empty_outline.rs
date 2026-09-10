use dioxus::prelude::*;
use icons::{Cloud, Upload};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyOutline() -> Element {
    rsx! {
        Empty {
            EmptyHeader {
                EmptyMedia { variant: EmptyMediaVariant::Icon,
                    Cloud {}
                }
                EmptyTitle { "Cloud Storage Empty" }
                EmptyDescription { "Upload files to your cloud storage to access them anywhere." }
            }
            EmptyContent {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
                    Upload {}
                    "Upload Files"
                }
            }
        }
    }
}
