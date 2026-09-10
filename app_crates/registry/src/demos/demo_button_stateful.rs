use dioxus::prelude::*;

use crate::ui::button::Button;

#[derive(Clone, PartialEq)]
enum ButtonState {
    Idle,
    Working,
    Done,
}

impl ButtonState {
    fn label(&self) -> &'static str {
        match self {
            ButtonState::Idle => "Do some hard work",
            ButtonState::Working => "⏳ Working...",
            ButtonState::Done => "Done! ✅",
        }
    }

    fn next(&self) -> Self {
        match self {
            ButtonState::Idle => ButtonState::Working,
            ButtonState::Working => ButtonState::Done,
            ButtonState::Done => ButtonState::Idle,
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
