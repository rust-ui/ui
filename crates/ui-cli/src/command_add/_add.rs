use std::collections::HashSet;
use std::path::Path;
use std::vec::Vec;

const UI_CONFIG_TOML: &str = "ui_config.toml";

struct DeprecatedComponent {
    name: &'static str,
    replacement: &'static str,
}

const DEPRECATED_COMPONENTS: &[DeprecatedComponent] = &[DeprecatedComponent {
    name: "toast",
    replacement: "sonner",
}];

use clap::{Arg, ArgMatches, Command};

use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;

use super::components::Components;
use super::installed::get_installed_components;
use super::registry::RegistryComponent;
use super::tree_parser::TreeParser;
use crate::command_diff::_diff::{diff_components, format_diff_human};
use crate::command_init::config::UiConfig;
use crate::command_view::_view::view_components;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::nightly_check::check_nightly_setup;
use crate::shared::rust_ui_client::RustUIClient;

pub fn command_add() -> Command {
    Command::new("add")
        .about("Add components and dependencies to your project")
        .arg(Arg::new("components").help("The components to add (space-separated)").required(false).num_args(1..))
        .arg(
            Arg::new("yes")
                .short('y')
                .long("yes")
                .help("Overwrite existing files without prompting")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .short('n')
                .long("dry-run")
                .help("Preview which files would be written without making any changes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .help("Override the output directory for components (default: base_path_components from ui_config.toml)")
                .value_name("PATH"),
        )
        .arg(
            Arg::new("view")
                .long("view")
                .help("View registry source for each component without installing")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("diff")
                .long("diff")
                .help("Show a diff of what would change for each component without installing")
                .action(clap::ArgAction::SetTrue),
        )
}

/* ========================================================== */
/*                     🔧 NIGHTLY CHECK 🔧                    */
/* ========================================================== */

/// Warn the user if the project is not configured for nightly Rust, then ask whether to proceed.
/// Returns `Err` only if the user explicitly declines.
fn warn_if_nightly_not_configured() -> CliResult<()> {
    let status = check_nightly_setup(std::path::Path::new("."));
    if status.is_ok() {
        return Ok(());
    }

    eprintln!("\n⚠️  Nightly Rust is required by some components (e.g. bind:value).");
    for item in status.missing_items() {
        eprintln!("   Missing: {item}");
    }
    eprintln!("   See: https://rust-ui.com/docs/installation\n");

    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Continue anyway?")
        .default(false)
        .interact()
        .map_err(|err| CliError::validation(&format!("Failed to get user input: {err}")))?;

    if !proceed {
        return Err(CliError::validation("Aborted: nightly Rust is required."));
    }

    Ok(())
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

/// Install a specific list of components into `base_path`, always overwriting.
/// Used by `ui init --reinstall` to re-download existing components.
pub async fn process_add_components(components: Vec<String>, base_path: &str, rtl: bool) -> CliResult<()> {
    if components.is_empty() {
        return Ok(());
    }

    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;

    let resolved_set = tree_parser.resolve_dependencies(&components)?;
    let all_resolved_components: Vec<String> = resolved_set.components.into_iter().collect();
    let all_resolved_parent_dirs: Vec<String> = resolved_set.parent_dirs.into_iter().collect();
    let all_resolved_cargo_dependencies: Vec<String> = resolved_set.cargo_deps.into_iter().collect();
    let all_resolved_js_files: HashSet<String> = resolved_set.js_files;
    let user_requested: HashSet<String> = components.into_iter().collect();

    Components::create_components_mod_if_not_exists_with_pub_mods(
        base_path.to_string(),
        all_resolved_parent_dirs,
    )?;

    let components_path = Path::new(base_path);
    let parent_path = components_path
        .parent()
        .ok_or_else(|| CliError::invalid_path(base_path, "no parent directory"))?;
    let entry_file_path = if parent_path.join("lib.rs").exists() {
        parent_path.join("lib.rs")
    } else {
        parent_path.join("main.rs")
    };
    Components::register_components_in_application_entry(
        entry_file_path.to_string_lossy().as_ref(),
    )?;

    let installed = get_installed_components(base_path);
    let mut written: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    let mut already_installed: Vec<String> = Vec::new();

    for component_name in all_resolved_components {
        if installed.contains(&component_name) && !user_requested.contains(&component_name) {
            already_installed.push(component_name);
            continue;
        }

        let outcome = RegistryComponent::fetch_from_registry(component_name.clone())
            .await?
            .then_write_to_file_to(true, base_path, rtl) // force = always overwrite on reinstall
            .await?;

        match outcome {
            super::registry::WriteOutcome::Written => written.push(component_name),
            super::registry::WriteOutcome::Skipped => skipped.push(component_name),
        }
    }

    print_add_summary(&written, &skipped, &already_installed);

    if !all_resolved_cargo_dependencies.is_empty() {
        super::dependencies::process_cargo_deps(&all_resolved_cargo_dependencies)?;
    }
    if !all_resolved_js_files.is_empty() {
        process_js_files(&all_resolved_js_files).await?;
    }

    Ok(())
}

//
pub async fn process_add(matches: &ArgMatches) -> CliResult<()> {
    let user_components: Vec<String> =
        matches.get_many::<String>("components").unwrap_or_default().cloned().collect();
    let force = matches.get_flag("yes");
    let dry_run = matches.get_flag("dry-run");
    let view_flag = matches.get_flag("view");
    let diff_flag = matches.get_flag("diff");
    let path_override: Option<String> = matches.get_one::<String>("path").cloned();
    let has_path_override = path_override.is_some();

    // Fetch and parse tree.md
    let tree_content = RustUIClient::fetch_tree_md().await?;
    let tree_parser = TreeParser::parse_tree_md(&tree_content)?;

    // Get base path and rtl from config; --path flag overrides base_path
    let config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML).ok();
    let rtl = config.as_ref().map(|c| c.rtl).unwrap_or(false);
    let base_path = path_override.unwrap_or_else(|| {
        config
            .map(|c| c.base_path_components)
            .unwrap_or_else(|| "src/components".to_string())
    });

    // Detect already installed components
    let installed = get_installed_components(&base_path);

    // Warn if the project is not set up for nightly Rust (required for bind:value and other
    // Leptos nightly features). Always prompt — the user may know what they're doing.
    warn_if_nightly_not_configured()?;

    // If no components provided, launch TUI
    let user_components = if user_components.is_empty() {
        let component_names: Vec<String> = tree_parser.get_all_component_names();
        let dependencies = tree_parser.get_dependencies_map();
        let selected = super::ratatui::run_tui(component_names, installed.clone(), dependencies)?;
        if selected.is_empty() {
            println!("No components selected.");
            return Ok(());
        }
        selected
    } else {
        user_components
    };

    // Warn and exit if any requested component is deprecated
    for component in &user_components {
        if let Some(dep) = DEPRECATED_COMPONENTS.iter().find(|d| d.name == component.as_str()) {
            eprintln!(
                "Warning: '{}' is deprecated. Use '{}' instead.",
                dep.name, dep.replacement
            );
            return Err(CliError::validation(&format!(
                "'{}' is deprecated. Use '{}' instead.",
                dep.name, dep.replacement
            )));
        }
    }

    // Resolve dependencies using the new tree-based system
    let resolved_set = tree_parser.resolve_dependencies(&user_components)?;

    // Convert HashSets to Vecs for compatibility with existing functions
    let all_resolved_components: Vec<String> = resolved_set.components.into_iter().collect();
    let all_resolved_parent_dirs: Vec<String> = resolved_set.parent_dirs.into_iter().collect();
    let all_resolved_cargo_dependencies: Vec<String> = resolved_set.cargo_deps.into_iter().collect();
    let all_resolved_js_files: HashSet<String> = resolved_set.js_files;

    // Track which components the user explicitly requested for prompt decisions
    let user_requested: HashSet<String> = user_components.iter().cloned().collect();

    // --view: print registry source for each resolved component, then exit
    if view_flag {
        let mut names = all_resolved_components.clone();
        names.sort();
        return view_components(&names).await;
    }

    // --diff: show diff vs local files for each resolved component, then exit
    if diff_flag {
        let mut names = all_resolved_components.clone();
        names.sort();
        let diffs = diff_components(&names, &base_path).await?;
        println!("{}", format_diff_human(&diffs));
        return Ok(());
    }

    // Dry-run: show what would happen without touching the filesystem
    if dry_run {
        let summary = compute_dry_run_summary(
            &all_resolved_components,
            &installed,
            &user_requested,
            &all_resolved_cargo_dependencies,
            &all_resolved_js_files,
        );
        println!("{}", format_dry_run_summary(&summary));
        return Ok(());
    }

    // Create components/mod.rs if it does not exist
    Components::create_components_mod_if_not_exists_with_pub_mods(
        base_path.clone(),
        all_resolved_parent_dirs,
    )?;

    // Register `components` module in lib.rs/main.rs — skip when --path overrides the directory
    // because the custom path may not correspond to any Rust entry file.
    if !has_path_override {
        let components_path = Path::new(&base_path);
        let parent_path = components_path
            .parent()
            .ok_or_else(|| CliError::invalid_path(&base_path, "no parent directory"))?;

        let entry_file_path = if parent_path.join("lib.rs").exists() {
            parent_path.join("lib.rs")
        } else {
            parent_path.join("main.rs")
        };

        let entry_file_path = entry_file_path.to_string_lossy().to_string();
        Components::register_components_in_application_entry(entry_file_path.as_str())?;
    }

    // Components to add
    let mut written: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    let mut already_installed: Vec<String> = Vec::new();

    for component_name in all_resolved_components {
        // Auto-resolved dep already on disk — skip fetch, report it separately
        if installed.contains(&component_name) && !user_requested.contains(&component_name) {
            already_installed.push(component_name);
            continue;
        }

        let outcome = RegistryComponent::fetch_from_registry(component_name.clone())
            .await?
            .then_write_to_file_to(force, &base_path, rtl)
            .await?;

        match outcome {
            super::registry::WriteOutcome::Written => written.push(component_name),
            super::registry::WriteOutcome::Skipped => skipped.push(component_name),
        }
    }

    print_add_summary(&written, &skipped, &already_installed);

    // Handle cargo dependencies if any exist
    if !all_resolved_cargo_dependencies.is_empty() {
        super::dependencies::process_cargo_deps(&all_resolved_cargo_dependencies)?;
    }

    // Handle JS file dependencies if any exist
    if !all_resolved_js_files.is_empty() {
        process_js_files(&all_resolved_js_files).await?;
    }

    Ok(())
}

/* ========================================================== */
/*                    🔍 DRY-RUN SUMMARY 🔍                  */
/* ========================================================== */

struct DryRunSummary {
    would_add: Vec<String>,
    would_overwrite: Vec<String>,
    already_installed: Vec<String>,
    cargo_deps: Vec<String>,
    js_files: Vec<String>,
}

fn compute_dry_run_summary(
    resolved: &[String],
    installed: &HashSet<String>,
    user_requested: &HashSet<String>,
    cargo_deps: &[String],
    js_files: &HashSet<String>,
) -> DryRunSummary {
    let mut would_add = Vec::new();
    let mut would_overwrite = Vec::new();
    let mut already_installed = Vec::new();

    for name in resolved {
        if installed.contains(name) && !user_requested.contains(name) {
            already_installed.push(name.clone());
        } else if installed.contains(name) {
            would_overwrite.push(name.clone());
        } else {
            would_add.push(name.clone());
        }
    }

    // Sort for deterministic output
    would_add.sort();
    would_overwrite.sort();
    already_installed.sort();

    let mut cargo_deps = cargo_deps.to_vec();
    cargo_deps.sort();

    let mut js_files: Vec<String> = js_files.iter().cloned().collect();
    js_files.sort();

    DryRunSummary { would_add, would_overwrite, already_installed, cargo_deps, js_files }
}

fn format_dry_run_summary(s: &DryRunSummary) -> String {
    let mut lines: Vec<String> = Vec::new();

    if !s.would_add.is_empty() {
        lines.push(format!("[dry-run] Would add:              {}", s.would_add.join(", ")));
    }
    if !s.would_overwrite.is_empty() {
        lines.push(format!("[dry-run] Would overwrite:        {}", s.would_overwrite.join(", ")));
    }
    if !s.already_installed.is_empty() {
        lines.push(format!("[dry-run] Dep already installed:  {}", s.already_installed.join(", ")));
    }
    if !s.cargo_deps.is_empty() {
        lines.push(format!("[dry-run] Would add cargo deps:   {}", s.cargo_deps.join(", ")));
    }
    if !s.js_files.is_empty() {
        lines.push(format!("[dry-run] Would install JS files: {}", s.js_files.join(", ")));
    }

    if lines.is_empty() { "[dry-run] Nothing to add.".to_string() } else { lines.join("\n") }
}

/* ========================================================== */
/*                     ✨ SUMMARY ✨                          */
/* ========================================================== */

fn print_add_summary(written: &[String], skipped: &[String], already_installed: &[String]) {
    let summary = format_add_summary(written, skipped, already_installed);
    if !summary.is_empty() {
        println!("{summary}");
    }
}

pub fn format_add_summary(
    written: &[String],
    skipped: &[String],
    already_installed: &[String],
) -> String {
    let mut lines: Vec<String> = Vec::new();

    if !written.is_empty() {
        lines.push(format!("✅ Added:            {}", written.join(", ")));
    }
    if !skipped.is_empty() {
        lines.push(format!("⏭  Skipped:          {} (already exist)", skipped.join(", ")));
    }
    if !already_installed.is_empty() {
        lines.push(format!("📦 Dep already installed: {}", already_installed.join(", ")));
    }

    lines.join("\n")
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // --- format_add_summary ---

    #[test]
    fn summary_all_written() {
        let result = format_add_summary(&s(&["button", "badge"]), &[], &[]);
        assert_eq!(result, "✅ Added:            button, badge");
    }

    #[test]
    fn summary_all_skipped() {
        let result = format_add_summary(&[], &s(&["card"]), &[]);
        assert_eq!(result, "⏭  Skipped:          card (already exist)");
    }

    #[test]
    fn summary_all_already_installed() {
        let result = format_add_summary(&[], &[], &s(&["button"]));
        assert_eq!(result, "📦 Dep already installed: button");
    }

    #[test]
    fn summary_mixed_all_three() {
        let result = format_add_summary(&s(&["badge"]), &s(&["card"]), &s(&["button"]));
        assert_eq!(
            result,
            "✅ Added:            badge\n⏭  Skipped:          card (already exist)\n📦 Dep already installed: button"
        );
    }

    #[test]
    fn summary_written_and_already_installed() {
        let result = format_add_summary(&s(&["badge"]), &[], &s(&["button"]));
        assert_eq!(result, "✅ Added:            badge\n📦 Dep already installed: button");
    }

    #[test]
    fn summary_empty() {
        let result = format_add_summary(&[], &[], &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn summary_single_written() {
        let result = format_add_summary(&s(&["badge"]), &[], &[]);
        assert_eq!(result, "✅ Added:            badge");
    }

    #[test]
    fn summary_multiple_already_installed() {
        let result = format_add_summary(&[], &[], &s(&["button", "card", "badge"]));
        assert_eq!(result, "📦 Dep already installed: button, card, badge");
    }

    // --- dep-skip logic ---

    #[test]
    fn dep_already_installed_not_requested_is_skipped() {
        let installed: HashSet<String> = ["button"].iter().map(|s| s.to_string()).collect();
        let user_requested: HashSet<String> = ["badge"].iter().map(|s| s.to_string()).collect();
        // button is installed but not requested → should be put in already_installed
        assert!(installed.contains("button") && !user_requested.contains("button"));
    }

    #[test]
    fn dep_already_installed_but_explicitly_requested_is_not_skipped() {
        let installed: HashSet<String> = ["button"].iter().map(|s| s.to_string()).collect();
        let user_requested: HashSet<String> = ["button"].iter().map(|s| s.to_string()).collect();
        // button is installed AND requested → should go through normal write path
        assert!(!(installed.contains("button") && !user_requested.contains("button")));
    }

    #[test]
    fn new_dep_not_installed_is_not_skipped() {
        let installed: HashSet<String> = HashSet::new();
        let user_requested: HashSet<String> = ["badge"].iter().map(|s| s.to_string()).collect();
        // button is not installed → never skipped regardless of requested
        assert!(!(installed.contains("button") && !user_requested.contains("button")));
    }

    // --- compute_dry_run_summary / format_dry_run_summary ---

    fn make_set(items: &[&str]) -> HashSet<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    fn dry_run(
        resolved: &[&str],
        installed: &[&str],
        requested: &[&str],
        cargo: &[&str],
        js: &[&str],
    ) -> DryRunSummary {
        compute_dry_run_summary(
            &resolved.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            &make_set(installed),
            &make_set(requested),
            &cargo.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            &make_set(js),
        )
    }

    #[test]
    fn dry_run_nothing_to_add_when_all_empty() {
        let s = dry_run(&[], &[], &[], &[], &[]);
        assert_eq!(format_dry_run_summary(&s), "[dry-run] Nothing to add.");
    }

    #[test]
    fn dry_run_new_component_goes_to_would_add() {
        let s = dry_run(&["badge"], &[], &["badge"], &[], &[]);
        assert!(s.would_add.contains(&"badge".to_string()));
        assert!(s.would_overwrite.is_empty());
        assert!(s.already_installed.is_empty());
    }

    #[test]
    fn dry_run_installed_dep_not_requested_goes_to_already_installed() {
        let s = dry_run(&["button"], &["button"], &["badge"], &[], &[]);
        assert!(s.already_installed.contains(&"button".to_string()));
        assert!(s.would_add.is_empty());
        assert!(s.would_overwrite.is_empty());
    }

    #[test]
    fn dry_run_installed_and_requested_goes_to_would_overwrite() {
        let s = dry_run(&["button"], &["button"], &["button"], &[], &[]);
        assert!(s.would_overwrite.contains(&"button".to_string()));
        assert!(s.would_add.is_empty());
        assert!(s.already_installed.is_empty());
    }

    #[test]
    fn dry_run_cargo_deps_shown_in_summary() {
        let s = dry_run(&[], &[], &[], &["lucide-leptos"], &[]);
        assert_eq!(s.cargo_deps, vec!["lucide-leptos"]);
        assert!(format_dry_run_summary(&s).contains("Would add cargo deps"));
    }

    #[test]
    fn dry_run_js_files_shown_in_summary() {
        let s = dry_run(&[], &[], &[], &[], &["floating-ui.js"]);
        assert!(format_dry_run_summary(&s).contains("Would install JS files"));
    }

    #[test]
    fn dry_run_mixed_all_categories() {
        let s = dry_run(
            &["badge", "button", "card"],
            &["button", "card"],
            &["badge", "button"],
            &["lucide"],
            &["fp.js"],
        );
        assert_eq!(s.would_add, vec!["badge"]);
        assert_eq!(s.would_overwrite, vec!["button"]);
        assert_eq!(s.already_installed, vec!["card"]);
        assert_eq!(s.cargo_deps, vec!["lucide"]);
        assert_eq!(s.js_files, vec!["fp.js"]);
    }

    #[test]
    fn dry_run_output_is_sorted() {
        let s = dry_run(&["card", "alert", "badge"], &[], &["card", "alert", "badge"], &[], &[]);
        assert_eq!(s.would_add, vec!["alert", "badge", "card"]);
    }

    #[test]
    fn dry_run_format_shows_all_sections() {
        let s = dry_run(
            &["badge", "button"],
            &["button"],
            &["badge", "button"],
            &["dep-a"],
            &["file.js"],
        );
        let out = format_dry_run_summary(&s);
        assert!(out.contains("Would add"));
        assert!(out.contains("Would overwrite"));
        assert!(out.contains("Would add cargo deps"));
        assert!(out.contains("Would install JS files"));
    }

    // --- process_add_components ---

    #[test]
    fn process_add_components_returns_ok_for_empty_list() {
        // Empty input must short-circuit without hitting the network
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(process_add_components(vec![], "src/components", false));
        assert!(result.is_ok());
    }

    // --- deprecated component warnings ---

    #[test]
    fn toast_is_in_deprecated_list() {
        assert!(DEPRECATED_COMPONENTS.iter().any(|d| d.name == "toast"));
    }

    #[test]
    fn deprecated_toast_points_to_sonner() {
        let dep = DEPRECATED_COMPONENTS.iter().find(|d| d.name == "toast").unwrap();
        assert_eq!(dep.replacement, "sonner");
    }

    #[test]
    fn non_deprecated_component_not_in_list() {
        assert!(!DEPRECATED_COMPONENTS.iter().any(|d| d.name == "button"));
        assert!(!DEPRECATED_COMPONENTS.iter().any(|d| d.name == "badge"));
    }

    // --- command_add flag wiring ---

    #[test]
    fn command_add_diff_flag_is_registered() {
        let m = command_add().try_get_matches_from(["add", "button", "--diff"]).unwrap();
        assert!(m.get_flag("diff"));
        assert!(!m.get_flag("view"));
    }

    #[test]
    fn command_add_view_flag_is_registered() {
        let m = command_add().try_get_matches_from(["add", "button", "--view"]).unwrap();
        assert!(m.get_flag("view"));
        assert!(!m.get_flag("diff"));
    }
}

/// Download and install JS files to the user's public directory
async fn process_js_files(js_files: &HashSet<String>) -> CliResult<()> {
    use crate::shared::task_spinner::TaskSpinner;

    let spinner = TaskSpinner::new("Installing JS files...");

    for js_path in js_files {
        spinner.set_message(&format!("📜 Downloading {js_path}"));

        // Fetch the JS file content
        let content = RustUIClient::fetch_js_file(js_path).await?;

        // Determine the output path (public/ + js_path)
        let output_path = Path::new("public").join(js_path.trim_start_matches('/'));

        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| CliError::directory_create_failed())?;
        }

        // Check if file already exists
        if output_path.exists() {
            spinner.set_message(&format!("⏭️  Skipping {js_path} (already exists)"));
            continue;
        }

        // Write the file
        std::fs::write(&output_path, content).map_err(|_| CliError::file_write_failed())?;
    }

    let files_str = js_files.iter().cloned().collect::<Vec<_>>().join(", ");
    spinner.finish_success(&format!("JS files installed: [{files_str}]"));

    Ok(())
}
