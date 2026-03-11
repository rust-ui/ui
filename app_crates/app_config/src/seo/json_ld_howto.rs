use leptos::prelude::*;
use leptos_meta::Script;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct HowToStep {
    pub name: String,
    pub text: String,
}

#[derive(Serialize)]
struct HowToSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    step: Vec<Step>,
}

#[derive(Serialize)]
struct Step {
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    text: String,
}

/// JSON-LD structured data component for HowTo schema.
///
/// This component generates Schema.org HowTo structured data for step-by-step guides,
/// tutorials, and installation instructions. This can help pages appear as featured
/// snippets in search results with step-by-step formatting.
///
/// # Example
///
/// ```rust,ignore
/// use app_config::{HowToStep, JsonLdHowTo};
///
/// let steps = vec![
///     HowToStep {
///         name: "Install dependencies".to_string(),
///         text: "Run 'cargo add leptos_ui' to add the component library".to_string(),
///     },
///     HowToStep {
///         name: "Import the component".to_string(),
///         text: "Add 'use leptos_ui::Button;' to your file".to_string(),
///     },
///     HowToStep {
///         name: "Use in your view".to_string(),
///         text: "Add '<Button>Click me</Button>' to your view! macro".to_string(),
///     },
/// ];
///
/// view! {
///     <JsonLdHowTo
///         name="How to Install Leptos Button Component"
///         description="Quick guide to add and use the Button component"
///         steps=steps
///     />
/// }
/// ```
#[component]
pub fn JsonLdHowTo(
    /// Guide/tutorial title (e.g., "How to Install Leptos Button Component")
    name: String,
    /// Optional description of what this guide accomplishes
    #[prop(optional)]
    description: Option<String>,
    /// List of steps in order
    steps: Vec<HowToStep>,
) -> impl IntoView {
    let schema_steps: Vec<Step> =
        steps.into_iter().map(|s| Step { type_: "HowToStep".to_string(), name: s.name, text: s.text }).collect();

    let schema = HowToSchema {
        context: "https://schema.org".to_string(),
        type_: "HowTo".to_string(),
        name,
        description,
        step: schema_steps,
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    view! { <Script type_="application/ld+json">{json_content}</Script> }
}

#[cfg(test)]
mod unit_tests {
    use serde_json::Value;

    use super::*;

    fn create_test_steps() -> Vec<HowToStep> {
        vec![
            HowToStep { name: "Install dependencies".to_string(), text: "Run 'cargo add leptos_ui'".to_string() },
            HowToStep { name: "Import component".to_string(), text: "Add 'use leptos_ui::Button;'".to_string() },
            HowToStep { name: "Use in view".to_string(), text: "Add '<Button>Click</Button>'".to_string() },
        ]
    }

    fn create_test_schema() -> HowToSchema {
        let steps = create_test_steps();
        let schema_steps: Vec<Step> =
            steps.into_iter().map(|s| Step { type_: "HowToStep".to_string(), name: s.name, text: s.text }).collect();

        HowToSchema {
            context: "https://schema.org".to_string(),
            type_: "HowTo".to_string(),
            name: "How to Install Button Component".to_string(),
            description: Some("Quick installation guide".to_string()),
            step: schema_steps,
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

        assert_eq!(json["@type"], "HowTo");
    }

    #[test]
    fn test_steps_are_array() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert!(json["step"].is_array());
    }

    #[test]
    fn test_steps_have_correct_type() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let steps = json["step"].as_array().unwrap();

        for step in steps {
            assert_eq!(step["@type"], "HowToStep");
        }
    }

    #[test]
    fn test_steps_have_name_and_text() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let steps = json["step"].as_array().unwrap();

        for step in steps {
            assert!(step.get("name").is_some());
            assert!(step.get("text").is_some());
            assert!(step["name"].is_string());
            assert!(step["text"].is_string());
        }
    }

    #[test]
    fn test_step_content_preserved() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let steps = json["step"].as_array().unwrap();

        assert_eq!(steps[0]["name"], "Install dependencies");
        assert_eq!(steps[0]["text"], "Run 'cargo add leptos_ui'");
        assert_eq!(steps[1]["name"], "Import component");
        assert_eq!(steps[2]["name"], "Use in view");
    }

    #[test]
    fn test_description_is_optional() {
        let mut schema = create_test_schema();
        schema.description = None;
        let json = serde_json::to_value(&schema).unwrap();

        // Description field should be omitted when None
        assert!(json.get("description").is_none());
    }

    #[test]
    fn test_description_included_when_some() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        assert!(json.get("description").is_some());
        assert_eq!(json["description"], "Quick installation guide");
    }

    #[test]
    fn test_all_required_fields_present() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();

        // Required Schema.org HowTo fields
        assert!(json.get("@context").is_some());
        assert!(json.get("@type").is_some());
        assert!(json.get("name").is_some());
        assert!(json.get("step").is_some());
    }

    #[test]
    fn test_minimum_steps() {
        let steps = vec![HowToStep { name: "Step 1".to_string(), text: "Do something".to_string() }];

        let schema_steps: Vec<Step> =
            steps.into_iter().map(|s| Step { type_: "HowToStep".to_string(), name: s.name, text: s.text }).collect();

        assert_eq!(schema_steps.len(), 1);
    }

    #[test]
    fn test_step_order_preserved() {
        let schema = create_test_schema();
        let json = serde_json::to_value(&schema).unwrap();
        let steps = json["step"].as_array().unwrap();

        // Steps should be in order
        assert_eq!(steps.len(), 3);
        assert!(steps[0]["name"].as_str().unwrap().contains("Install"));
        assert!(steps[1]["name"].as_str().unwrap().contains("Import"));
        assert!(steps[2]["name"].as_str().unwrap().contains("Use"));
    }
}
