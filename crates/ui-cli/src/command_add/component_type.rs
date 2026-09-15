use heck::ToSnakeCase;
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum ComponentType {
    Ui,
    Demos,
    Hooks,
    Extensions,
}

impl ComponentType {
    /// Get the directory path for this component type
    pub fn to_path(&self) -> String {
        self.to_string().to_snake_case()
    }

    /// Determine component type from component name patterns
    pub fn from_component_name(component_name: &str) -> Self {
        if component_name.starts_with("demo_") {
            Self::Demos
        } else if component_name.starts_with("use_") {
            Self::Hooks
        } else if component_name.contains("extension") {
            Self::Extensions
        } else {
            Self::Ui
        }
    }
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    // --- from_component_name ---

    #[test]
    fn demo_prefix_maps_to_demos() {
        assert_eq!(ComponentType::from_component_name("demo_button"), ComponentType::Demos);
    }

    #[test]
    fn use_prefix_maps_to_hooks() {
        assert_eq!(ComponentType::from_component_name("use_floating"), ComponentType::Hooks);
    }

    #[test]
    fn extension_substring_maps_to_extensions() {
        assert_eq!(ComponentType::from_component_name("my_extension"), ComponentType::Extensions);
    }

    #[test]
    fn plain_name_maps_to_ui() {
        assert_eq!(ComponentType::from_component_name("button"), ComponentType::Ui);
        assert_eq!(ComponentType::from_component_name("badge"), ComponentType::Ui);
        assert_eq!(ComponentType::from_component_name("card"), ComponentType::Ui);
    }

    // demo_ takes priority over extension substring
    #[test]
    fn demo_prefix_takes_priority_over_extension() {
        assert_eq!(ComponentType::from_component_name("demo_extension"), ComponentType::Demos);
    }

    // --- to_path ---

    #[test]
    fn to_path_returns_lowercase_string() {
        assert_eq!(ComponentType::Ui.to_path(), "ui");
        assert_eq!(ComponentType::Demos.to_path(), "demos");
        assert_eq!(ComponentType::Hooks.to_path(), "hooks");
        assert_eq!(ComponentType::Extensions.to_path(), "extensions");
    }
}
