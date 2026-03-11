use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

use super::site_config::SiteConfig;

#[component]
pub fn SeoMeta(
    title: String,
    description: String,
    #[prop(optional)] canonical_url: Option<String>,
    #[prop(optional)] image_url: Option<String>,
    #[prop(optional)] og_title: Option<String>,
    /// Page type for Open Graph (optional, defaults to "website")
    #[prop(optional)]
    og_type: Option<String>,
) -> impl IntoView {
    let canonical = canonical_url.unwrap_or_else(|| SiteConfig::BASE_URL.to_string());
    let og_image = image_url.unwrap_or_else(|| format!("{}/og-image.webp", SiteConfig::BASE_URL));
    let og_title_text = og_title.unwrap_or_else(|| title.clone());
    let og_type_value = og_type.unwrap_or_else(|| "website".to_string());

    view! {
        <Title text=title />
        <Meta name="description" content=description.clone() />
        <Link rel="canonical" href=canonical.clone() />

        // OpenGraph tags for social sharing
        <Meta property="og:title" content=og_title_text.clone() />
        <Meta property="og:description" content=description.clone() />
        <Meta property="og:url" content=canonical />
        <Meta property="og:type" content=og_type_value />
        <Meta property="og:image" content=og_image.clone() />
        <Meta property="og:image:width" content="1200" />
        <Meta property="og:image:height" content="630" />

        // Twitter Card tags
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content=og_title_text />
        <Meta name="twitter:description" content=description />
        <Meta name="twitter:image" content=og_image />
    }
}
