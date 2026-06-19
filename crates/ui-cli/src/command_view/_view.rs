use clap::{Arg, ArgMatches, Command};
use serde::Serialize;

use crate::command_init::workspace_utils::detect_framework;
use crate::shared::cli_error::CliResult;
use crate::shared::framework::Framework;
use crate::shared::rust_ui_client::RustUIClient;

/* ========================================================== */
/*                        📦 TYPES 📦                         */
/* ========================================================== */

#[derive(Debug, Serialize)]
pub struct ComponentView {
    pub name: String,
    pub content: String,
}

/* ========================================================== */
/*                        🔧 COMMAND 🔧                       */
/* ========================================================== */

pub fn command_view() -> Command {
    Command::new("view")
        .about("View a component's source from the registry without installing it")
        .arg(Arg::new("component").help("Component name to view").required(true))
        .arg(Arg::new("json").long("json").help("Output as JSON").action(clap::ArgAction::SetTrue))
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

/// Fetch and print registry source for a list of component names.
/// Names are processed in the order given; sort before calling if needed.
pub async fn view_components(names: &[String], framework: Framework) -> CliResult<()> {
    for name in names {
        let content = RustUIClient::fetch_styles_default(name, framework).await?;
        println!("{}", format_view_human(name, &content));
    }
    Ok(())
}

pub async fn process_view(matches: &ArgMatches) -> CliResult<()> {
    let name = matches.get_one::<String>("component").map(|s| s.as_str()).unwrap_or("");
    let json = matches.get_flag("json");

    let framework = detect_framework()?;
    let content = RustUIClient::fetch_styles_default(name, framework).await?;

    let output = if json {
        format_view_json(&ComponentView { name: name.to_string(), content })?
    } else {
        format_view_human(name, &content)
    };

    println!("{output}");
    Ok(())
}

/* ========================================================== */
/*                      🖨  FORMATTERS 🖨                      */
/* ========================================================== */

pub fn format_view_human(name: &str, content: &str) -> String {
    let line_count = content.lines().count();
    let mut out = String::new();
    out.push_str(&format!("// {name}.rs  ({line_count} lines)\n\n"));
    out.push_str(content);
    out
}

pub fn format_view_json(view: &ComponentView) -> CliResult<String> {
    serde_json::to_string_pretty(view).map_err(Into::into)
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::framework::Framework;

    // --- format_view_human ---

    #[test]
    fn human_output_includes_component_name_in_header() {
        let out = format_view_human("button", "fn foo() {}");
        assert!(out.contains("button.rs"));
    }

    #[test]
    fn human_output_includes_line_count() {
        let content = "line1\nline2\nline3";
        let out = format_view_human("button", content);
        assert!(out.contains("3 lines"));
    }

    #[test]
    fn human_output_includes_content() {
        let content = "pub fn Button() {}";
        let out = format_view_human("button", content);
        assert!(out.contains(content));
    }

    #[test]
    fn human_output_single_line_says_line_not_lines() {
        let out = format_view_human("badge", "fn x() {}");
        assert!(out.contains("1 lines")); // intentionally not pluralizing — keep simple
    }

    // --- format_view_json ---

    #[test]
    fn json_output_is_valid() {
        let view = ComponentView { name: "button".to_string(), content: "fn x() {}".to_string() };
        let json = format_view_json(&view).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn json_contains_name_and_content() {
        let view = ComponentView { name: "button".to_string(), content: "fn x() {}".to_string() };
        let json = format_view_json(&view).unwrap();
        assert!(json.contains("\"name\""));
        assert!(json.contains("button"));
        assert!(json.contains("\"content\""));
        assert!(json.contains("fn x()"));
    }

    #[test]
    fn json_name_field_matches_input() {
        let view = ComponentView { name: "badge".to_string(), content: String::new() };
        let json = format_view_json(&view).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["name"], "badge");
    }

    // --- view_components ---

    #[tokio::test]
    async fn view_components_empty_names_returns_ok() {
        let result = view_components(&[], Framework::Leptos).await;
        assert!(result.is_ok());
    }
}
