use dioxus::prelude::*;
use serde::Serialize;

use super::site_config::SiteConfig;

#[derive(Serialize)]
struct OrganizationSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@graph")]
    graph: Vec<GraphNode>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GraphNode {
    Organization(OrganizationNode),
    WebSite(WebSiteNode),
}

#[derive(Serialize)]
struct OrganizationNode {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
    name: String,
    url: String,
    logo: String,
}

#[derive(Serialize)]
struct WebSiteNode {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
    url: String,
    name: String,
    publisher: PublisherRef,
}

#[derive(Serialize)]
struct PublisherRef {
    #[serde(rename = "@id")]
    id: String,
}

/// JSON-LD structured data component for Organization + WebSite schema.
///
/// Emits both nodes in a single `@graph` so search engines can attribute the site
/// to an entity and offer a sitelinks search box. Meant to be rendered once, on
/// the homepage only (duplicating `@id`-bearing nodes across pages is redundant,
/// not harmful, but the homepage is the canonical place for it).
///
/// # Example
///
/// ```rust,ignore
/// use app_config::JsonLdOrganization;
///
/// rsx! {
///     JsonLdOrganization {}
/// }
/// ```
#[component]
pub fn JsonLdOrganization() -> Element {
    let base = SiteConfig::BASE_URL;
    let org_id = format!("{base}/#organization");
    let website_id = format!("{base}/#website");

    let schema = OrganizationSchema {
        context: "https://schema.org".to_string(),
        graph: vec![
            GraphNode::Organization(OrganizationNode {
                type_: "Organization".to_string(),
                id: org_id.clone(),
                name: SiteConfig::TITLE.to_string(),
                url: base.to_string(),
                logo: format!("{base}/og-image.webp"),
            }),
            GraphNode::WebSite(WebSiteNode {
                type_: "WebSite".to_string(),
                id: website_id,
                url: base.to_string(),
                name: SiteConfig::TITLE.to_string(),
                publisher: PublisherRef { id: org_id },
            }),
        ],
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    rsx! {
        document::Script { r#type: "application/ld+json", "{json_content}" }
    }
}
