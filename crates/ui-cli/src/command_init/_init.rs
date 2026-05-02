use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use clap::{Arg, Command};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Select};

const UI_CONFIG_TOML: &str = "ui_config.toml";
const PACKAGE_JSON: &str = "package.json";

use super::backup::FileBackup;
use super::colors::{AccentColor, BaseColor};
use super::config::{UiConfig, add_init_crates};
use super::install::InstallType;
use super::workspace_utils::{check_leptos_dependency, get_tailwind_input_file};
use crate::command_add::installed::get_installed_components;
use crate::command_init::install::install_dependencies;
use crate::command_init::template::MyTemplate;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::task_spinner::TaskSpinner;

/// Returned by `process_init`. Non-empty `to_reinstall` means the caller
/// should re-download those components (e.g. via `process_add_components`).
pub struct InitOutcome {
    pub to_reinstall: Vec<String>,
    pub base_path: String,
    pub rtl: bool,
}

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_init() -> Command {
    Command::new("init")
        .about("Initialize the project")
        .arg(Arg::new("project_name").help("The name of the project to initialize").required(false))
        .arg(
            Arg::new("yes")
                .short('y')
                .long("yes")
                .help("Skip confirmation prompts and accept defaults")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force overwrite existing files without prompting")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("reinstall")
                .long("reinstall")
                .help("Re-download and overwrite all already-installed components")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("rtl")
                .long("rtl")
                .help("Enable RTL support (physical CSS classes → logical equivalents on ui add)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-rtl")
                .long("no-rtl")
                .help("Disable RTL support")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(Command::new("run").about("Run the initialization logic"))
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Run project initialisation.
///
/// - `force`     – overwrite existing files without prompting (`--yes` / `--force`)
/// - `reinstall` – `Some(true)` = always reinstall components, `Some(false)` = never,
///   `None` = prompt when existing components are detected
pub async fn process_init(force: bool, reinstall: Option<bool>, rtl: Option<bool>) -> CliResult<InitOutcome> {
    // Check if Leptos is installed before proceeding
    if !check_leptos_dependency()? {
        return Err(CliError::config(
            "Leptos dependency not found in Cargo.toml. Please install Leptos first.",
        ));
    }

    // Get tailwind input file from Cargo.toml metadata
    let tailwind_input_file = get_tailwind_input_file()?;

    // Read the existing config (if any) so we can detect installed components
    // and derive the base_path *before* we overwrite ui_config.toml.
    let existing_config = UiConfig::try_reading_ui_config(UI_CONFIG_TOML).ok();
    let base_path = existing_config
        .as_ref()
        .map(|c| c.base_path_components.clone())
        .unwrap_or_else(|| "src/components".to_string());

    // Detect components installed in the current project (empty on first run)
    let installed: Vec<String> = get_installed_components(&base_path).into_iter().collect();

    // Prompt for base + accent colors (or use defaults when --yes/--force)
    let (base_color, accent_color) = if force {
        (BaseColor::default(), AccentColor::default())
    } else {
        (prompt_base_color()?, prompt_accent_color()?)
    };

    // Resolve RTL: explicit flag wins, --yes defaults to false, otherwise prompt
    let rtl_enabled = match rtl {
        Some(v) => v,
        None if force => false,
        None => prompt_rtl()?,
    };

    // Back up ui_config.toml — restored automatically on Drop if we error out
    let mut config_backup = FileBackup::new(Path::new(UI_CONFIG_TOML))
        .map_err(|e| CliError::file_operation(&e.to_string()))?;

    let ui_config = UiConfig {
        base_color: base_color.label().to_lowercase(),
        color_theme: accent_color.label().to_lowercase(),
        rtl: rtl_enabled,
        ..UiConfig::default()
    };
    let ui_config_toml = toml::to_string_pretty(&ui_config)?;

    // ui_config.toml - always write (config file)
    write_template_file(UI_CONFIG_TOML, &ui_config_toml).await?;

    // package.json - merge with existing to preserve user dependencies
    merge_package_json(PACKAGE_JSON, MyTemplate::PACKAGE_JSON).await?;

    // tailwind.css - ask before overwriting if exists (skipped when --yes or --force)
    let css = MyTemplate::build_css(base_color, accent_color);
    write_template_with_confirmation(&tailwind_input_file, &css, force).await?;

    add_init_crates().await?;

    install_dependencies(&[InstallType::Tailwind]).await?;

    // All writes succeeded — disarm the backup
    if let Some(ref mut backup) = config_backup {
        backup.disarm();
    }

    // Determine which components to reinstall
    let to_reinstall = if installed.is_empty() {
        vec![]
    } else {
        let should_reinstall = match reinstall {
            Some(v) => v,
            None if force => true,
            None => {
                Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!(
                        "{} existing component(s) found. Re-install them?",
                        installed.len()
                    ))
                    .default(false)
                    .interact()
                    .map_err(|e| CliError::validation(&e.to_string()))?
            }
        };
        if should_reinstall { installed } else { vec![] }
    };

    Ok(InitOutcome { to_reinstall, base_path, rtl: rtl_enabled })
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn prompt_base_color() -> CliResult<BaseColor> {
    let labels = BaseColor::all_labels();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Base color")
        .default(0)
        .items(&labels)
        .interact()
        .map_err(|e| CliError::validation(&e.to_string()))?;
    Ok(BaseColor::from_index(selection))
}

fn prompt_rtl() -> CliResult<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable RTL support?")
        .default(false)
        .interact()
        .map_err(|e| CliError::validation(&e.to_string()))
}

fn prompt_accent_color() -> CliResult<AccentColor> {
    let labels = AccentColor::all_labels();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Accent color")
        .default(0)
        .items(&labels)
        .interact()
        .map_err(|e| CliError::validation(&e.to_string()))?;
    Ok(AccentColor::from_index(selection))
}

/// Write template file (always writes, no confirmation)
async fn write_template_file(file_name: &str, template: &str) -> CliResult<()> {
    let file_path = Path::new(".").join(file_name);
    let spinner = TaskSpinner::new(&format!("Writing {file_name}..."));

    write_file_content(&file_path, template)?;

    spinner.finish_success(&format!("{file_name} written."));
    Ok(())
}

/// Merge package.json with existing file to preserve user dependencies
async fn merge_package_json(file_name: &str, template: &str) -> CliResult<()> {
    let file_path = Path::new(".").join(file_name);
    let file_exists = file_path.exists();
    let spinner = TaskSpinner::new(&format!("Writing {file_name}..."));

    let content = if file_exists {
        let existing_content = fs::read_to_string(&file_path)?;
        merge_json_objects(&existing_content, template)?
    } else {
        template.to_string()
    };

    write_file_content(&file_path, &content)?;

    let action = if file_exists { "merged" } else { "written" };
    spinner.finish_success(&format!("{file_name} {action}."));
    Ok(())
}

/// Write template file with confirmation if file already exists.
/// When `force` is true, overwrites without prompting.
async fn write_template_with_confirmation(
    file_name: &str,
    template: &str,
    force: bool,
) -> CliResult<()> {
    let file_path = Path::new(".").join(file_name);

    if file_path.exists() && !force {
        let should_overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{file_name} already exists. Overwrite?"))
            .default(false)
            .interact()
            .map_err(|err| CliError::validation(&format!("Failed to get user input: {err}")))?;

        if !should_overwrite {
            println!("⏭️  Skipping {file_name}");
            return Ok(());
        }
    }

    let spinner = TaskSpinner::new(&format!("Writing {file_name}..."));
    write_file_content(&file_path, template)?;
    spinner.finish_success(&format!("{file_name} written."));
    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Write content to a file, creating parent directories if needed
fn write_file_content(file_path: &Path, content: &str) -> io::Result<()> {
    // Create the directory if it doesn't exist
    if let Some(dir) = file_path.parent() {
        fs::create_dir_all(dir)?;
    }

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Merge JSON objects: template values are added to existing, preserving existing fields
fn merge_json_objects(existing: &str, template: &str) -> CliResult<String> {
    let mut existing_json: serde_json::Value = serde_json::from_str(existing)
        .map_err(|err| CliError::file_operation(&format!("Failed to parse existing JSON: {err}")))?;

    let template_json: serde_json::Value = serde_json::from_str(template)
        .map_err(|err| CliError::file_operation(&format!("Failed to parse template JSON: {err}")))?;

    if let (Some(existing_obj), Some(template_obj)) =
        (existing_json.as_object_mut(), template_json.as_object())
    {
        for (key, value) in template_obj {
            existing_obj.insert(key.clone(), value.clone());
        }
    }

    serde_json::to_string_pretty(&existing_json)
        .map_err(|err| CliError::file_operation(&format!("Failed to serialize JSON: {err}")))
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    // --- command_init flags ---

    #[test]
    fn command_init_yes_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "--yes"]).unwrap();
        assert!(m.get_flag("yes"));
        assert!(!m.get_flag("force"));
    }

    #[test]
    fn command_init_yes_short_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "-y"]).unwrap();
        assert!(m.get_flag("yes"));
    }

    #[test]
    fn command_init_force_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "--force"]).unwrap();
        assert!(m.get_flag("force"));
        assert!(!m.get_flag("yes"));
    }

    #[test]
    fn command_init_force_short_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "-f"]).unwrap();
        assert!(m.get_flag("force"));
    }

    #[test]
    fn command_init_both_flags_can_be_combined() {
        let m = command_init().try_get_matches_from(["init", "--yes", "--force"]).unwrap();
        assert!(m.get_flag("yes"));
        assert!(m.get_flag("force"));
    }

    #[test]
    fn command_init_reinstall_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "--reinstall"]).unwrap();
        assert!(m.get_flag("reinstall"));
    }

    #[test]
    fn command_init_reinstall_is_false_by_default() {
        let m = command_init().try_get_matches_from(["init"]).unwrap();
        assert!(!m.get_flag("reinstall"));
    }

    #[test]
    fn command_init_rtl_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "--rtl"]).unwrap();
        assert!(m.get_flag("rtl"));
        assert!(!m.get_flag("no-rtl"));
    }

    #[test]
    fn command_init_no_rtl_flag_is_registered() {
        let m = command_init().try_get_matches_from(["init", "--no-rtl"]).unwrap();
        assert!(m.get_flag("no-rtl"));
        assert!(!m.get_flag("rtl"));
    }

    #[test]
    fn command_init_rtl_defaults_false_with_yes_flag() {
        // --yes means force=true → rtl defaults to false without prompting
        let m = command_init().try_get_matches_from(["init", "--yes"]).unwrap();
        assert!(!m.get_flag("rtl"));
        assert!(!m.get_flag("no-rtl"));
    }

    #[test]
    fn test_merge_json_preserves_existing_dependencies() {
        let existing = r#"{
  "name": "my-app",
  "dependencies": {
    "axios": "^1.0.0",
    "react": "^18.0.0"
  }
}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Template field added
        assert_eq!(parsed["type"], "module");
        // Existing fields preserved
        assert_eq!(parsed["name"], "my-app");
        assert_eq!(parsed["dependencies"]["axios"], "^1.0.0");
        assert_eq!(parsed["dependencies"]["react"], "^18.0.0");
    }

    #[test]
    fn test_merge_json_template_takes_precedence() {
        let existing = r#"{"type": "commonjs", "name": "app"}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Template value overwrites existing
        assert_eq!(parsed["type"], "module");
        // Other existing fields preserved
        assert_eq!(parsed["name"], "app");
    }

    #[test]
    fn test_merge_json_empty_existing() {
        let existing = r#"{}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["type"], "module");
    }

    #[test]
    fn test_merge_json_complex_existing() {
        let existing = r#"{
  "name": "my-leptos-app",
  "private": true,
  "scripts": {
    "dev": "trunk serve"
  },
  "devDependencies": {
    "tailwindcss": "^4.0.0",
    "tw-animate-css": "^1.0.0"
  }
}"#;
        let template = r#"{"type": "module"}"#;

        let result = merge_json_objects(existing, template).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Template field added
        assert_eq!(parsed["type"], "module");
        // All existing fields preserved
        assert_eq!(parsed["name"], "my-leptos-app");
        assert_eq!(parsed["private"], true);
        assert_eq!(parsed["scripts"]["dev"], "trunk serve");
        assert_eq!(parsed["devDependencies"]["tailwindcss"], "^4.0.0");
    }

    #[test]
    fn test_write_file_content_creates_directories() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("nested").join("dir").join("file.txt");

        write_file_content(&file_path, "test content").unwrap();

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_write_file_content_overwrites() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("file.txt");

        write_file_content(&file_path, "first").unwrap();
        write_file_content(&file_path, "second").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "second");
    }
}
