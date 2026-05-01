use leptos::prelude::*;

use crate::leptos::leptos_converter::MdComponentProps;

/// Replaces the default `<a>` renderer.
/// External links get a hover tooltip showing the URL.
/// Internal links render as plain anchors.
pub fn link_with_tooltip(props: MdComponentProps) -> impl IntoView {
    let href = props.attributes.get("href").and_then(|v| v.clone()).unwrap_or_default();
    let children = props.children;
    let is_external = href.starts_with("http://") || href.starts_with("https://");
    let tooltip_text = href.clone();
    let target = is_external.then_some("_blank");
    let rel = is_external.then_some("noopener noreferrer");

    view! {
        <span class="relative group/link inline">
            <a href=href target=target rel=rel class="text-primary underline underline-offset-2">
                {children()}
            </a>
            {is_external.then(|| {
                view! {
                    <span class="absolute bottom-full left-0 mb-2 px-2.5 py-1.5 text-xs bg-popover text-popover-foreground border border-border rounded-lg shadow-lg whitespace-nowrap opacity-0 group-hover/link:opacity-100 transition-opacity pointer-events-none z-50 max-w-[320px] truncate font-normal not-italic">
                        {tooltip_text}
                    </span>
                }
            })}
        </span>
    }
}
