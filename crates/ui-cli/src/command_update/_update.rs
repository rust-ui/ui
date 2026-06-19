use std::path::Path;

use clap::{Arg, ArgMatches, Command};
use serde::Serialize;

use crate::command_add::component_type::ComponentType;
use crate::command_add::installed::get_installed_components;
use crate::command_init::config::UiConfig;
use crate::command_init::workspace_utils::detect_framework;
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

const UI_CONFIG_TOML: &str = "ui_config.toml";

/* ========================================================== */
/*                        📦 TYPES 📦                         */
/* ========================================================== */

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStatus {
    UpToDate,
    Outdated,
    NotInRegistry,
}

#[derive(Debug, Serialize)]
pub struct ComponentUpdateInfo {
    pub name: String,
    pub status: ComponentStatus,
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_update() -> Command {
    Command::new("update")
        .about("Check installed components for updates against the registry")
        .arg(Arg::new("json").long("json").help("Output as JSON").action(clap::ArgAction::SetTrue))
}

pub async fn process_update(matches: &ArgMatches) -> CliResult<()> {
    let json = matches.get_flag("json");

    let config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?;
    let framework = detect_framework()?;
    let base_path = config.base_path_components;

    let mut installed: Vec<String> = get_installed_components(&base_path).into_iter().collect();
    installed.sort();

    if installed.is_empty() {
        println!("No components installed.");
        return Ok(());
    }

    println!("Checking {} installed component{}...", installed.len(), if installed.len() == 1 { "" } else { "s" });

    let mut results: Vec<ComponentUpdateInfo> = Vec::new();

    for name in &installed {
        let component_type = ComponentType::from_component_name(name);
        let relative_path = format!("{}/{}.rs", component_type.to_path(), name);
        let local_path = Path::new(&base_path).join(&relative_path);

        let local_content = match std::fs::read_to_string(&local_path) {
            Ok(c) => c,
            Err(_) => {
                results.push(ComponentUpdateInfo { name: name.clone(), status: ComponentStatus::NotInRegistry });
                continue;
            }
        };

        let status = match RustUIClient::fetch_styles_default(name, framework).await {
            Ok(remote_content) => compare_content(&local_content, &remote_content),
            Err(_) => ComponentStatus::NotInRegistry,
        };

        results.push(ComponentUpdateInfo { name: name.clone(), status });
    }

    let output =
        if json { format_update_json(&results)? } else { format_update_summary(&results) };
    println!("{output}");

    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Compares local and remote content, trimming whitespace to avoid false positives.
pub fn compare_content(local: &str, remote: &str) -> ComponentStatus {
    if local.trim() == remote.trim() { ComponentStatus::UpToDate } else { ComponentStatus::Outdated }
}

/// Human-readable summary of update check results.
pub fn format_update_summary(results: &[ComponentUpdateInfo]) -> String {
    if results.is_empty() {
        return String::new();
    }

    let name_width = results.iter().map(|r| r.name.len()).max().unwrap_or(0);

    let mut lines: Vec<String> = results
        .iter()
        .map(|r| {
            let padded = format!("{:<width$}", r.name, width = name_width);
            match r.status {
                ComponentStatus::UpToDate => format!("  ✅ {padded}  up to date"),
                ComponentStatus::Outdated => {
                    format!("  ⚠️  {padded}  outdated  →  ui add {} -y", r.name)
                }
                ComponentStatus::NotInRegistry => format!("  ❓ {padded}  not in registry"),
            }
        })
        .collect();

    let outdated_count = results.iter().filter(|r| r.status == ComponentStatus::Outdated).count();
    lines.push(String::new());
    if outdated_count == 0 {
        lines.push("All components are up to date.".to_string());
    } else {
        lines.push(format!(
            "{outdated_count} component{} outdated.",
            if outdated_count == 1 { " is" } else { "s are" }
        ));
    }

    lines.join("\n")
}

/// Machine-readable JSON output.
pub fn format_update_json(results: &[ComponentUpdateInfo]) -> CliResult<String> {
    serde_json::to_string_pretty(results).map_err(Into::into)
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    fn info(name: &str, status: ComponentStatus) -> ComponentUpdateInfo {
        ComponentUpdateInfo { name: name.to_string(), status }
    }

    // --- compare_content ---

    #[test]
    fn identical_content_is_up_to_date() {
        assert_eq!(compare_content("fn foo() {}", "fn foo() {}"), ComponentStatus::UpToDate);
    }

    #[test]
    fn different_content_is_outdated() {
        assert_eq!(compare_content("fn foo() {}", "fn bar() {}"), ComponentStatus::Outdated);
    }

    #[test]
    fn trailing_newline_difference_is_ignored() {
        assert_eq!(compare_content("fn foo() {}\n", "fn foo() {}"), ComponentStatus::UpToDate);
    }

    #[test]
    fn leading_whitespace_difference_is_ignored() {
        assert_eq!(compare_content("  fn foo() {}", "fn foo() {}"), ComponentStatus::UpToDate);
    }

    #[test]
    fn empty_strings_are_equal() {
        assert_eq!(compare_content("", ""), ComponentStatus::UpToDate);
    }

    // --- format_update_summary ---

    #[test]
    fn empty_results_returns_empty_string() {
        assert_eq!(format_update_summary(&[]), String::new());
    }

    #[test]
    fn all_up_to_date_shows_success_message() {
        let results = vec![info("button", ComponentStatus::UpToDate), info("badge", ComponentStatus::UpToDate)];
        let out = format_update_summary(&results);
        assert!(out.contains("All components are up to date."));
        assert!(!out.contains("outdated"));
    }

    #[test]
    fn outdated_component_shows_update_hint() {
        let results = vec![info("button", ComponentStatus::Outdated)];
        let out = format_update_summary(&results);
        assert!(out.contains("ui add button -y"));
        assert!(out.contains("outdated"));
    }

    #[test]
    fn outdated_count_is_correct_singular() {
        let results = vec![info("button", ComponentStatus::Outdated), info("badge", ComponentStatus::UpToDate)];
        let out = format_update_summary(&results);
        assert!(out.contains("1 component is outdated."));
    }

    #[test]
    fn outdated_count_is_correct_plural() {
        let results =
            vec![info("button", ComponentStatus::Outdated), info("badge", ComponentStatus::Outdated)];
        let out = format_update_summary(&results);
        assert!(out.contains("2 components are outdated."));
    }

    #[test]
    fn not_in_registry_shows_question_mark() {
        let results = vec![info("my_custom", ComponentStatus::NotInRegistry)];
        let out = format_update_summary(&results);
        assert!(out.contains("not in registry"));
    }

    #[test]
    fn all_statuses_shown_together() {
        let results = vec![
            info("button", ComponentStatus::UpToDate),
            info("badge", ComponentStatus::Outdated),
            info("custom", ComponentStatus::NotInRegistry),
        ];
        let out = format_update_summary(&results);
        assert!(out.contains("up to date"));
        assert!(out.contains("outdated"));
        assert!(out.contains("not in registry"));
    }

    // --- format_update_json ---

    #[test]
    fn json_output_is_valid() {
        let results = vec![info("button", ComponentStatus::UpToDate)];
        let json = format_update_json(&results).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
    }

    #[test]
    fn json_contains_name_and_status() {
        let results = vec![info("button", ComponentStatus::Outdated)];
        let json = format_update_json(&results).unwrap();
        assert!(json.contains("button"));
        assert!(json.contains("outdated"));
    }

    #[test]
    fn json_status_serialized_as_snake_case() {
        let results = vec![
            info("a", ComponentStatus::UpToDate),
            info("b", ComponentStatus::NotInRegistry),
        ];
        let json = format_update_json(&results).unwrap();
        assert!(json.contains("up_to_date"));
        assert!(json.contains("not_in_registry"));
    }
}
