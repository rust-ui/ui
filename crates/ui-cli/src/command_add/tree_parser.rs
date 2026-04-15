use std::collections::{BTreeMap, HashMap, HashSet};

use crate::shared::cli_error::CliResult;

#[derive(Debug, Clone)]
pub struct TreeParser {
    components: HashMap<String, ComponentEntry>,
}

#[derive(Debug, Clone)]
pub struct ComponentEntry {
    pub name: String,
    pub category: String,
    pub dependencies: Vec<String>,
    pub cargo_deps: Vec<String>,
    pub js_files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedSet {
    pub components: HashSet<String>,
    pub cargo_deps: HashSet<String>,
    pub parent_dirs: HashSet<String>,
    pub js_files: HashSet<String>,
}

impl TreeParser {
    pub fn parse_tree_md(content: &str) -> CliResult<Self> {
        let mut components = HashMap::new();
        let mut current_component: Option<ComponentEntry> = None;
        let mut dependency_stack: Vec<String> = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and code block markers
            if line.is_empty() || line.starts_with("```") {
                continue;
            }

            // Parse component lines (*)
            if let Some(line_content) = line.strip_prefix("* ") {
                // Save previous component if exists
                if let Some(component) = current_component.take() {
                    components.insert(component.name.clone(), component);
                }

                if let Some((name_part, category_part)) = line_content.rsplit_once(" (") {
                    let name = name_part.trim().to_string();
                    let category = category_part.trim_end_matches(')').to_string();

                    current_component = Some(ComponentEntry {
                        name: name.clone(),
                        category,
                        dependencies: Vec::new(),
                        cargo_deps: Vec::new(),
                        js_files: Vec::new(),
                    });

                    dependency_stack.clear();
                    dependency_stack.push(name);
                }
            }
            // Parse dependency lines (**)
            else if let Some(dep_content) = line.strip_prefix("** ") {
                if let Some(cargo_dep_name) = dep_content.strip_prefix("cargo: ") {
                    // Cargo dependency
                    let cargo_dep = cargo_dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.cargo_deps.push(cargo_dep);
                    }
                } else if let Some(js_path) = dep_content.strip_prefix("js: ") {
                    // JS file dependency
                    let js_file = js_path.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.js_files.push(js_file);
                    }
                } else if let Some((dep_name, _)) = dep_content.rsplit_once(" (") {
                    // Registry dependency
                    let dep_name = dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.dependencies.push(dep_name.clone());
                    }

