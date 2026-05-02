use clap::{Arg, ArgMatches, Command};
use serde::Serialize;

use crate::command_add::installed::get_installed_components;
use crate::command_init::config::UiConfig;
use crate::command_init::workspace_utils::analyze_workspace;
use crate::shared::cli_error::CliResult;

const UI_CONFIG_TOML: &str = "ui_config.toml";

/* ========================================================== */
/*                        📦 TYPES 📦                         */
/* ========================================================== */

#[derive(Serialize)]
pub struct InfoData {
    pub config_file: String,
    pub base_color: String,
    pub base_path: String,
    pub rtl: bool,
    pub workspace: Option<bool>,
    pub target_crate: Option<String>,
    pub installed: Vec<String>,
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_info() -> Command {
    Command::new("info")
        .about("Show project configuration and installed components")
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output as JSON")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn process_info(matches: &ArgMatches) -> CliResult<()> {
    let json = matches.get_flag("json");

    let config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML)?;
    let installed = get_installed_components(&config.base_path_components);
    let workspace = analyze_workspace().ok();

    let data = build_info_data(&config.base_color, &config.base_path_components, config.rtl, &installed, workspace.as_ref());

    let output = if json { format_info_json(&data)? } else { format_info(&data) };
    println!("{output}");
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

pub fn build_info_data(
    base_color: &str,
    base_path: &str,
    rtl: bool,
    installed: &std::collections::HashSet<String>,
    workspace: Option<&crate::command_init::workspace_utils::WorkspaceInfo>,
) -> InfoData {
    let mut sorted_installed: Vec<String> = installed.iter().cloned().collect();
    sorted_installed.sort();

    let (ws_flag, target_crate) = match workspace {
        Some(ws) => (Some(ws.is_workspace), ws.target_crate.clone()),
        None => (None, None),
    };

    InfoData {
        config_file: UI_CONFIG_TOML.to_string(),
        base_color: base_color.to_string(),
        base_path: base_path.to_string(),
        rtl,
        workspace: ws_flag,
        target_crate,
        installed: sorted_installed,
    }
}

/// Human-readable formatter.
pub fn format_info(data: &InfoData) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("  Config file   {}", data.config_file));
    lines.push(format!("  Base color    {}", data.base_color));
    lines.push(format!("  Base path     {}", data.base_path));
    lines.push(format!("  RTL           {}", if data.rtl { "yes" } else { "no" }));

    if let Some(is_workspace) = data.workspace {
        lines.push(format!("  Workspace     {}", if is_workspace { "yes" } else { "no" }));
    }
    if let Some(ref crate_name) = data.target_crate {
        lines.push(format!("  Target crate  {crate_name}"));
    }

    let count = data.installed.len();
    if count == 0 {
        lines.push("  Installed     none".to_string());
    } else {
        lines.push(format!("  Installed ({count})  {}", data.installed.join(", ")));
    }

    lines.join("\n")
}

