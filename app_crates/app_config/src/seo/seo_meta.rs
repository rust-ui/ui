use dioxus::prelude::*;

use super::site_config::SiteConfig;

#[component]
pub fn SeoMeta(
    title: String,
    description: String,
    #[props(default)] canonical_url: Option<String>,
    #[props(default)] image_url: Option<String>,
    #[props(default)] og_title: Option<String>,
    /// Page type for Open Graph (optional, defaults to "website")
    #[props(default)]
    og_type: Option<String>,
) -> Element {
    let canonical = canonical_url.unwrap_or_else(|| SiteConfig::BASE_URL.to_string());
    let og_image = image_url.unwrap_or_else(|| format!("{}/og-image.webp", SiteConfig::BASE_URL));
    let og_title_text = og_title.unwrap_or_else(|| title.clone());
    let og_type_value = og_type.unwrap_or_else(|| "website".to_string());

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{description}" }
        document::Link { rel: "canonical", href: "{canonical}" }

        // OpenGraph tags for social sharing
        document::Meta { property: "og:title", content: "{og_title_text}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:url", content: "{canonical}" }
        document::Meta { property: "og:type", content: "{og_type_value}" }
        document::Meta { property: "og:image", content: "{og_image}" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }

        // Twitter Card tags
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{og_title_text}" }
        document::Meta { name: "twitter:description", content: "{description}" }
        document::Meta { name: "twitter:image", content: "{og_image}" }
    }
}
