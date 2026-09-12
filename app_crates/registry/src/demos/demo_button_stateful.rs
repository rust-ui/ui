use dioxus::prelude::*;

use crate::ui::button::Button;

#[derive(Clone, PartialEq)]
enum ButtonState {
    Idle,
    Working,
    Done,
}

impl ButtonState {
    const fn label(&self) -> &'static str {
        match self {
            Self::Idle => "Do some hard work",
            Self::Working => "⏳ Working...",
            Self::Done => "Done! ✅",
        }
    }

    const fn next(&self) -> Self {
        match self {
            Self::Idle => Self::Working,
            Self::Working => Self::Done,
            Self::Done => Self::Idle,
        }
    }
}

#[component]
pub fn DemoButtonStateful() -> Element {
    let mut state = use_signal(|| ButtonState::Idle);

    rsx! {
        Button {
            onclick: move |_| {
                let next = state.read().next();
                state.set(next);
            },
            "{state.read().label()}"
        }
    }
}
