use leptos::prelude::*;

use crate::leptos::leptos_converter::MdComponentProps;

/// Usage in markdown:
/// <UrlPreview href="https://example.com" />
/// <UrlPreview href="https://example.com" label="Watch the video" />
/// YouTube URLs are automatically embedded as an iframe.
pub fn url_preview_md(props: MdComponentProps) -> impl IntoView {
    let href = props.attributes.get("href").and_then(|v| v.clone()).unwrap_or_default();
    let label = props.attributes.get("label").and_then(|v| v.clone()).unwrap_or_else(|| href.clone());

    if let Some(video_id) = extract_youtube_id(&href) {
        let embed_src = format!("https://www.youtube.com/embed/{}", video_id);
        view! {
            <div class="overflow-hidden relative my-6 mx-auto w-full max-w-2xl rounded-xl shadow-lg aspect-video">
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
            <span class="inline-block relative my-1 group">
                <a
                    href=href
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex gap-2 items-center py-2 px-3 text-sm no-underline rounded-xl border transition-colors text-muted-foreground border-border bg-card hover:bg-muted/60"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="opacity-60 size-4 shrink-0"
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
                        class="opacity-40 size-3 shrink-0"
                    >
                        <path d="M7 7h10v10M7 17 17 7" />
                    </svg>
                </a>
                <span class="absolute left-1/2 bottom-full z-50 py-1 px-2.5 mb-2 text-xs whitespace-nowrap rounded-lg border shadow-md opacity-0 transition-opacity -translate-x-1/2 pointer-events-none group-hover:opacity-100 text-popover-foreground bg-popover border-border">
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