/// Machine-readable JSON formatter.
pub fn format_info_json(data: &InfoData) -> CliResult<String> {
    serde_json::to_string_pretty(data).map_err(Into::into)
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::command_init::workspace_utils::WorkspaceInfo;

    fn installed(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn no_workspace() -> Option<WorkspaceInfo> {
        None
    }

    fn single_crate_workspace() -> Option<WorkspaceInfo> {
        Some(WorkspaceInfo {
            is_workspace: false,
            workspace_root: None,
            target_crate: Some("my-app".to_string()),
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        })
    }

    fn full_workspace() -> Option<WorkspaceInfo> {
        Some(WorkspaceInfo {
            is_workspace: true,
            workspace_root: Some(std::path::PathBuf::from("/project")),
            target_crate: Some("frontend".to_string()),
            target_crate_path: None,
            components_base_path: "frontend/src/components".to_string(),
        })
    }

    fn data(color: &str, path: &str, names: &[&str], ws: Option<WorkspaceInfo>) -> InfoData {
        build_info_data(color, path, false, &installed(names), ws.as_ref())
    }

    fn data_rtl(color: &str, path: &str, rtl: bool) -> InfoData {
        build_info_data(color, path, rtl, &installed(&[]), None)
    }

    // --- format_info (human-readable) ---

    #[test]
    fn shows_config_fields() {
        let result = format_info(&data("neutral", "src/components", &[], no_workspace()));
        assert!(result.contains("ui_config.toml"));
        assert!(result.contains("neutral"));
        assert!(result.contains("src/components"));
    }

    #[test]
    fn shows_none_when_no_components_installed() {
        let result = format_info(&data("neutral", "src/components", &[], no_workspace()));
        assert!(result.contains("none"));
    }

    #[test]
    fn shows_installed_components_sorted() {
        let result = format_info(&data("neutral", "src/components", &["card", "button", "badge"], no_workspace()));
        assert!(result.contains("badge, button, card"));
    }

    #[test]
    fn shows_installed_count() {
        let result = format_info(&data("neutral", "src/components", &["button", "badge"], no_workspace()));
        assert!(result.contains("(2)"));
    }

    #[test]
    fn shows_workspace_no_when_single_crate() {
        let result = format_info(&data("neutral", "src/components", &[], single_crate_workspace()));
        assert!(result.contains("no"));
    }

    #[test]
    fn shows_workspace_yes_when_in_workspace() {
        let result = format_info(&data("neutral", "src/components", &[], full_workspace()));
        assert!(result.contains("yes"));
        assert!(result.contains("frontend"));
    }

    #[test]
    fn shows_target_crate_when_available() {
        let result = format_info(&data("neutral", "src/components", &[], single_crate_workspace()));
        assert!(result.contains("my-app"));
    }

    #[test]
    fn no_workspace_info_omits_workspace_line() {
        let result = format_info(&data("neutral", "src/components", &[], no_workspace()));
        assert!(!result.contains("Workspace"));
        assert!(!result.contains("Target crate"));
    }

    #[test]
    fn single_installed_component() {
        let result = format_info(&data("neutral", "src/components", &["button"], no_workspace()));
        assert!(result.contains("(1)"));
        assert!(result.contains("button"));
    }

    // --- format_info_json ---

    #[test]
    fn json_output_is_valid_json() {
        let d = data("neutral", "src/components", &["button"], no_workspace());
        let json = format_info_json(&d).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn json_contains_all_fields() {
        let d = data("neutral", "src/components", &["button"], no_workspace());
        let json = format_info_json(&d).unwrap();
        assert!(json.contains("base_color"));
        assert!(json.contains("base_path"));
        assert!(json.contains("config_file"));
        assert!(json.contains("installed"));
        assert!(json.contains("rtl"));
    }

    #[test]
    fn info_includes_rtl_false() {
        let result = format_info(&data_rtl("neutral", "src/components", false));
        assert!(result.contains("no"));
    }

    #[test]
    fn info_includes_rtl_true() {
        let result = format_info(&data_rtl("neutral", "src/components", true));
        assert!(result.contains("yes"));
    }

    #[test]
    fn info_json_includes_rtl_field() {
        let d = data_rtl("neutral", "src/components", true);
        let json = format_info_json(&d).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["rtl"], true);
    }

    #[test]
    fn json_installed_is_array() {
        let d = data("neutral", "src/components", &["badge", "button"], no_workspace());
        let json = format_info_json(&d).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["installed"].is_array());
        assert_eq!(parsed["installed"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn json_workspace_null_when_no_workspace() {
        let d = data("neutral", "src/components", &[], no_workspace());
        let json = format_info_json(&d).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["workspace"].is_null());
    }

    #[test]
    fn json_workspace_true_when_in_workspace() {
        let d = data("neutral", "src/components", &[], full_workspace());
        let json = format_info_json(&d).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["workspace"], true);
    }

    #[test]
    fn json_installed_sorted() {
        let d = data("neutral", "src/components", &["card", "alert", "badge"], no_workspace());
        let json = format_info_json(&d).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let names: Vec<&str> = parsed["installed"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(names, vec!["alert", "badge", "card"]);
    }
}
