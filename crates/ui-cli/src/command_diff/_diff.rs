use std::path::Path;

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use serde::Serialize;
use similar::{ChangeTag, TextDiff};

use crate::command_add::component_type::ComponentType;
use crate::command_add::installed::get_installed_components;
use crate::command_init::config::UiConfig;
use crate::command_init::workspace_utils::detect_framework;
use crate::shared::framework::Framework;
use crate::shared::cli_error::CliResult;
use crate::shared::rust_ui_client::RustUIClient;

const UI_CONFIG_TOML: &str = "ui_config.toml";
const CONTEXT_LINES: usize = 3;

/* ========================================================== */
/*                        📦 TYPES 📦                         */
/* ========================================================== */

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffStatus {
    UpToDate,
    Changed,
    NotInRegistry,
}

#[derive(Debug, Clone)]
pub struct ComponentDiff {
    pub name: String,
    pub status: DiffStatus,
    pub local: String,
    pub remote: String,
}

/* ========================================================== */
/*                        🔧 COMMAND 🔧                       */
/* ========================================================== */

pub fn command_diff() -> Command {
    Command::new("diff")
        .about("Show line-by-line diff of installed components vs the registry")
        .arg(Arg::new("component").help("Component name to diff (omit to diff all installed)").required(false))
        .arg(Arg::new("json").long("json").help("Output as JSON").action(clap::ArgAction::SetTrue))
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

/// Fetch registry content and compute diffs for a list of component names.
/// Names are processed in the order given; sort before calling if needed.
pub async fn diff_components(
    names: &[String],
    base_path: &str,
    framework: Framework,
) -> CliResult<Vec<ComponentDiff>> {
    let mut diffs: Vec<ComponentDiff> = Vec::new();
    for name in names {
        let component_type = ComponentType::from_component_name(name);
        let local_path = Path::new(base_path).join(component_type.to_path()).join(format!("{name}.rs"));
        match RustUIClient::fetch_styles_default(name, framework).await {
            Ok(remote) => {
                let local = std::fs::read_to_string(&local_path).unwrap_or_default();
                let status = if local == remote { DiffStatus::UpToDate } else { DiffStatus::Changed };
                diffs.push(ComponentDiff { name: name.clone(), status, local, remote });
            }
            Err(_) => {
                diffs.push(ComponentDiff {
                    name: name.clone(),
                    status: DiffStatus::NotInRegistry,
                    local: String::new(),
                    remote: String::new(),
                });
            }
        }
    }
    Ok(diffs)
}

pub async fn process_diff(matches: &ArgMatches) -> CliResult<()> {
    let json = matches.get_flag("json");
    let component_arg: Option<&String> = matches.get_one("component");

    let config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?;
    let framework = detect_framework()?;
    let base_path = config.base_path_components;

    let names: Vec<String> = if let Some(name) = component_arg {
        vec![name.clone()]
    } else {
        let mut installed: Vec<String> = get_installed_components(&base_path).into_iter().collect();
        installed.sort();
        installed
    };

    if names.is_empty() {
        println!("No components installed.");
        return Ok(());
    }

    if component_arg.is_none() {
        println!("Checking {} installed component{}...\n", names.len(), if names.len() == 1 { "" } else { "s" });
    }

    let diffs = diff_components(&names, &base_path, framework).await?;

    let output = if json { format_diff_json(&diffs)? } else { format_diff_human(&diffs) };
    println!("{output}");

    Ok(())
}

/* ========================================================== */
/*                      🖨  FORMATTERS 🖨                      */
/* ========================================================== */

/// Human-readable diff output with context lines.
pub fn format_diff_human(diffs: &[ComponentDiff]) -> String {
    let name_width = diffs.iter().map(|d| d.name.len()).max().unwrap_or(0);
    let mut output = String::new();

    // When showing multiple components, show a summary line for each
    let multi = diffs.len() > 1;

    let mut changed_count = 0;

    for diff in diffs {
        match diff.status {
            DiffStatus::UpToDate => {
                if multi {
                    let padded = format!("{:<width$}", diff.name, width = name_width);
                    output.push_str(&format!("  {} {}  up to date\n", "✅".green(), padded));
                }
            }
            DiffStatus::NotInRegistry => {
                let padded = format!("{:<width$}", diff.name, width = name_width);
                output.push_str(&format!("  {} {}  not in registry\n", "❓", padded));
            }
            DiffStatus::Changed => {
                changed_count += 1;
                let td = TextDiff::from_lines(&diff.local, &diff.remote);
                let change_count = td.iter_all_changes().filter(|c| c.tag() != ChangeTag::Equal).count();

                if multi {
                    let padded = format!("{:<width$}", diff.name, width = name_width);
                    output.push_str(&format!(
                        "  {} {}  changed  ({} line{})\n",
                        "⚠️ ".yellow(),
                        padded,
                        change_count,
                        if change_count == 1 { "" } else { "s" }
                    ));
                }

                output.push_str(&format!("\n--- {} (local)\n+++ {} (registry)\n\n", diff.name, diff.name));
                let mut first = true;
                for group in td.grouped_ops(CONTEXT_LINES) {
                    if !first { output.push_str(&"  ...\n".dimmed().to_string()); }
                    first = false;
                    for op in &group {
                        for change in td.iter_changes(op) {
                            let line = change.value().trim_end_matches('\n');
                            match change.tag() {
                                ChangeTag::Equal  => output.push_str(&format!("{}\n", format!("  {line}").dimmed())),
                                ChangeTag::Delete => output.push_str(&format!("{}\n", format!("- {line}").red())),
                                ChangeTag::Insert => output.push_str(&format!("{}\n", format!("+ {line}").green())),
                            }
                        }
                    }
                }
                output.push('\n');
            }
        }
    }

    if multi {
        output.push('\n');
        if changed_count == 0 {
            output.push_str("All components are up to date.");
        } else {
            output.push_str(&format!(
                "{} component{} changed. Run `ui diff <name>` to inspect.",
                changed_count,
                if changed_count == 1 { " has" } else { "s have" }
            ));
        }
    }

    output
}

/// Machine-readable JSON output.
pub fn format_diff_json(diffs: &[ComponentDiff]) -> CliResult<String> {
    let json_diffs: Vec<serde_json::Value> = diffs
        .iter()
        .map(|d| {
            let td = TextDiff::from_lines(&d.local, &d.remote);
            let hunks: Vec<serde_json::Value> = td
                .grouped_ops(0)
                .into_iter()
                .map(|group| {
                    let (mut removed, mut added) = (Vec::new(), Vec::new());
                    for op in &group {
                        for change in td.iter_changes(op) {
                            let line = change.value().trim_end_matches('\n').to_string();
                            match change.tag() {
                                ChangeTag::Delete => removed.push(line),
                                ChangeTag::Insert => added.push(line),
                                ChangeTag::Equal => {}
                            }
                        }
                    }
                    serde_json::json!({ "removed": removed, "added": added })
                })
                .collect();
            let status = serde_json::to_value(&d.status).unwrap_or_default();
            serde_json::json!({ "name": d.name, "status": status, "hunks": hunks })
        })
        .collect();

    serde_json::to_string_pretty(&json_diffs).map_err(Into::into)
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::framework::Framework;

    fn make_diff(name: &str, status: DiffStatus, local: &str, remote: &str) -> ComponentDiff {
        ComponentDiff { name: name.to_string(), status, local: local.to_string(), remote: remote.to_string() }
    }

    // --- format_diff_human ---

    #[test]
    fn up_to_date_single_component_shows_no_diff_block() {
        let diff = make_diff("button", DiffStatus::UpToDate, "fn foo() {}", "fn foo() {}");
        let out = format_diff_human(&[diff]);
        assert!(!out.contains("---"));
        assert!(!out.contains("+++"));
    }

    #[test]
    fn changed_component_shows_diff_headers() {
        let diff = make_diff("button", DiffStatus::Changed, "let x = 1;", "let x = 2;");
        let out = format_diff_human(&[diff]);
        assert!(out.contains("--- button (local)"));
        assert!(out.contains("+++ button (registry)"));
    }

    #[test]
    fn multi_up_to_date_shows_all_up_to_date_message() {
        let diffs = vec![
            make_diff("badge", DiffStatus::UpToDate, "x", "x"),
            make_diff("card", DiffStatus::UpToDate, "x", "x"),
        ];
        let out = format_diff_human(&diffs);
        assert!(out.contains("All components are up to date."));
    }

    #[test]
    fn multi_changed_shows_changed_count() {
        let diffs = vec![
            make_diff("button", DiffStatus::Changed, "old", "new"),
            make_diff("badge", DiffStatus::UpToDate, "x", "x"),
        ];
        let out = format_diff_human(&diffs);
        assert!(out.contains("1 component has changed"));
    }

    #[test]
    fn not_in_registry_shows_question_mark_label() {
        let diffs = vec![make_diff("my_custom", DiffStatus::NotInRegistry, "", "")];
        let out = format_diff_human(&diffs);
        assert!(out.contains("not in registry"));
    }

    // --- format_diff_json ---

    #[test]
    fn json_output_is_valid_array() {
        let diffs = vec![make_diff("button", DiffStatus::UpToDate, "x", "x")];
        let json = format_diff_json(&diffs).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
    }

    #[test]
    fn json_status_serialized_correctly() {
        let diffs = vec![
            make_diff("a", DiffStatus::UpToDate, "x", "x"),
            make_diff("b", DiffStatus::Changed, "old", "new"),
            make_diff("c", DiffStatus::NotInRegistry, "", ""),
        ];
        let json = format_diff_json(&diffs).unwrap();
        assert!(json.contains("up_to_date"));
        assert!(json.contains("changed"));
        assert!(json.contains("not_in_registry"));
    }

    #[test]
    fn json_contains_hunks_for_changed_component() {
        let diffs = vec![make_diff("button", DiffStatus::Changed, "fn foo() {}\nold\n", "fn foo() {}\nnew\n")];
        let json = format_diff_json(&diffs).unwrap();
        assert!(json.contains("hunks"));
        assert!(json.contains("old"));
        assert!(json.contains("new"));
    }

    // --- diff_components ---

    #[tokio::test]
    async fn diff_components_empty_names_returns_empty_vec() {
        let result = diff_components(&[], "any/path", Framework::Leptos).await.unwrap();
        assert!(result.is_empty());
    }
}
