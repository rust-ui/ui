use dioxus::prelude::*;
use tw_merge::tw_merge;
use wasm_bindgen::JsCast;
use web_sys::Element as WebElement;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

#[component]
pub fn DraggableZone(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("dragabble__container bg-neutral-600 p-4 mt-4", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-name": "DraggableZone", class: "{merged}", {children} }
    }
}

/* ========================================================== */
/*                     COMPONENTS                             */
/* ========================================================== */

/// Outer wrapper. Sets up drag event delegation on `document` via `use_effect`,
/// which runs after Dioxus WASM hydration.
#[component]
pub fn Draggable(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;

            let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };

            // dragstart — mark the element being dragged
            let dragstart = Closure::<dyn Fn(web_sys::DragEvent)>::new(move |e: web_sys::DragEvent| {
                if let Some(target) = e.target() {
                    if let Ok(el) = target.dyn_into::<HtmlElement>() {
                        let _ = el.class_list().add_1("dragging");
                    }
                }
            });
            let _ = document.add_event_listener_with_callback("dragstart", dragstart.as_ref().unchecked_ref());
            dragstart.forget();

            // dragend — remove dragging class
            let dragend = Closure::<dyn Fn(web_sys::DragEvent)>::new(move |e: web_sys::DragEvent| {
                if let Some(target) = e.target() {
                    if let Ok(el) = target.dyn_into::<HtmlElement>() {
                        let _ = el.class_list().remove_1("dragging");
                    }
                }
            });
            let _ = document.add_event_listener_with_callback("dragend", dragend.as_ref().unchecked_ref());
            dragend.forget();

            // dragover — reorder items
            let dragover = Closure::<dyn Fn(web_sys::DragEvent)>::new(move |e: web_sys::DragEvent| {
                e.prevent_default();
                // Find the closest container
                let target = e.target().and_then(|t| t.dyn_into::<HtmlElement>().ok());
                let container = target.and_then(|el| el.closest("[data-name='DraggableZone']").ok().flatten());
                let Some(container) = container else { return };

                let dragging =
                    container.query_selector(".dragging").ok().flatten().and_then(|n| n.dyn_into::<HtmlElement>().ok());
                let Some(dragging_el) = dragging else { return };

                let after_el = get_drag_after_element(&container, e.client_y() as f64);

                match after_el {
                    None => {
                        let _ = container.append_child(&dragging_el);
                    }
                    Some(after) => {
                        let _ = container.insert_before(&dragging_el, Some(&after));
                    }
                }
            });
            let _ = document.add_event_listener_with_callback("dragover", dragover.as_ref().unchecked_ref());
            dragover.forget();
        }
    });

    let merged = tw_merge!("flex flex-col gap-4 w-full max-w-4xl", class.as_deref().unwrap_or(""));
    rsx! {
        div { class: "{merged}", "data-name": "Draggable", {children} }
    }
}

#[component]
pub fn DraggableItem(#[props(into)] text: String) -> Element {
    rsx! {
        div {
            class: "p-4 border cursor-move border-input bg-card draggable [&.dragging]:opacity-50",
            draggable: "true",
            tabindex: "0",
            "data-name": "DraggableItem",
            "{text}"
        }
    }
}

/* ========================================================== */
/*                     HELPERS                                */
/* ========================================================== */

/// Returns the element after which the dragged item should be inserted,
/// based on the cursor's Y position. Returns `None` to append at the end.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn get_drag_after_element(container: &WebElement, y: f64) -> Option<WebElement> {
    let items = container.query_selector_all(".draggable:not(.dragging)").ok()?;

    let mut closest_offset = f64::NEG_INFINITY;
    let mut closest: Option<WebElement> = None;

    for i in 0..items.length() {
        let Some(node) = items.get(i) else { continue };
        let Ok(el) = node.dyn_into::<WebElement>() else { continue };
        let top = js_sys::Reflect::get(el.as_ref(), &wasm_bindgen::JsValue::from_str("offsetTop"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let height = js_sys::Reflect::get(el.as_ref(), &wasm_bindgen::JsValue::from_str("offsetHeight"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let offset = y - top - height / 2.0;
        if offset < 0.0 && offset > closest_offset {
            closest_offset = offset;
            closest = Some(el);
        }
    }

    closest
}
