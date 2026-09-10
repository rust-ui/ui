use dioxus::prelude::*;

use crate::ui::toast_custom::_context::ToasterContext;
use crate::ui::toast_custom::_data::{ToastData, ToastPosition};
use crate::ui::toast_custom::_template_styles::TEMPLATE_STYLES;
use crate::ui::toast_custom::toast::Toast;

const CONTAINER_POSITIONS: &[ToastPosition] =
    &[ToastPosition::TopLeft, ToastPosition::TopRight, ToastPosition::BottomRight, ToastPosition::BottomLeft];

#[component]
pub fn Toaster(#[props(default = false)] stacked: bool) -> Element {
    let toaster = expect_toaster();

    rsx! {
        style { dangerous_inner_html: TEMPLATE_STYLES }

        for position in CONTAINER_POSITIONS {
            if !is_container_empty(position) {
                div {
                    class: get_container_class(stacked, position),
                    style: "margin: {get_container_margin(position)}; position: fixed; inset: {get_container_inset(position)}; z-index: var(--leptoaster-z-index);",
                    for toast in get_toasts_for_position(&toaster, position) {
                        Toast { key: "{toast.id}", toast }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     FUNCTIONS                              */
/* ========================================================== */

pub fn provide_toaster() {
    let queue_signal = use_signal(Vec::<ToastData>::new);
    use_context_provider(|| ToasterContext::new(queue_signal));
}

#[must_use]
pub fn expect_toaster() -> ToasterContext {
    consume_context::<ToasterContext>()
}

/* ========================================================== */
/*                     HELPERS                                */
/* ========================================================== */

fn is_container_empty(position: &ToastPosition) -> bool {
    let toaster = expect_toaster();
    !toaster.queue_signal.read().iter().any(|toast| toast.position.eq(position))
}

fn get_toasts_for_position(toaster: &ToasterContext, position: &ToastPosition) -> Vec<ToastData> {
    let toasts = toaster.queue_signal.read().clone();
    match position {
        ToastPosition::BottomLeft | ToastPosition::BottomRight => {
            toasts.into_iter().filter(|toast| toast.position.eq(position)).collect()
        }
        ToastPosition::TopLeft | ToastPosition::TopRight => {
            toasts.into_iter().filter(|toast| toast.position.eq(position)).rev().collect()
        }
    }
}

#[allow(dead_code)]
fn get_container_id(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft => "top_left",
        ToastPosition::TopRight => "top_right",
        ToastPosition::BottomRight => "bottom_right",
        ToastPosition::BottomLeft => "bottom_left",
    }
}

fn get_container_inset(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft => "12px auto auto 12px",
        ToastPosition::TopRight => "12px 12px auto auto",
        ToastPosition::BottomRight => "auto 12px 12px auto",
        ToastPosition::BottomLeft => "auto auto 12px 12px",
    }
}

fn get_container_margin(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft | ToastPosition::BottomLeft => "0 0 0 12px",
        ToastPosition::TopRight | ToastPosition::BottomRight => "0 12px 0 0",
    }
}

fn get_container_class(stacked: bool, position: &ToastPosition) -> &'static str {
    if !stacked {
        return "";
    }

    match position {
        ToastPosition::BottomLeft | ToastPosition::BottomRight => "leptoaster-stack-container-bottom",
        ToastPosition::TopLeft | ToastPosition::TopRight => "leptoaster-stack-container-top",
    }
}
