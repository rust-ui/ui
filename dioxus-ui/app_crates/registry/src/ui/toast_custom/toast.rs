use dioxus::prelude::*;

use crate::ui::toast_custom::_data::{ToastData, ToastLevel, ToastPosition};
use crate::ui::toast_custom::toaster::expect_toaster;

const ANIMATION_DURATION: u64 = 200;

/// A toast element with the supplied alert style.
#[component]
pub fn Toast(toast: ToastData) -> Element {
    let slide_in_animation_name = get_slide_in_animation_name(&toast.position);
    let slide_out_animation_name = get_slide_out_animation_name(&toast.position);

    let mut animation_name_signal = use_signal(|| slide_in_animation_name);

    let (background_color, border_color, text_color) = get_colors(&toast.level);
    let (initial_left, initial_right) = get_initial_positions(&toast.position);
    let cursor = get_cursor(toast.dismissable);
    let expiry_ms = toast.expiry.unwrap_or(0);

    use_effect(move || {
        if let Some(expiry) = toast.expiry {
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(expiry).await;
                if !*toast.clear_signal.peek() {
                    toast.clear_signal.clone().set(true);
                }
            });
        }
    });

    use_effect(move || {
        let toaster = expect_toaster();
        if *toast.clear_signal.read() {
            animation_name_signal.set(slide_out_animation_name);
            let toast_id = toast.id;
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(ANIMATION_DURATION as u32).await;
                toaster.remove(toast_id);
            });
        }
    });

    let handle_click = move |_| {
        if !toast.dismissable {
            return;
        }
        toast.clear_signal.clone().set(true);
    };

    let anim = animation_name_signal();

    rsx! {
        style {
            r#"@keyframes toast_tracker {{
            0% {{ transform: scaleX(1); }}
            100% {{ transform: scaleX(0); }}
            }}"#
        }

        div {
            style: "width: 100%; margin: 12px 0; padding: 16px; background-color: {background_color}; border: 1px solid; border-color: {border_color}; border-radius: 4px; position: relative; cursor: {cursor}; overflow: hidden; box-sizing: border-box; left: {initial_left}; right: {initial_right}; display: flex; transition: transform 150ms ease-out, opacity 150ms ease-out; transition-delay: 250ms, 0s; animation-name: {anim}; animation-duration: {ANIMATION_DURATION}ms; animation-timing-function: linear; animation-fill-mode: forwards;",
            onclick: handle_click,

            span {
                style: "color: {text_color}; font-size: var(--leptoaster-font-size); line-height: var(--leptoaster-line-height); font-family: var(--leptoaster-font-family); font-weight: var(--leptoaster-font-weight); display: inline-block; max-width: 100%; text-overflow: ellipsis; overflow: hidden;",
                "{toast.message}"
            }

            if toast.expiry.is_some() && toast.progress {
                div {
                    style: "height: var(--leptoaster-progress-height); width: 100%; background-color: {text_color}; position: absolute; bottom: 0; left: 0; transform-origin: left; animation-name: toast_tracker; animation-duration: {expiry_ms}ms; animation-timing-function: linear; animation-fill-mode: forwards;",
                }
            }
        }
    }
}

fn get_slide_in_animation_name(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft | ToastPosition::BottomLeft => "leptoaster-slide-in-left",
        ToastPosition::TopRight | ToastPosition::BottomRight => "leptoaster-slide-in-right",
    }
}

fn get_slide_out_animation_name(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft | ToastPosition::BottomLeft => "leptoaster-slide-out-left",
        ToastPosition::TopRight | ToastPosition::BottomRight => "leptoaster-slide-out-right",
    }
}

fn get_colors(level: &ToastLevel) -> (&'static str, &'static str, &'static str) {
    match level {
        ToastLevel::Info => (
            "var(--leptoaster-info-background-color)",
            "var(--leptoaster-info-border-color)",
            "var(--leptoaster-info-text-color)",
        ),
        ToastLevel::Success => (
            "var(--leptoaster-success-background-color)",
            "var(--leptoaster-success-border-color)",
            "var(--leptoaster-success-text-color)",
        ),
        ToastLevel::Warn => (
            "var(--leptoaster-warn-background-color)",
            "var(--leptoaster-warn-border-color)",
            "var(--leptoaster-warn-text-color)",
        ),
        ToastLevel::Error => (
            "var(--leptoaster-error-background-color)",
            "var(--leptoaster-error-border-color)",
            "var(--leptoaster-error-text-color)",
        ),
    }
}

fn get_initial_positions(position: &ToastPosition) -> (&'static str, &'static str) {
    match position {
        ToastPosition::TopLeft | ToastPosition::BottomLeft => {
            ("calc((var(--leptoaster-width) + 12px * 2) * -1)", "auto")
        }
        ToastPosition::TopRight | ToastPosition::BottomRight => {
            ("auto", "calc((var(--leptoaster-width) + 12px * 2) * -1)")
        }
    }
}

fn get_cursor(dismissable: bool) -> &'static str {
    match dismissable {
        true => "pointer",
        false => "default",
    }
}
