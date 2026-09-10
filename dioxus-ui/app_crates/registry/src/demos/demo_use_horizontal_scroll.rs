use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use crate::hooks::use_horizontal_scroll::{HorizontalScrollState, use_horizontal_scroll};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn DemoUseHorizontalScroll() -> Element {
    let mut scroll_container_element = use_signal(|| None::<web_sys::Element>);
    let scroll_ctx = use_horizontal_scroll(scroll_container_element.into(), None, None);
    let scroll_state = (scroll_ctx.scroll_state)();

    rsx! {
        div { class: "w-full",
            div { class: "flex justify-between items-center mb-4",
                h3 { class: "text-lg font-semibold", "Horizontal Scroll Demo" }
                div { class: "flex gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::IconSm,
                        disabled: scroll_state == HorizontalScrollState::Start,
                        onclick: move |_| scroll_ctx.scroll_by.call(-1),
                        ChevronLeft { class: "size-4" }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::IconSm,
                        disabled: scroll_state == HorizontalScrollState::End,
                        onclick: move |_| scroll_ctx.scroll_by.call(1),
                        ChevronRight { class: "size-4" }
                    }
                }
            }

            div {
                class: "flex overflow-x-scroll gap-4 snap-x snap-mandatory scroll-smooth [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
                onmounted: move |event| {
                    if let Some(element) = event.data().downcast::<web_sys::Element>().cloned() {
                        scroll_container_element.set(Some(element));
                    }
                },
                onscroll: move |event| scroll_ctx.on_scroll.call(event),
                for i in 0..6 {
                    div { class: "flex justify-center items-center w-64 h-40 bg-gray-300 rounded-lg shrink-0 snap-start",
                        span { class: "text-2xl font-bold text-gray-600", "Card {i + 1}" }
                    }
                }
            }

            div { class: "mt-4 text-sm text-muted-foreground",
                "Scroll state: "
                span { class: "font-semibold", "{scroll_state}" }
            }
        }
    }
}
