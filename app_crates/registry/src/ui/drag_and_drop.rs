use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Draggable, div, "flex flex-col gap-4 w-full max-w-4xl"}
    clx! {DraggableZone, div, "dragabble__container", "bg-neutral-600 p-4 mt-4"}

    // TODO. ItemRoot (needs `draggable` as clx attribute).
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DraggableItem(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="p-4 border cursor-move border-input bg-card draggable" draggable="true">
            {text}
        </div>
    }
}
