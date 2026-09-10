use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Warning,
}

#[component]
pub fn Callout(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] title: Option<String>,
    #[props(default = CalloutVariant::default())] variant: CalloutVariant,
    children: Element,
) -> Element {
    let variant_class = match variant {
        CalloutVariant::Default => "border-border bg-surface text-surface-foreground",
        CalloutVariant::Info => "border-info bg-info-light text-foreground dark:bg-info-dark/20 dark:border-info/50",
        CalloutVariant::Warning => {
            "border-warning bg-warning-light text-foreground dark:bg-warning-dark/20 dark:border-warning/50"
        }
    };

    let merged = tw_merge!(
        "relative w-full rounded-xl border px-4 py-3 text-sm md:-mx-1 [&_code]:bg-black/5 [&_code]:rounded [&_code]:px-1 [&_code]:py-0.5 dark:[&_code]:bg-white/10",
        variant_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { "data-name": "Callout", class: "{merged}",
            if let Some(t) = title {
                p { class: "mb-1 font-medium leading-none", "{t}" }
            }
            p { class: "text-sm leading-relaxed text-muted-foreground", {children} }
        }
    }
}
