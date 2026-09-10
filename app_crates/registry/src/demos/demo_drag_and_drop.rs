use dioxus::prelude::*;

use crate::ui::drag_and_drop::{Draggable, DraggableItem, DraggableZone};

#[component]
pub fn DemoDragAndDrop() -> Element {
    rsx! {
        Draggable { class: "max-w-2xl",
            DraggableZone {
                DraggableItem { text: "1" }
                DraggableItem { text: "2" }
            }
            DraggableZone {
                DraggableItem { text: "3" }
                DraggableItem { text: "4" }
            }
        }
    }
}
