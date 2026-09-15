use clap::{Arg, ArgMatches, Command};

use crate::command_add::tree_parser::TreeParser;
use crate::command_init::workspace_utils::detect_framework;
use crate::command_list::_list::{filter_by_query, format_list, format_list_json};
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_search() -> Command {
    Command::new("search")
        .about("Search available components by name")
        .arg(Arg::new("query").help("Search query (case-insensitive)").required(true))
        .arg(Arg::new("json").long("json").help("Output as JSON").action(clap::ArgAction::SetTrue))
}

pub async fn process_search(matches: &ArgMatches) -> CliResult<()> {
    let query = matches.get_one::<String>("query").map(|s| s.as_str()).unwrap_or("");
    let json = matches.get_flag("json");

    let framework = detect_framework()?;
    let tree_content = RustUIClient::fetch_tree_md(framework).await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;
    let by_category = tree_parser.get_components_by_category();
    let filtered = filter_by_query(&by_category, query);

    let output = if json { format_list_json(&filtered)? } else { format_search_result(&filtered, query) };
    println!("{output}");
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Wraps `format_list` with a search-specific header.
pub fn format_search_result(filtered: &std::collections::BTreeMap<String, Vec<String>>, query: &str) -> String {
    let total: usize = filtered.values().map(|v| v.len()).sum();

    if total == 0 {
        return format!("No components found matching \"{query}\".");
    }

    let list = format_list(filtered);
    // Replace the generic header with a search-specific one
    list.replacen(
        &format!("Available components ({total} total)"),
        &format!("Search results for \"{query}\" ({total} found)"),
        1,
    )
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn make_map(entries: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
        entries
            .iter()
            .map(|(cat, names)| (cat.to_string(), names.iter().map(|n| n.to_string()).collect()))
            .collect()
    }

    #[test]
    fn no_match_shows_query_in_message() {
        let map = make_map(&[("ui", &["button"])]);
        let result = format_search_result(&filter_by_query(&map, "zzz"), "zzz");
        assert!(result.contains("zzz"));
        assert!(result.contains("No components found"));
    }

    #[test]
    fn match_shows_search_header_with_query() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let result = format_search_result(&filter_by_query(&map, "button"), "button");
        assert!(result.contains("Search results for \"button\""));
        assert!(result.contains("1 found"));
    }

    #[test]
    fn match_lists_components() {
        let map = make_map(&[("ui", &["button", "badge"]), ("demos", &["demo_button"])]);
        let result = format_search_result(&filter_by_query(&map, "button"), "button");
        assert!(result.contains("button"));
        assert!(result.contains("demo_button"));
        assert!(!result.contains("badge"));
    }

    #[test]
    fn search_across_multiple_categories() {
        let map = make_map(&[("ui", &["button"]), ("demos", &["demo_button"])]);
        let filtered = filter_by_query(&map, "button");
        assert!(filtered.contains_key("ui"));
        assert!(filtered.contains_key("demos"));
    }

    #[test]
    fn empty_query_returns_all() {
        let map = make_map(&[("ui", &["button", "badge"])]);
        let filtered = filter_by_query(&map, "");
        assert_eq!(filtered["ui"].len(), 2);
    }
}
