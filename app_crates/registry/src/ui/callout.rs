use leptos::prelude::*;
use tw_merge::tw_merge;

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Warning,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Callout(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] variant: CalloutVariant,
) -> impl IntoView {
    let variant_class = match variant {
        CalloutVariant::Default => "border-border bg-surface text-surface-foreground",
        CalloutVariant::Info => "border-info bg-info-light text-foreground dark:bg-info-dark/20 dark:border-info/50",
        CalloutVariant::Warning => {
            "border-warning bg-warning-light text-foreground dark:bg-warning-dark/20 dark:border-warning/50"
        }
    };

    let merged_class = tw_merge!(
        "relative w-full rounded-xl border px-4 py-3 text-sm md:-mx-1 [&_code]:bg-black/5 [&_code]:rounded [&_code]:px-1 [&_code]:py-0.5 dark:[&_code]:bg-white/10",
        variant_class,
        class
    );

    view! {
        <div class=merged_class data-name="Callout">
            {title.map(|t| view! { <p class="mb-1 font-medium leading-none">{t}</p> })}
            <p class="text-sm leading-relaxed text-muted-foreground">{children()}</p>
        </div>
    }
}
