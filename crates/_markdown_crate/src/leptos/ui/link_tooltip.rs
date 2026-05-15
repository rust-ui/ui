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
        <span class="inline relative group/link">
            <a href=href target=target rel=rel class="underline text-primary underline-offset-2">
                {children()}
            </a>
            {is_external
                .then(|| {
                    view! {
                        <span class="absolute left-0 bottom-full z-50 py-1.5 px-2.5 mb-2 text-xs not-italic font-normal whitespace-nowrap rounded-lg border shadow-lg opacity-0 transition-opacity pointer-events-none bg-popover text-popover-foreground border-border max-w-[320px] truncate group-hover/link:opacity-100">
                            {tooltip_text}
                        </span>
                    }
                })}
        </span>
    }
}
