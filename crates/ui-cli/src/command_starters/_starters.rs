use std::process::{Command as ProcessCommand, Stdio};

use clap::Command;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::shared::cli_error::{CliError, CliResult};

// TODO. Use cargo-generate later for more customization.

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_starters() -> Command {
    Command::new("starters").about("Choose and install starter templates")
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Display, EnumString, EnumIter)]
#[strum(serialize_all = "kebab-case")]
enum StarterTemplate {
    Tauri,
    TauriFullstack,
}

pub async fn process_starters() -> CliResult<()> {
    let templates: Vec<StarterTemplate> = StarterTemplate::iter().collect();
    let template_names: Vec<String> = templates.iter().map(|t| t.to_string()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a starter template")
        .items(&template_names)
        .default(0)
        .interact()
        .map_err(|_| CliError::validation("Failed to get user selection"))?;

    let selected_template =
        templates.get(selection).ok_or_else(|| CliError::validation("Invalid selection"))?;
    clone_starter_template(selected_template)?;

    Ok(())
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Helper function to clone a starter template repository
fn clone_starter_template(template: &StarterTemplate) -> CliResult<()> {
    let template_name = template.to_string();
    println!("Installing {template_name} starter...");

    let output = ProcessCommand::new("git")
        .arg("clone")
        .arg(format!("https://github.com/rust-ui/start-{template_name}.git"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| CliError::git_clone_failed())?;

    if output.status.success() {
        println!("✅ Successfully cloned {template_name} starter template");
    } else {
        return Err(CliError::git_clone_failed());
    }

    Ok(())
}
