use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MaskSide {
    #[default]
    Default,
    Left,
    Right,
    Top,
    Bottom,
}

impl MaskSide {
    pub fn class(self) -> &'static str {
        match self {
            MaskSide::Default => "",
            MaskSide::Left => "left-0 w-1/3 bg-gradient-to-r",
            MaskSide::Right => "right-0 w-1/3 bg-gradient-to-l",
            MaskSide::Top => "top-0 w-full bg-gradient-to-b",
            MaskSide::Bottom => "bottom-0 w-full bg-gradient-to-t",
        }
    }
}

#[component]
pub fn MaskWrapper(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "flex justify-center items-center",
        "relative w-full h-full",
        "rounded-lg border",
        "overflow-hidden",
        "min-h-[300px]",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "MaskWrapper", class: "{c}", {children} } }
}

#[component]
pub fn Mask(
    #[props(default = MaskSide::Default)] side: MaskSide,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let c = tw_merge!(
        "absolute inset-y-0 pointer-events-none from-white dark:from-background",
        side.class(),
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "Mask", class: "{c}" } }
}
