use std::collections::BTreeMap;

use clap::{Arg, ArgMatches, Command};
use serde::Serialize;

use crate::command_add::tree_parser::TreeParser;
use crate::command_init::workspace_utils::detect_framework;
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_list() -> Command {
    Command::new("list").about("List all available components from the registry").arg(
        Arg::new("json").long("json").help("Output as JSON").action(clap::ArgAction::SetTrue),
    )
}

pub async fn process_list(matches: &ArgMatches) -> CliResult<()> {
    let json = matches.get_flag("json");

    let framework = detect_framework()?;
    let tree_content = RustUIClient::fetch_tree_md(framework).await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;
    let by_category = tree_parser.get_components_by_category();

    let output = if json { format_list_json(&by_category)? } else { format_list(&by_category) };
    println!("{output}");
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

#[derive(Serialize)]
struct ListJson<'a> {
    total: usize,
    categories: &'a BTreeMap<String, Vec<String>>,
}

/// Filters components by a case-insensitive query on the component name.
/// Returns a new map containing only matching names; empty categories are dropped.
pub fn filter_by_query(by_category: &BTreeMap<String, Vec<String>>, query: &str) -> BTreeMap<String, Vec<String>> {
    let q = query.to_lowercase();
    by_category
        .iter()
        .filter_map(|(cat, names)| {
            let matched: Vec<String> = names.iter().filter(|n| n.to_lowercase().contains(&q)).cloned().collect();
            if matched.is_empty() { None } else { Some((cat.clone(), matched)) }
        })
        .collect()
}

/// Human-readable formatter — one component per line, grouped by category.
pub fn format_list(by_category: &BTreeMap<String, Vec<String>>) -> String {
    let total: usize = by_category.values().map(|v| v.len()).sum();

    if total == 0 {
        return "No components found.".to_string();
    }

    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Available components ({total} total)"));
    lines.push(String::new());

    for (category, names) in by_category {
        lines.push(format!("  {} ({})", category, names.len()));
        for name in names {
            lines.push(format!("    {name}"));
        }
        lines.push(String::new());
    }

    if lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n")
}

/// Machine-readable JSON formatter.
pub fn format_list_json(by_category: &BTreeMap<String, Vec<String>>) -> CliResult<String> {
    let total: usize = by_category.values().map(|v| v.len()).sum();
    let payload = ListJson { total, categories: by_category };
    serde_json::to_string_pretty(&payload).map_err(Into::into)
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map(entries: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
        entries
            .iter()
            .map(|(cat, names)| (cat.to_string(), names.iter().map(|n| n.to_string()).collect()))
            .collect()
    }

    // --- format_list ---

    #[test]
    fn empty_map_shows_no_components_message() {
        let result = format_list(&make_map(&[]));
        assert_eq!(result, "No components found.");
    }

    #[test]
    fn shows_total_count() {
        let map = make_map(&[("ui", &["button", "badge"]), ("demos", &["demo_button"])]);
        assert!(format_list(&map).contains("3 total"));
    }

    #[test]
    fn shows_category_with_count() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        assert!(format_list(&map).contains("ui (2)"));
    }

    #[test]
    fn shows_each_component_on_its_own_line() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let result = format_list(&map);
        assert!(result.contains("    button"));
        assert!(result.contains("    badge"));
    }

    #[test]
    fn categories_appear_in_alphabetical_order() {
        let map = make_map(&[("ui", &["button"]), ("demos", &["demo_button"]), ("hooks", &["use_x"])]);
        let result = format_list(&map);
        let demos_pos = result.find("demos").unwrap();
        let hooks_pos = result.find("hooks").unwrap();
        let ui_pos = result.find("  ui").unwrap();
        assert!(demos_pos < hooks_pos);
        assert!(hooks_pos < ui_pos);
    }

    #[test]
    fn single_category_single_component() {
        let map = make_map(&[("ui", &["button"])]);
        let result = format_list(&map);
        assert!(result.contains("1 total"));
        assert!(result.contains("ui (1)"));
        assert!(result.contains("    button"));
    }

    // --- filter_by_query ---

    #[test]
    fn filter_returns_exact_match() {
        let map = make_map(&[("ui", &["button", "badge", "card"])]);
        let filtered = filter_by_query(&map, "button");
        assert_eq!(filtered["ui"], vec!["button"]);
    }

    #[test]
    fn filter_is_case_insensitive() {
        let map = make_map(&[("ui", &["button", "Badge"])]);
        let filtered = filter_by_query(&map, "BUTTON");
        assert!(filtered["ui"].contains(&"button".to_string()));
    }

    #[test]
    fn filter_matches_partial_name() {
        let map = make_map(&[("demos", &["demo_button", "demo_badge", "demo_card"])]);
        let filtered = filter_by_query(&map, "badge");
        assert_eq!(filtered["demos"], vec!["demo_badge"]);
    }

    #[test]
    fn filter_drops_empty_categories() {
        let map = make_map(&[("ui", &["button"]), ("demos", &["demo_card"])]);
        let filtered = filter_by_query(&map, "button");
        assert!(filtered.contains_key("ui"));
        assert!(!filtered.contains_key("demos"));
    }

    #[test]
    fn filter_returns_empty_map_when_no_match() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let filtered = filter_by_query(&map, "zzz");
        assert!(filtered.is_empty());
    }

    #[test]
    fn filter_empty_query_returns_all() {
        let map = make_map(&[("ui", &["button", "badge"]), ("demos", &["demo_button"])]);
        let filtered = filter_by_query(&map, "");
        assert_eq!(filtered.len(), map.len());
    }

    // --- format_list_json ---

    #[test]
    fn json_output_is_valid() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let json = format_list_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn json_contains_total_and_categories() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let json = format_list_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["total"], 2);
        assert!(parsed["categories"].is_object());
    }

    #[test]
    fn json_categories_contain_component_arrays() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let json = format_list_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let ui = parsed["categories"]["ui"].as_array().unwrap();
        assert_eq!(ui.len(), 2);
    }

    #[test]
    fn json_empty_map_has_zero_total() {
        let map = make_map(&[]);
        let json = format_list_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["total"], 0);
    }
}
