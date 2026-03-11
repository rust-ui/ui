use leptos::prelude::*;

use crate::leptos::leptos_converter::MdComponentProps;

pub fn callout_md(props: MdComponentProps) -> impl IntoView {
    let title = props.attributes.get("title").and_then(|v| v.clone());
    let variant = props.attributes.get("variant").and_then(|v| v.as_deref()).unwrap_or("default");

    let variant_class = match variant {
        "info" | "Info" => "border-info bg-info-light text-foreground dark:bg-info-dark/20 dark:border-info/50",
        "warning" | "Warning" => {
            "border-warning bg-warning-light text-foreground dark:bg-warning-dark/20 dark:border-warning/50"
        }
        _ => "border-border bg-surface text-surface-foreground",
    };

    let class = format!(
        "relative w-full rounded-xl border px-4 py-3 text-sm md:-mx-1 [&_code]:bg-black/5 [&_code]:rounded [&_code]:px-1 [&_code]:py-0.5 dark:[&_code]:bg-white/10 {}",
        variant_class
    );

    let children = props.children;

    view! {
        <div class=class data-slot="callout">
            {title.map(|t| view! { <p class="mb-1 font-medium leading-none">{t}</p> })}
            <p class="text-sm leading-relaxed text-muted-foreground">{children()}</p>
        </div>
    }
}
