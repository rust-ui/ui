use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Drag-resize never eats more than this fraction of the container width.
const MAX_RATIO: f64 = 0.8;

/// Screen-size presets a resizable panel can be snapped to. App code maps its
/// own screen-size enum (e.g. a toolbar's Desktop/Tablet/Phone toggle) onto this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizablePreset {
    /// Full width: no panel reserved.
    Desktop,
    /// Reserve space so the remaining preview is roughly tablet-sized.
    Tablet,
    /// Reserve space so the remaining preview is roughly phone-sized.
    Phone,
}

impl ResizablePreset {
    /// Target width (px) of the *preview* pane for this preset. `Desktop` has
    /// no cap, so it collapses the resizable background panel back to 0.
    const fn target_preview_width(self) -> f64 {
        match self {
            Self::Desktop => f64::MAX,
            Self::Tablet => 768.0,
            Self::Phone => 390.0,
        }
    }
}

/// State returned by `use_resizable`.
#[derive(Clone, Copy)]
pub struct ResizableState {
    /// Current width (px) of the resizable background panel. 0 = collapsed.
    pub background_width: ReadSignal<f64>,
}

type PointerClosure = Closure<dyn FnMut(web_sys::PointerEvent)>;

/// Toggles the drag-in-progress cursor/selection lockout on `<html>`/`<body>`.
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
fn set_drag_cursor_active(active: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = web_sys::window() else { return };
        let Some(document) = window.document() else { return };

        if let Some(html) = document
            .document_element()
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        {
            if active {
                let _ = html.style().set_property("cursor", "col-resize");
            } else {
                let _ = html.style().remove_property("cursor");
            }
        }
        if let Some(body) = document.body() {
            if active {
                let _ = body.class_list().add_2("pointer-events-none", "select-none");
            } else {
                let _ = body.class_list().remove_2("pointer-events-none", "select-none");
            }
        }
    }
}

/// Detaches the `pointermove`/`pointerup` listeners stashed by a drag session.
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
fn remove_drag_listeners(
    move_holder: &Rc<RefCell<Option<PointerClosure>>>,
    up_holder: &Rc<RefCell<Option<PointerClosure>>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };

    if let Some(c) = move_holder.borrow_mut().take() {
        let _ = document.remove_event_listener_with_callback("pointermove", c.as_ref().unchecked_ref());
    }
    if let Some(c) = up_holder.borrow_mut().take() {
        let _ = document.remove_event_listener_with_callback("pointerup", c.as_ref().unchecked_ref());
    }
    }
}

/// Hook driving a `Resizable`/`ResizableHandle`/`ResizableBackground` group.
///
/// Ports `resizable.js`'s pointer-drag resize logic to Dioxus/web-sys, plus
/// adds screen-size presets (previously a broken `CustomEvent` bridge to a
/// JS file that was never loaded).
///
/// # Arguments
/// * `container_element` - the `Resizable` root element (set from its `onmounted`)
/// * `handle_element` - the `ResizableHandle` element (set from its `onmounted`)
/// * `preset` - current screen-size preset; changing it snaps the panel width
pub fn use_resizable(
    container_element: ReadSignal<Option<web_sys::Element>>,
    handle_element: ReadSignal<Option<web_sys::Element>>,
    preset: ReadSignal<ResizablePreset>,
) -> ResizableState {
    let background_width = use_signal(|| 0.0_f64);

    let is_mounted = Arc::new(AtomicBool::new(true));
    let is_mounted_cleanup = Arc::clone(&is_mounted);
    use_drop(move || {
        is_mounted_cleanup.store(false, Ordering::SeqCst);
    });

    // Attach the drag machinery once the handle element is available.
    // wasm32-only: `web_sys`/`js_sys` FFI calls panic unconditionally on a
    // native (non-wasm) target, even behind `if let Some(...)` — see
    // use_table_of_contents.rs for the iOS crash this pattern caused.
    let is_mounted_for_drag = Arc::clone(&is_mounted);
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let Some(handle) = handle_element.peek().clone() else {
            return;
        };
        let Some(window) = web_sys::window() else { return };
        let Some(document) = window.document() else { return };

        let move_holder: Rc<RefCell<Option<PointerClosure>>> = Rc::new(RefCell::new(None));
        let up_holder: Rc<RefCell<Option<PointerClosure>>> = Rc::new(RefCell::new(None));

        let is_mounted_for_down = Arc::clone(&is_mounted_for_drag);
        let mut background_width_for_down = background_width;
        let pointerdown_handler = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if !is_mounted_for_down.load(Ordering::SeqCst) {
                return;
            }
            e.prevent_default();
            let Some(container) = container_element.peek().clone() else {
                return;
            };
            let container_width = container.get_bounding_client_rect().width();
            let start_pos = f64::from(e.client_x());
            let start_size = *background_width_for_down.peek();

            set_drag_cursor_active(true);

            let is_mounted_for_move = Arc::clone(&is_mounted_for_down);
            let mut background_width_for_move = background_width_for_down;
            let do_resize = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
                if !is_mounted_for_move.load(Ordering::SeqCst) {
                    return;
                }
                let delta = start_pos - f64::from(e.client_x());
                let new_size = (start_size + delta).clamp(0.0, container_width * MAX_RATIO);
                background_width_for_move.set(new_size);
            }) as Box<dyn FnMut(web_sys::PointerEvent)>);

            let move_holder_for_stop = Rc::clone(&move_holder);
            let up_holder_for_stop = Rc::clone(&up_holder);
            let stop_resize = Closure::wrap(Box::new(move |_e: web_sys::PointerEvent| {
                set_drag_cursor_active(false);
                remove_drag_listeners(&move_holder_for_stop, &up_holder_for_stop);
            }) as Box<dyn FnMut(web_sys::PointerEvent)>);

            if let Some(window) = web_sys::window()
                && let Some(document) = window.document()
            {
                let _ = document.add_event_listener_with_callback("pointermove", do_resize.as_ref().unchecked_ref());
                let _ = document.add_event_listener_with_callback("pointerup", stop_resize.as_ref().unchecked_ref());
            }
            *move_holder.borrow_mut() = Some(do_resize);
            *up_holder.borrow_mut() = Some(stop_resize);
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);

        let _ = handle.add_event_listener_with_callback("pointerdown", pointerdown_handler.as_ref().unchecked_ref());
        pointerdown_handler.forget();
    });

    // Snap to a screen-size preset whenever it changes.
    use_effect(move || {
        let target = preset();
        let Some(container) = container_element.peek().clone() else {
            return;
        };
        let container_width = container.get_bounding_client_rect().width();
        let target_width = target.target_preview_width();
        let new_bg = (container_width - target_width).clamp(0.0, container_width * MAX_RATIO);
        background_width.clone().set(new_bg);
    });

    ResizableState {
        background_width: background_width.into(),
    }
}
