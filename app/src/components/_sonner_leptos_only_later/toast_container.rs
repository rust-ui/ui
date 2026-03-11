use std::cmp::max;
use std::time::Duration;

use js_sys::Date;
use leptos::prelude::*;
use leptos::wasm_bindgen::closure::Closure;
use tw_merge::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, PointerEvent};

use crate::components::sonner_leptos::toast_id::ToastId;
use crate::components::sonner_leptos::types::{HeightT, Toast, ToasterPosition, Toasts, decode_message};

#[component]
pub fn ToastContainer(
    index: Signal<usize>,
    toast: Toast,
    duration_from_toaster: Duration,
    visible_toasts: usize,
    position: ToasterPosition,
    #[prop(into)] remove_toast: Callback<ToastId>,
    expanded: ReadSignal<bool>,
    expand_by_default: bool,
    num_toasts: Signal<usize>,
    heights: RwSignal<Vec<HeightT>>,
    gap: usize,
) -> impl IntoView {
    let mounted_signal = RwSignal::new(false);
    let removed_signal = RwSignal::new(false);
    let swiping_signal = RwSignal::new(false);
    let swipe_out_signal = RwSignal::new(false);
    let is_visible = move || index.get() < visible_toasts;
    let is_front = move || index.get() == 0;
    let height_index = move || {
        heights.with(|heights| {
            heights
                .iter()
                .position(|height| height.toast_id == toast.id)
                .unwrap_or(0)
        })
    };
    let toasts_height_before = move || {
        heights.with(|heights| {
            let mut acc = 0.0;
            for height in heights.iter().take(height_index()) {
                acc += height.height;
            }
            acc
        })
    };
    let offset = move || (height_index() * gap) as f64 + toasts_height_before();
    let is_expanded = move || expanded.get() || (expand_by_default && mounted_signal.get());
    let duration = toast.options.duration.unwrap_or(duration_from_toaster);
    let position = toast.options.position.unwrap_or(position);

    let initial_height_signal = RwSignal::new(0.0);
    let offset_before_remove_signal = RwSignal::new(0.0);

    Effect::new(move |_| {
        if let Some(document) = window().document() {
            if let Ok(Some(toast_container_node)) = document.query_selector(".sonner-toast-container") {
                let height = toast_container_node.get_bounding_client_rect().height();
                initial_height_signal.set(height);
                heights.update(|heights| {
                    heights.insert(
                        0,
                        HeightT {
                            toast_id: toast.id,
                            height,
                        },
                    )
                });
            }
        }
    });

    let delete_toast = move || {
        // Prevent double deletion
        if removed_signal.get_untracked() {
            return;
        }
        removed_signal.set(true);
        offset_before_remove_signal.set(offset());
        heights.update(|heights| {
            if let Some(i) = heights.iter().position(|t| t.toast_id == toast.id) {
                heights.remove(i);
            }
        });

        set_timeout(
            move || {
                remove_toast.run(toast.id);
            },
            Duration::from_millis(200),
        );
    };

    // The close button calls a window.postMessage which we then pick up here and delete the toast if the ids match
    Effect::new(move |_| {
        let toast_id = toast.id;
        let closure = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
            if let Some(id) = ev.data().as_string() {
                if let Some(id) = decode_message(id) {
                    if id == toast_id {
                        delete_toast();
                    }
                }
            }
        }) as Box<dyn Fn(web_sys::MessageEvent)>);

        let _ = window().add_event_listener_with_callback("message", closure.as_ref().unchecked_ref());
        closure.forget();
    });

    Effect::new(move |_| {
        mounted_signal.set(true);
    });

    Effect::new(move |_| {
        set_timeout(delete_toast, duration);
    });

    #[derive(Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    let drag_start_time_signal = RwSignal::<Option<f64>>::new(None);
    let pointer_start_signal = RwSignal::<Option<Point>>::new(None);
    let swipe_amount_signal = RwSignal::<i32>::new(0);
    let handle_pointerdown = move |ev: PointerEvent| {
        if !toast.options.dismissible {
            return;
        }
        drag_start_time_signal.set(Some(Date::new_0().get_time()));
        offset_before_remove_signal.set(offset());

        if let Some(target) = ev.target() {
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                let _ = element.set_pointer_capture(ev.pointer_id());
                if element.tag_name() == "BUTTON" {
                    return;
                }
                swiping_signal.set(true);
                pointer_start_signal.set(Some(Point {
                    x: ev.client_x(),
                    y: ev.client_y(),
                }));
            }
        }
    };

    let handle_pointerup = move |_| {
        if swipe_out_signal.get() || !toast.options.dismissible {
            return;
        }
        pointer_start_signal.set(None);
        let time_taken = Date::new_0().get_time() - drag_start_time_signal.with(|t| t.unwrap_or(0.0));
        let velocity = swipe_amount_signal.with(|a| a.abs() as f64) / time_taken;

        if swipe_amount_signal.with(|a| a.abs() >= 20) || velocity > 0.11 {
            offset_before_remove_signal.set(offset());
            delete_toast();
            swipe_out_signal.set(true);
            return;
        };

        swipe_amount_signal.set(0);
        swiping_signal.set(false);
    };

    let handle_pointermove = move |ev: PointerEvent| {
        if !toast.options.dismissible {
            return;
        };
        let _pointer_start = if let Some(pointer_start) = pointer_start_signal.get() {
            pointer_start
        } else {
            return;
        };

        let y_position = ev.client_y() - _pointer_start.y;
        let x_position = ev.client_x() - _pointer_start.x;

        let clamped_y = match position {
            ToasterPosition::BottomRight => max(0, y_position),
        };
        let swipe_start_threshold = if ev.pointer_type() == "touch" { 10 } else { 2 };
        let is_allowed_to_swipe = clamped_y.abs() > swipe_start_threshold;

        if is_allowed_to_swipe {
            swipe_amount_signal.set(y_position);
        } else if x_position.abs() > swipe_start_threshold {
            pointer_start_signal.set(None);
        }
    };

    let class = tw_merge!("sonner-toast-container", "absolute opacity-0 outline-none box-border",);

    view! {
        <li
            aria-atomic="true"
            role="status"
            tab-index=0
            class=class
            data-mounted=move || mounted_signal.get().to_string()
            data-removed=move || removed_signal.get().to_string()
            data-visible=move || is_visible().to_string()
            data-y-position=position.y()
            data-x-position=position.x()
            data-index=index
            data-front=move || is_front().to_string()
            data-swiping=move || swiping_signal.get().to_string()
            data-swipe-out=move || swipe_out_signal.get().to_string()
            data-expanded=move || is_expanded().to_string()
            data-dismissible=toast.options.dismissible.to_string()
            style=("--index", move || format!("{}", index.get()))
            style=("--toasts-before", move || format!("{}", index.get()))
            style=("--z-index", move || format!("{}", num_toasts.get() - index.get()))
            style=("--offset", move || format!("{}px", offset()))
            style=(
                "--initial-height",
                move || {
                    if expand_by_default { "auto".to_string() } else { format!("{}px", initial_height_signal.get()) }
                },
            )
            style=("--swipe-amount", move || format!("{}px", swipe_amount_signal.get()))
            on:pointerdown=handle_pointerdown
            on:pointerup=handle_pointerup
            on:pointermove=handle_pointermove
        >
            <RenderToastView toast_id=toast.id />
        </li>
    }
}

#[component]
fn RenderToastView(toast_id: ToastId) -> impl IntoView {
    let toasts_context = expect_context::<Toasts>();

    // Since AnyView is not Clone, we need to access it without extracting
    // We'll render it by getting a reference within the reactive context
    view! {
        {move || {
            toasts_context
                .views
                .with_value(|views| {
                    views.get(&toast_id).map(|view| { unsafe { std::ptr::read(view as *const AnyView) } })
                })
        }}
    }
}
