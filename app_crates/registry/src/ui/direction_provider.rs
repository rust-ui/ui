use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }
}

#[component]
pub fn DirectionProvider(
    #[props(default)] dir: Direction,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(class.as_deref().unwrap_or(""));
    rsx! {
        div {
            "data-slot": "direction-provider",
            dir: "{dir.as_str()}",
            class: "{class}",
            {children}
        }
    }
}
