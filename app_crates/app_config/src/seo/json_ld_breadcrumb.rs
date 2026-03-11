use leptos::prelude::*;
use leptos_meta::Script;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    pub name: String,
    pub url: Option<String>, // None for current page (no link)
}

#[derive(Serialize)]
struct BreadcrumbListSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "itemListElement")]
    item_list_element: Vec<ListItem>,
}

#[derive(Serialize)]
struct ListItem {
    #[serde(rename = "@type")]
    type_: String,
    position: usize,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<String>,
}

/// JSON-LD structured data component for BreadcrumbList schema.
///
/// This component generates Schema.org BreadcrumbList structured data for navigation breadcrumbs,
/// which helps search engines understand site hierarchy and display breadcrumb trails in search results.
///
/// # Example
///
/// ```rust,ignore
/// use app_config::{BreadcrumbItem, JsonLdBreadcrumb};
///
/// let breadcrumbs = vec![
///     BreadcrumbItem {
///         name: "Home".to_string(),
///         url: Some("https://www.rust-ui.com".to_string()),
///     },
///     BreadcrumbItem {
///         name: "Components".to_string(),
///         url: Some("https://www.rust-ui.com/docs/components".to_string()),
///     },
///     BreadcrumbItem {
///         name: "Button".to_string(),
///         url: None, // Current page - no link
///     },
/// ];
///
/// view! {
///     <JsonLdBreadcrumb breadcrumbs=breadcrumbs />
/// }
/// ```
#[component]
pub fn JsonLdBreadcrumb(
    /// List of breadcrumb items from root to current page
    breadcrumbs: Vec<BreadcrumbItem>,
) -> impl IntoView {
    let items: Vec<ListItem> = breadcrumbs
        .into_iter()
        .enumerate()
        .map(|(idx, crumb)| ListItem {
            type_: "ListItem".to_string(),
            position: idx + 1,
            name: crumb.name,
            item: crumb.url,
        })
        .collect();

    let schema = BreadcrumbListSchema {
        context: "https://schema.org".to_string(),
        type_: "BreadcrumbList".to_string(),
        item_list_element: items,
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    view! { <Script type_="application/ld+json">{json_content}</Script> }
}

#[cfg(test)]
mod unit_tests {
    use serde_json::Value;

    use super::*;

    fn create_test_breadcrumbs() -> Vec<BreadcrumbItem> {
        vec![
            BreadcrumbItem { name: "Home".to_string(), url: Some("https://www.rust-ui.com".to_string()) },
            BreadcrumbItem {
                name: "Components".to_string(),
                url: Some("https://www.rust-ui.com/docs/components".to_string()),
            },
            BreadcrumbItem {
                name: "Button".to_string(),
                url: None, // Current page
            },
        ]
    }

    fn create_test_schema() -> BreadcrumbListSchema {
        let breadcrumbs = create_test_breadcrumbs();
        let items: Vec<ListItem> = breadcrumbs
            .into_iter()
            .enumerate()
            .map(|(idx, crumb)| ListItem {
                type_: "ListItem".to_string(),
                position: idx + 1,
                name: crumb.name,
                item: crumb.url,
            })
            .collect();

        BreadcrumbListSchema {
            context: "https://schema.org".to_string(),
            type_: "BreadcrumbList".to_string(),
            item_list_element: items,
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

        assert_eq!(json["@type"], "BreadcrumbList");
    }

    #[test]
    fn test_item_list_element_is_array() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert!(json["itemListElement"].is_array());
    }

    #[test]
    fn test_list_items_have_correct_type() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let items = json["itemListElement"].as_array().unwrap();

        for item in items {
            assert_eq!(item["@type"], "ListItem");
        }
    }

    #[test]
    fn test_positions_are_sequential() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let items = json["itemListElement"].as_array().unwrap();

        for (idx, item) in items.iter().enumerate() {
            assert_eq!(item["position"], idx + 1);
        }
    }

    #[test]
    fn test_last_item_has_no_url() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let items = json["itemListElement"].as_array().unwrap();
        let last_item = items.last().unwrap();

        // Last item (current page) should not have "item" field
        assert!(last_item.get("item").is_none());
    }

    #[test]
    fn test_non_last_items_have_urls() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let items = json["itemListElement"].as_array().unwrap();

        // All items except last should have "item" field with URL
        for item in &items[..items.len() - 1] {
            assert!(item.get("item").is_some());
            let url = item["item"].as_str().unwrap();
            assert!(url.starts_with("https://"));
        }
    }

    #[test]
    fn test_breadcrumb_names_are_preserved() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let items = json["itemListElement"].as_array().unwrap();

        assert_eq!(items[0]["name"], "Home");
        assert_eq!(items[1]["name"], "Components");
        assert_eq!(items[2]["name"], "Button");
    }

    #[test]
    fn test_urls_are_absolute() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let items = json["itemListElement"].as_array().unwrap();

        for item in &items[..items.len() - 1] {
            if let Some(url) = item.get("item") {
                let url_str = url.as_str().unwrap();
                assert!(url_str.starts_with("https://"));
            }
        }
    }

    #[test]
    fn test_minimum_breadcrumb_trail() {
        let breadcrumbs = vec![
            BreadcrumbItem { name: "Home".to_string(), url: Some("https://www.rust-ui.com".to_string()) },
            BreadcrumbItem { name: "Page".to_string(), url: None },
        ];

        let items: Vec<ListItem> = breadcrumbs
            .into_iter()
            .enumerate()
            .map(|(idx, crumb)| ListItem {
                type_: "ListItem".to_string(),
                position: idx + 1,
                name: crumb.name,
                item: crumb.url,
            })
            .collect();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].position, 1);
        assert_eq!(items[1].position, 2);
    }
}
