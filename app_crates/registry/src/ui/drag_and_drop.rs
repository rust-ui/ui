use leptos::prelude::*;
use leptos::wasm_bindgen::closure::Closure;
use leptos_ui::clx;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, HtmlElement};

mod components {
    use super::*;
    clx! {DraggableZone, div, "dragabble__container", "bg-neutral-600 p-4 mt-4"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ COMPONENTS ✨                       */
/* ========================================================== */

/// Outer wrapper. Sets up drag event delegation on `document` via `Effect::new`,
/// which runs after Leptos WASM hydration — so listeners are never stripped by
/// node replacement the way vanilla JS attached-at-parse-time listeners are.
#[component]
pub fn Draggable(
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    Effect::new(move |_| {
        let Some(document) = window().document() else { return };

        // dragstart — mark the element being dragged
        let dragstart = Closure::<dyn Fn(DragEvent)>::new(|e: DragEvent| {
            let Some(target) = e.target() else { return };
            let Some(el) = target.dyn_ref::<HtmlElement>() else { return };
            if el.class_list().contains("draggable") {
                let _ = el.class_list().add_1("dragging");
            }
        });
        let _ = document.add_event_listener_with_callback(
            "dragstart",
            dragstart.as_ref().unchecked_ref(),
        );
        dragstart.forget();

        // dragend — unmark the dragged element
        let dragend = Closure::<dyn Fn(DragEvent)>::new(|e: DragEvent| {
            let Some(target) = e.target() else { return };
            let Some(el) = target.dyn_ref::<HtmlElement>() else { return };
            if el.class_list().contains("draggable") {
                let _ = el.class_list().remove_1("dragging");
            }
        });
        let _ = document.add_event_listener_with_callback(
            "dragend",
            dragend.as_ref().unchecked_ref(),
        );
        dragend.forget();

        // dragover — move the dragging element to the correct position in the zone
        let dragover = Closure::<dyn Fn(DragEvent)>::new(move |e: DragEvent| {
            let Some(target) = e.target() else { return };
            let Some(el) = target.dyn_ref::<Element>() else { return };
            let Ok(Some(container)) = el.closest(".dragabble__container") else { return };
            e.prevent_default();

            let Some(doc) = window().document() else { return };
            let Ok(Some(dragging)) = doc.query_selector(".dragging") else { return };

            let after = get_drag_after_element(&container, f64::from(e.client_y()));
            if let Some(after_el) = after {
                let _ = container.insert_before(
                    dragging.unchecked_ref(),
                    Some(after_el.unchecked_ref()),
                );
            } else {
                let _ = container.append_child(dragging.unchecked_ref());
            }
        });
        let _ = document.add_event_listener_with_callback(
            "dragover",
            dragover.as_ref().unchecked_ref(),
        );
        dragover.forget();
    });

    let merged = tw_merge::tw_merge!("flex flex-col gap-4 w-full max-w-4xl", class);
    view! {
        <div class=merged data-name="Draggable">
            {children()}
        </div>
    }
}

#[component]
pub fn DraggableItem(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div
            class="p-4 border cursor-move border-input bg-card draggable [&.dragging]:opacity-50"
            draggable="true"
            tabindex="0"
            data-name="DraggableItem"
        >
            {text}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Returns the element after which the dragged item should be inserted,
/// based on the cursor's Y position. Returns `None` to append at the end.
fn get_drag_after_element(container: &Element, y: f64) -> Option<HtmlElement> {
    let items = container
        .query_selector_all(".draggable:not(.dragging)")
        .ok()?;

    let mut closest_offset = f64::NEG_INFINITY;
    let mut closest: Option<HtmlElement> = None;

    for i in 0..items.length() {
        let Some(node) = items.get(i) else { continue };
        let Ok(el) = node.dyn_into::<HtmlElement>() else { continue };
        let rect = el.get_bounding_client_rect();
        let offset = y - rect.top() - rect.height() / 2.0;
        if offset < 0.0 && offset > closest_offset {
            closest_offset = offset;
            closest = Some(el);
        }
    }

    closest
}
