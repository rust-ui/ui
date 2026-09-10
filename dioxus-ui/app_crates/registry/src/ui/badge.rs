use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Accent,
    Muted,
    Destructive,
    Outline,
    Success,
    Warning,
    Info,
}

impl BadgeVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "border-transparent shadow bg-primary text-primary-foreground hover:bg-primary/80",
            BadgeVariant::Secondary => {
                "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            BadgeVariant::Accent => "border-transparent bg-accent text-accent-foreground hover:bg-accent/80",
            BadgeVariant::Muted => "border-transparent bg-muted text-muted-foreground hover:bg-muted/80",
            BadgeVariant::Destructive => {
                "border-transparent shadow bg-destructive text-destructive-foreground hover:bg-destructive/80"
            }
            BadgeVariant::Outline => "text-foreground",
            BadgeVariant::Success => "border-transparent bg-success-light text-success-dark hover:bg-success-light/80",
            BadgeVariant::Warning => "border-transparent bg-warning-light text-warning-dark hover:bg-warning-light/80",
            BadgeVariant::Info => "border-transparent bg-info-light text-info-dark hover:bg-info-light/80",
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum BadgeSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl BadgeSize {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeSize::Default => "px-2.5 py-0.5 text-xs",
            BadgeSize::Sm => "px-1.5 py-0.5 text-[10px]",
            BadgeSize::Lg => "px-3 py-1 text-sm",
        }
    }
}

#[component]
pub fn Badge(
    #[props(into, optional)] class: Option<String>,
    #[props(default = BadgeVariant::default())] variant: BadgeVariant,
    #[props(default = BadgeSize::default())] size: BadgeSize,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center font-semibold rounded-md border transition-colors focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2 w-fit",
        variant.as_str(),
        size.as_str(),
        class.as_deref().unwrap_or("")
    );
    rsx! { span { class: "{merged}", {children} } }
}
