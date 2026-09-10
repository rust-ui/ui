use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum StatusVariant {
    #[default]
    Default,
    Active,
    Inactive,
    Normal,
}

impl StatusVariant {
    fn color(&self) -> &'static str {
        match self {
            StatusVariant::Default => "bg-neutral-300",
            StatusVariant::Active => "bg-green-300",
            StatusVariant::Inactive => "bg-orange-300",
            StatusVariant::Normal => "bg-sky-300",
        }
    }
}

#[component]
fn StatusIndicator(
    #[props(default = StatusVariant::default())] variant: StatusVariant,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let merged = tw_merge!(
        "absolute top-0 right-0 -mt-1 -mr-1 rounded-full size-4",
        variant.color(),
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}" } }
}

#[component]
pub fn Status(
    #[props(into, optional)] class: Option<String>,
    #[props(default = StatusVariant::default())] variant: StatusVariant,
    children: Element,
) -> Element {
    let merged = tw_merge!("relative", class.as_deref().unwrap_or(""));
    rsx! {
        div { class: "{merged}",
            {children}
            StatusIndicator { variant, class: "animate-ping" }
            StatusIndicator { variant }
        }
    }
}
