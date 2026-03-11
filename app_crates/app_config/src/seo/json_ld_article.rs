use leptos::prelude::*;
use leptos_meta::Script;
use serde::Serialize;

use super::site_config::SiteConfig;

#[derive(Serialize)]
struct TechArticleSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    headline: String,
    description: String,
    author: Author,
    #[serde(rename = "datePublished")]
    date_published: String,
    #[serde(rename = "dateModified")]
    date_modified: String,
    url: String,
    #[serde(rename = "mainEntityOfPage")]
    main_entity_of_page: MainEntity,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    keywords: Vec<String>,
    #[serde(rename = "articleSection")]
    article_section: String,
}

#[derive(Serialize)]
struct Author {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
}

#[derive(Serialize)]
struct MainEntity {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
}

/// JSON-LD structured data component for TechArticle schema.
///
/// This component generates Schema.org TechArticle structured data for documentation pages,
/// which helps search engines understand the content and display rich snippets in search results.
///
/// # Example
///
/// ```rust,ignore
/// use app_config::JsonLdArticle;
///
/// view! {
///     <JsonLdArticle
///         title="Leptos Button Component"
///         description="Beautiful button component for Leptos applications"
///         url="https://www.rust-ui.com/docs/components/button"
///         keywords=vec!["leptos".to_string(), "rust".to_string(), "button".to_string()]
///         article_section="Components"
///     />
/// }
/// ```
#[component]
pub fn JsonLdArticle(
    /// Article title/headline
    title: String,
    /// Article description
    description: String,
    /// Canonical URL of the article
    url: String,
    /// Optional publication date (ISO 8601 format: "2024-01-01")
    #[prop(optional)]
    date_published: Option<String>,
    /// Optional modification date (ISO 8601 format: "2025-11-08")
    #[prop(optional)]
    date_modified: Option<String>,
    /// Article keywords/tags
    #[prop(default = Vec::new())]
    keywords: Vec<String>,
    /// Article section (e.g., "Components", "Hooks", "Documentation")
    #[prop(default = "Documentation".to_string())]
    article_section: String,
) -> impl IntoView {
    let schema = TechArticleSchema {
        context: "https://schema.org".to_string(),
        type_: "TechArticle".to_string(),
        headline: title,
        description,
        author: Author { type_: "Organization".to_string(), id: format!("{}/#organization", SiteConfig::BASE_URL) },
        date_published: date_published.unwrap_or_else(|| "2024-01-01".to_string()),
        date_modified: date_modified.unwrap_or_else(|| "2025-11-08".to_string()),
        url: url.clone(),
        main_entity_of_page: MainEntity { type_: "WebPage".to_string(), id: url },
        keywords,
        article_section,
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    view! { <Script type_="application/ld+json">{json_content}</Script> }
}

#[cfg(test)]
mod unit_tests {
    use serde_json::Value;

    use super::*;

    fn create_test_schema() -> TechArticleSchema {
        TechArticleSchema {
            context: "https://schema.org".to_string(),
            type_: "TechArticle".to_string(),
            headline: "Test Article".to_string(),
            description: "Test description".to_string(),
            author: Author { type_: "Organization".to_string(), id: format!("{}/#organization", SiteConfig::BASE_URL) },
            date_published: "2024-01-01".to_string(),
            date_modified: "2025-11-08".to_string(),
            url: format!("{}/docs/test", SiteConfig::BASE_URL),
            main_entity_of_page: MainEntity {
                type_: "WebPage".to_string(),
                id: format!("{}/docs/test", SiteConfig::BASE_URL),
            },
            keywords: vec!["test".to_string(), "article".to_string()],
            article_section: "Components".to_string(),
        }
    }

    #[test]
    fn test_schema_serializes_to_valid_json() {
        let schema = create_test_schema();
        let json = serde_json::to_string(&schema).expect("Should serialize to JSON");
        let parsed: Value = serde_json::from_str(&json).expect("Should parse as valid JSON");

        assert!(parsed.is_object());
    }

    #[test]
    fn test_schema_has_required_context() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["@context"], "https://schema.org");
    }

    #[test]
    fn test_schema_has_correct_type() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["@type"], "TechArticle");
    }

    #[test]
    fn test_author_is_organization_type() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["author"]["@type"], "Organization");
    }

    #[test]
    fn test_author_id_references_organization() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let author_id = json["author"]["@id"].as_str().unwrap();

        assert!(author_id.ends_with("/#organization"));
        assert!(author_id.starts_with("https://"));
    }

    #[test]
    fn test_main_entity_is_webpage_type() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["mainEntityOfPage"]["@type"], "WebPage");
    }

    #[test]
    fn test_url_matches_main_entity_id() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["url"], json["mainEntityOfPage"]["@id"]);
    }

    #[test]
    fn test_date_format_is_iso8601() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        let date_published = json["datePublished"].as_str().unwrap();
        let date_modified = json["dateModified"].as_str().unwrap();

        // Basic ISO 8601 format check (YYYY-MM-DD)
        assert!(date_published.len() >= 10);
        assert!(date_modified.len() >= 10);
        assert!(date_published.contains('-'));
        assert!(date_modified.contains('-'));
    }

    #[test]
    fn test_keywords_array_serializes_correctly() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert!(json["keywords"].is_array());
        let keywords = json["keywords"].as_array().unwrap();
        assert_eq!(keywords.len(), 2);
        assert_eq!(keywords[0], "test");
        assert_eq!(keywords[1], "article");
    }

    #[test]
    fn test_empty_keywords_are_omitted() {
        let mut schema = create_test_schema();
        schema.keywords = vec![];
        let json = serde_json::to_value(&schema).unwrap();

        // Should not have keywords field when empty (skip_serializing_if)
        assert!(json.get("keywords").is_none());
    }

    #[test]
    fn test_all_required_fields_present() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        // Required Schema.org TechArticle fields
        assert!(json.get("@context").is_some());
        assert!(json.get("@type").is_some());
        assert!(json.get("headline").is_some());
        assert!(json.get("description").is_some());
        assert!(json.get("author").is_some());
        assert!(json.get("datePublished").is_some());
        assert!(json.get("dateModified").is_some());
        assert!(json.get("url").is_some());
        assert!(json.get("mainEntityOfPage").is_some());
        assert!(json.get("articleSection").is_some());
    }

    #[test]
    fn test_urls_are_absolute() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        let url = json["url"].as_str().unwrap();
        let author_id = json["author"]["@id"].as_str().unwrap();
        let entity_id = json["mainEntityOfPage"]["@id"].as_str().unwrap();

        assert!(url.starts_with("https://"));
        assert!(author_id.starts_with("https://"));
        assert!(entity_id.starts_with("https://"));
    }

    #[test]
    fn test_base_url_constant_is_valid() {
        assert!(SiteConfig::BASE_URL.starts_with("https://"));
        assert!(!SiteConfig::BASE_URL.ends_with('/'));
    }
}
