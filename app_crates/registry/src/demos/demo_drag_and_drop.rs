use leptos::prelude::*;

use crate::ui::drag_and_drop::{Draggable, DraggableItem, DraggableZone};

#[component]
pub fn DemoDragAndDrop() -> impl IntoView {
    view! {
        <style>
            {".draggable.dragging {
            opacity: 0.5;
            }
            "}
        </style>

        <Draggable class="max-w-2xl">
            <DraggableZone>
                <DraggableItem text="1" />
                <DraggableItem text="2" />
            </DraggableZone>
            <DraggableZone>
                <DraggableItem text="3" />
                <DraggableItem text="4" />
            </DraggableZone>
        </Draggable>

        <script src="/components/drag_and_drop.js"></script>
    }
}
