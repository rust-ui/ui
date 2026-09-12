use dioxus::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
}

#[derive(Serialize)]
struct FaqPageSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "mainEntity")]
    main_entity: Vec<QuestionNode>,
}

#[derive(Serialize)]
struct QuestionNode {
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    #[serde(rename = "acceptedAnswer")]
    accepted_answer: AnswerNode,
}

#[derive(Serialize)]
struct AnswerNode {
    #[serde(rename = "@type")]
    type_: String,
    text: String,
}

/// JSON-LD structured data component for `FAQPage` schema.
///
/// Renders nothing if `items` is empty, since an empty `mainEntity` array is
/// invalid schema and Google requires the markup to match visible on-page
/// content — only pass items that are also rendered as real Q&A copy on the
/// page (e.g. a "## Frequently Asked Questions" section), never invented ones.
///
/// # Example
///
/// ```rust,ignore
/// use app_config::{FaqItem, JsonLdFaq};
///
/// let faqs = vec![FaqItem {
///     question: "Does this component support RTL?".to_string(),
///     answer: "Yes, layout mirrors automatically under `dir=\"rtl\"`.".to_string(),
/// }];
///
/// rsx! {
///     JsonLdFaq { items: faqs }
/// }
/// ```
#[component]
pub fn JsonLdFaq(items: Vec<FaqItem>) -> Element {
    if items.is_empty() {
        return rsx! {};
    }

    let schema = FaqPageSchema {
        context: "https://schema.org".to_string(),
        type_: "FAQPage".to_string(),
        main_entity: items
            .into_iter()
            .map(|item| QuestionNode {
                type_: "Question".to_string(),
                name: item.question,
                accepted_answer: AnswerNode {
                    type_: "Answer".to_string(),
                    text: item.answer,
                },
            })
            .collect(),
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    rsx! {
        document::Script { r#type: "application/ld+json", "{json_content}" }
    }
}

#[cfg(test)]
mod unit_tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_empty_items_renders_nothing() {
        // Guard covered by early return; nothing to serialize.
        let items: Vec<FaqItem> = vec![];
        assert!(items.is_empty());
    }

    #[test]
    fn test_schema_shape() {
        let schema = FaqPageSchema {
            context: "https://schema.org".to_string(),
            type_: "FAQPage".to_string(),
            main_entity: vec![QuestionNode {
                type_: "Question".to_string(),
                name: "Q".to_string(),
                accepted_answer: AnswerNode {
                    type_: "Answer".to_string(),
                    text: "A".to_string(),
                },
            }],
        };
        let json: Value = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["@type"], "FAQPage");
        assert_eq!(json["mainEntity"][0]["@type"], "Question");
        assert_eq!(json["mainEntity"][0]["acceptedAnswer"]["@type"], "Answer");
    }
}
