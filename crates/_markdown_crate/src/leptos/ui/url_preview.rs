use leptos::prelude::*;

use crate::leptos::leptos_converter::MdComponentProps;

/// Usage in markdown:
/// <UrlPreview href="https://example.com" />
/// <UrlPreview href="https://example.com" label="Watch the video" />
/// YouTube URLs are automatically embedded as an iframe.
pub fn url_preview_md(props: MdComponentProps) -> impl IntoView {
    let href = props.attributes.get("href").and_then(|v| v.clone()).unwrap_or_default();
    let label = props
        .attributes
        .get("label")
        .and_then(|v| v.clone())
        .unwrap_or_else(|| href.clone());

    if let Some(video_id) = extract_youtube_id(&href) {
        let embed_src = format!("https://www.youtube.com/embed/{}", video_id);
        view! {
            <div class="relative my-6 w-full max-w-2xl mx-auto rounded-xl overflow-hidden shadow-lg aspect-video">
                <iframe
                    src=embed_src
                    title=label
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen
                    class="absolute inset-0 w-full h-full"
                ></iframe>
            </div>
        }
        .into_any()
    } else {
        let tooltip = href.clone();
        view! {
            <span class="relative group inline-block my-1">
                <a
                    href=href
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-2 px-3 py-2 text-sm text-muted-foreground no-underline rounded-xl border border-border bg-card hover:bg-muted/60 transition-colors"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="size-4 shrink-0 opacity-60"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
                        <path d="M2 12h20" />
                    </svg>
                    <span class="truncate max-w-[360px]">{label}</span>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="size-3 shrink-0 opacity-40"
                    >
                        <path d="M7 7h10v10M7 17 17 7" />
                    </svg>
                </a>
                <span class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2.5 py-1 text-xs text-popover-foreground bg-popover border border-border rounded-lg whitespace-nowrap shadow-md opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
                    {tooltip}
                </span>
            </span>
        }
        .into_any()
    }
}

fn extract_youtube_id(url: &str) -> Option<String> {
    // https://www.youtube.com/watch?v=t5YeSDBfT4g
    if let Some(pos) = url.find("v=") {
        let id = &url[pos + 2..];
        let end = id.find(['&', '#']).unwrap_or(id.len());
        return Some(id[..end].to_string());
    }
    // https://youtu.be/t5YeSDBfT4g
    if let Some(after) = url.split("youtu.be/").nth(1) {
        let end = after.find(['?', '#']).unwrap_or(after.len());
        return Some(after[..end].to_string());
    }
    None
}