                    // Update dependency stack for nested dependencies
                    dependency_stack.truncate(1); // Keep only root component
                    dependency_stack.push(dep_name);
                }
            }
            // Parse nested dependency lines (***)
            else if let Some(dep_content) = line.strip_prefix("*** ") {
                if let Some(cargo_dep_name) = dep_content.strip_prefix("cargo: ") {
                    // Nested cargo dependency - add to root component
                    let cargo_dep = cargo_dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.cargo_deps.push(cargo_dep);
                    }
                } else if let Some(js_path) = dep_content.strip_prefix("js: ") {
                    // Nested JS file dependency - add to root component
                    let js_file = js_path.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.js_files.push(js_file);
                    }
                } else if let Some((dep_name, _)) = dep_content.rsplit_once(" (") {
                    // Nested registry dependency - add to root component
                    let dep_name = dep_name.trim().to_string();
                    if let Some(ref mut component) = current_component {
                        component.dependencies.push(dep_name);
                    }
                }
            }
        }

        // Save last component
        if let Some(component) = current_component {
            components.insert(component.name.clone(), component);
        }

        Ok(TreeParser { components })
    }

    pub fn get_all_component_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.components.keys().cloned().collect();
        names.sort();
        names
    }

    /// Returns components grouped by category, with both categories and names sorted.
    pub fn get_components_by_category(&self) -> BTreeMap<String, Vec<String>> {
        let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for entry in self.components.values() {
            map.entry(entry.category.clone()).or_default().push(entry.name.clone());
        }
        for names in map.values_mut() {
            names.sort();
        }
        map
    }

    pub fn get_dependencies_map(&self) -> HashMap<String, Vec<String>> {
        self.components
            .iter()
            .map(|(name, entry)| (name.clone(), entry.dependencies.clone()))
            .collect()
    }

    pub fn resolve_dependencies(&self, user_components: &[String]) -> CliResult<ResolvedSet> {
        let mut resolved_components = HashSet::new();
        let mut resolved_cargo_deps = HashSet::new();
        let mut resolved_parent_dirs = HashSet::new();
        let mut resolved_js_files = HashSet::new();

        // Process each user component
        for component_name in user_components {
            if let Some(component_entry) = self.components.get(component_name) {
                // Add the component itself
                resolved_components.insert(component_name.clone());
                resolved_parent_dirs.insert(component_entry.category.clone());

                // Add its direct dependencies
                for dep in &component_entry.dependencies {
                    resolved_components.insert(dep.clone());

                    // Add parent dir for dependency
                    if let Some(dep_entry) = self.components.get(dep) {
                        resolved_parent_dirs.insert(dep_entry.category.clone());
                    }
                }

                // Add cargo dependencies
                for cargo_dep in &component_entry.cargo_deps {
                    resolved_cargo_deps.insert(cargo_dep.clone());
                }

                // Add JS file dependencies
                for js_file in &component_entry.js_files {
                    resolved_js_files.insert(js_file.clone());
                }
            } else {
                println!("⚠️  Component '{component_name}' not found in registry. Skipping...");
            }
        }

        // Debug output — intentionally kept for visibility during development
        println!("📦 Final set of resolved components: {resolved_components:?}");
        println!("📦 Final set of cargo dependencies: {resolved_cargo_deps:?}");
        if !resolved_js_files.is_empty() {
            println!("📦 Final set of JS files: {resolved_js_files:?}");
        }

        Ok(ResolvedSet {
            components: resolved_components,
            cargo_deps: resolved_cargo_deps,
            parent_dirs: resolved_parent_dirs,
            js_files: resolved_js_files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TREE: &str = r#"
* button (ui)
** badge (ui)
** cargo: some-crate

* badge (ui)

* card (ui)
** button (ui)
*** badge (ui)

* demo_button (demos)
** button (ui)

* select (ui)
** cargo: strum
** js: /hooks/lazy_load_sonner.js

* sheet (ui)
** js: /hooks/lazy_load_sonner.js
** button (ui)
"#;

    #[test]
    fn parse_tree_md_extracts_components() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let names = parser.get_all_component_names();
        assert!(names.contains(&"button".to_string()));
        assert!(names.contains(&"badge".to_string()));
        assert!(names.contains(&"card".to_string()));
        assert!(names.contains(&"demo_button".to_string()));
    }

    #[test]
    fn parse_tree_md_extracts_dependencies() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let deps_map = parser.get_dependencies_map();

        assert_eq!(deps_map.get("button").unwrap(), &vec!["badge".to_string()]);
        assert!(deps_map.get("badge").unwrap().is_empty());
        assert_eq!(deps_map.get("card").unwrap(), &vec!["button".to_string(), "badge".to_string()]);
    }

    #[test]
    fn parse_tree_md_extracts_cargo_deps() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let entry = parser.components.get("button").unwrap();
        assert!(entry.cargo_deps.contains(&"some-crate".to_string()));
    }

    #[test]
    fn parse_tree_md_extracts_category() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        assert_eq!(parser.components.get("button").unwrap().category, "ui");
        assert_eq!(parser.components.get("demo_button").unwrap().category, "demos");
    }

    #[test]
    fn get_all_component_names_sorted() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let names = parser.get_all_component_names();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn resolve_dependencies_collects_cargo_deps() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let resolved = parser.resolve_dependencies(&["button".to_string()]).unwrap();
        assert!(resolved.cargo_deps.contains("some-crate"));
    }

    #[test]
    fn resolve_dependencies_includes_transitive() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let resolved = parser.resolve_dependencies(&["card".to_string()]).unwrap();

        assert!(resolved.components.contains("card"));
        assert!(resolved.components.contains("button"));
        assert!(resolved.components.contains("badge"));
    }

    #[test]
    fn resolve_dependencies_collects_parent_dirs() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let resolved = parser.resolve_dependencies(&["demo_button".to_string()]).unwrap();

        assert!(resolved.parent_dirs.contains("demos"));
        assert!(resolved.parent_dirs.contains("ui"));
    }

    #[test]
    fn resolve_dependencies_missing_component_skipped() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let resolved = parser.resolve_dependencies(&["nonexistent".to_string()]).unwrap();
        assert!(resolved.components.is_empty());
    }

    #[test]
    fn parse_tree_md_extracts_js_files() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let select = parser.components.get("select").unwrap();
        assert!(select.js_files.contains(&"/hooks/lazy_load_sonner.js".to_string()));
    }

    #[test]
    fn resolve_dependencies_collects_js_files() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let resolved = parser.resolve_dependencies(&["select".to_string()]).unwrap();
        assert!(resolved.js_files.contains("/hooks/lazy_load_sonner.js"));
    }

    #[test]
    fn resolve_dependencies_js_files_deduped() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        // Both select and sheet use the same JS file
        let resolved = parser.resolve_dependencies(&["select".to_string(), "sheet".to_string()]).unwrap();
        // Should only contain one instance
        assert_eq!(resolved.js_files.len(), 1);
        assert!(resolved.js_files.contains("/hooks/lazy_load_sonner.js"));
    }

    #[test]
    fn component_without_js_has_empty_js_files() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let button = parser.components.get("button").unwrap();
        assert!(button.js_files.is_empty());
    }

    #[test]
    fn get_components_by_category_groups_correctly() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let by_cat = parser.get_components_by_category();

        assert!(by_cat.contains_key("ui"));
        assert!(by_cat.contains_key("demos"));
        assert!(by_cat["ui"].contains(&"button".to_string()));
        assert!(by_cat["demos"].contains(&"demo_button".to_string()));
    }

    #[test]
    fn get_components_by_category_names_are_sorted() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let by_cat = parser.get_components_by_category();

        let ui = &by_cat["ui"];
        let mut sorted = ui.clone();
        sorted.sort();
        assert_eq!(ui, &sorted);
    }

    #[test]
    fn get_components_by_category_categories_are_sorted() {
        let parser = TreeParser::parse_tree_md(SAMPLE_TREE).unwrap();
        let by_cat = parser.get_components_by_category();

        let keys: Vec<&String> = by_cat.keys().collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }
}
