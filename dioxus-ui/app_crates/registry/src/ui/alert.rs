use dioxus::prelude::*;
use tw_merge::tw_merge;

#[allow(dead_code)]
#[derive(Default, Clone, PartialEq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

impl AlertVariant {
    fn as_str(&self) -> &'static str {
        match self {
            AlertVariant::Default => "bg-background text-foreground",
            AlertVariant::Destructive => {
                "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
            }
        }
    }
}

#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(
        "relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground [&>svg~*]:pl-7",
        variant.as_str(),
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { role: "alert", class: "{class}", {children} }
    }
}

#[component]
pub fn AlertTitle(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let class = tw_merge!("mb-1 font-medium tracking-tight leading-none", class.as_deref().unwrap_or(""));
    rsx! {
        h4 { class: "{class}", {children} }
    }
}

#[component]
pub fn AlertDescription(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let class = tw_merge!("text-sm [&_p]:leading-relaxed", class.as_deref().unwrap_or(""));
    rsx! {
        p { class: "{class}", {children} }
    }
}
