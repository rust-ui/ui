use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, PointerEvent};

use crate::components::sonner_leptos::toast_container::ToastContainer;
use crate::components::sonner_leptos::toast_id::ToastId;
use crate::components::sonner_leptos::types::{HeightT, ToasterPosition, Toasts};

pub const DEFAULT_TOAST_DURATION: u64 = 5000;

/// Toaster context provider.
/// Wrap your app in the Toaster to use the Toasts context in children
#[component]
pub fn Toaster(
    #[prop(default = ToasterPosition::BottomRight)] position: ToasterPosition,
    #[prop(default = false)] expand: bool,
    #[prop(default = Duration::from_millis(DEFAULT_TOAST_DURATION))] duration: Duration,
    #[prop(default = 14)] gap: usize,
    /// The maximum amount of toasts that should be visible at any point
    #[prop(default = 3)]
    visible_toasts: usize,
    children: Children,
) -> impl IntoView {
    let toaster_styles = include_str!("./toaster.css");

    let toasts = StoredValue::new_local(Vec::new());
    let views = StoredValue::new_local(std::collections::HashMap::new());
    let trigger_signal = RwSignal::new(0usize);
    let (expanded, set_expanded) = signal(false);
    let interacting_signal = RwSignal::new(false);
    let heights = RwSignal::<Vec<HeightT>>::new(Vec::new());

    provide_context(Toasts { toasts, views, trigger: trigger_signal });
    Effect::new(move |_| {
        trigger_signal.track();
        // Ensure expanded is always false when no toasts are present / only one left
        if toasts.with_value(|t| t.len()) <= 1 {
            set_expanded.set(false);
        }
    });

    let remove_toast = Callback::new(move |toast_id: ToastId| {
        toasts.update_value(|toasts_vec| {
            if let Some(index) = toasts_vec.iter().position(|t| t.id == toast_id) {
                toasts_vec.remove(index);
            }
        });
        trigger_signal.update(|t| *t += 1);
    });

    let on_pointerdown = move |e: PointerEvent| {
        let mut is_dismissible = true;
        if let Some(target) = e.target() {
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                if let Some(dismissible) = element.dataset().get("dismissible") {
                    is_dismissible = dismissible != "false";
                }
            };
        };
        if is_dismissible {
            interacting_signal.set(true);
        }
    };

    view! {
        <style>{toaster_styles}</style>

        {children()}

        <Show when=move || {
            trigger.track();
            !toasts.with_value(|t| t.is_empty())
        }>
            <section aria-label="Notifications" tab-index=-1>
                <ol
                    class="border-box fixed p-0 m-0 list-none outline-none leptos-toaster"
                    tab-index=-1
                    data-y-position=position.y()
                    data-x-position=position.x()
                    style=("--gap", format!("{}px", gap))
                    style=("--width", "356px")
                    style=("--offset", "32px")
                    style=(
                        "--front-toast-height",
                        move || {
                            format!(
                                "{}px",
                                heights.with(|heights| { heights.first().map(|h| h.height).unwrap_or(0.0) }),
                            )
                        },
                    )
                    on:mouseenter=move |_| set_expanded.set(true)
                    on:mousemove=move |_| set_expanded.set(true)
                    on:mouseleave=move |_| {
                        if !interacting_signal.get() {
                            set_expanded.set(false)
                        }
                    }
                    on:pointerdown=on_pointerdown
                    on:pointerup=move |_| interacting_signal.set(true)
                >
                    <For
                        each=move || {
                            trigger_signal.track();
                            toasts.with_value(|t| t.iter().map(|toast| toast.id).collect::<Vec<_>>())
                        }
                        key=move |toast_id| *toast_id
                        children=move |toast_id| {
                            let toast = toasts
                                .with_value(|toasts_vec| { toasts_vec.iter().find(|t| t.id == toast_id).cloned() });
                            let Some(toast) = toast else {
                                return view! {}.into_any();
                            };
                            let index = Memo::new(move |_| {
                                trigger_signal.track();
                                toasts
                                    .with_value(|toasts_list| {
                                        toasts_list.iter().position(|t| t.id == toast_id).unwrap_or_default()
                                    })
                            });
                            // Look up the toast by ID from StoredValue

                            view! {
                                // Doing this since we
                                // 1. don't want the view to rerender, and in turn, the ToastContainer to rerender when a new toast is added, because that makes the internal logic more complex. For instance the timeout to delete the toast after the duration would have to keep track of the timeout handle between rerenders. And
                                // 2. enumerating the toasts vec will not give a reactive index, so we need to get it like this
                                <ToastContainer
                                    index=Signal::derive(move || index.get())
                                    toast
                                    visible_toasts
                                    position
                                    duration_from_toaster=duration
                                    remove_toast=remove_toast
                                    expanded
                                    expand_by_default=expand
                                    num_toasts=Signal::derive(move || {
                                        trigger_signal.track();
                                        toasts.with_value(|t| t.len())
                                    })
                                    heights
                                    gap
                                />
                            }
                                .into_any()
                        }
                    />

                </ol>
            </section>
        </Show>
    }
}
