---
title: "Toast"
name: "toast"
cargo_dependencies: []
registry_dependencies: ["toast_custom"]
type: "components:ui/toast_custom"
path: "ui/toast.rs"
description: "Rust/UI component that displays toast notifications."
tags: []
---

# Toast

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add toast
```

## Component Code

```rust
use std::time::Duration;

use leptos::prelude::*;

use crate::components::ui::toast_custom::_data::{ToastData, ToastLevel, ToastPosition};
use crate::components::ui::toast_custom::toaster::expect_toaster;

const ANIMATION_DURATION: u64 = 200;

/// A toast element with the supplied alert style.
#[component]
pub fn Toast(toast: ToastData) -> impl IntoView {
    let slide_in_animation_name = get_slide_in_animation_name(&toast.position);
    let slide_out_animation_name = get_slide_out_animation_name(&toast.position);

    let animation_name_signal = RwSignal::new(slide_in_animation_name);

    let (background_color, border_color, text_color) = get_colors(&toast.level);
    let (initial_left, initial_right) = get_initial_positions(&toast.position);

    Effect::new(move |_| {
        if let Some(expiry) = toast.expiry {
            set_timeout(
                move || {
                    if !toast.clear_signal.get_untracked() {
                        // Use try_set to avoid panic if component is unmounted
                        let _ = toast.clear_signal.try_set(true);
                    }
                },
                Duration::from_millis(expiry as u64),
            );
        }
    });

    Effect::new(move |_| {
        let toaster = expect_toaster();

        if toast.clear_signal.get() {
            // Use try_set to avoid panic if component is unmounted
            let _ = animation_name_signal.try_set(slide_out_animation_name);

            set_timeout(move || toaster.remove(toast.id), Duration::from_millis(ANIMATION_DURATION));
        }
    });

    let handle_click = move |_| {
        if !toast.dismissable {
            return;
        }

        toast.clear_signal.set(true);
    };

    view! {
        <style>
            {r#"
            @keyframes toast_tracker {
            0% { transform: scaleX(1); }
            100% { transform: scaleX(0); }
            }
            "#}
        </style>

        <div
            style:width="100%"
            style:margin="12px 0"
            style:padding="16px"
            style:background-color=background_color
            style:border="1px solid"
            style:border-color=border_color
            style:border-radius="4px"
            style:position="relative"
            style:cursor=get_cursor(toast.dismissable)
            style:overflow="hidden"
            style:box-sizing="border-box"
            style:left=initial_left
            style:right=initial_right
            style:display="flex"
            style:transition="transform 150ms ease-out, opacity 150ms ease-out"
            style:transition-delay="250ms, 0s"
            style:animation-name=animation_name_signal
            style:animation-duration=format!("{}ms", ANIMATION_DURATION)
            style:animation-timing-function="linear"
            style:animation-fill-mode="forwards"
            on:click=handle_click
        >
            <span
                style:color=text_color
                style:font-size="var(--leptoaster-font-size)"
                style:line-height="var(--leptoaster-line-height)"
                style:font-family="var(--leptoaster-font-family)"
                style:font-weight="var(--leptoaster-font-weight)"
                style:display="inline-block"
                style:max-width="100%"
                style:text-overflow="ellipsis"
                style:overflow="hidden"
            >
                {toast.message}
            </span>

            <Show when=move || { toast.expiry.is_some() && toast.progress }>
                <div
                    style:height="var(--leptoaster-progress-height)"
                    style:width="100%"
                    style:background-color=text_color
                    style:position="absolute"
                    style:bottom="0"
                    style:left="0"
                    style:transform-origin="left"
                    style:animation-name="toast_tracker"
                    style:animation-duration=format!("{}ms", toast.expiry.unwrap_or(0))
                    style:animation-timing-function="linear"
                    style:animation-fill-mode="forwards"
                />
            </Show>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

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
```
