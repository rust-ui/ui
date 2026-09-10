use dioxus::prelude::*;
use icons::{Bell, RefreshCcw};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyBackground() -> Element {
    rsx! {
        Empty { class: "bg-muted/30",
            EmptyHeader {
                EmptyMedia { variant: EmptyMediaVariant::Icon,
                    Bell {}
                }
                EmptyTitle { "No Notifications" }
                EmptyDescription { "You're all caught up. New notifications will appear here." }
            }
            EmptyContent {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
                    RefreshCcw {}
                    "Refresh"
                }
            }
        }
    }
}
