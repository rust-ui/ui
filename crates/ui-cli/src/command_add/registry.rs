use std::io::Write;
use std::path::Path;

use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;

use super::component_type::ComponentType;
use super::rtl::apply_rtl_transforms;
use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::rust_ui_client::RustUIClient;

/* ========================================================== */
/*                        📦 TYPES 📦                         */
/* ========================================================== */

#[derive(Debug, PartialEq)]
pub enum WriteOutcome {
    Written,
    Skipped,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub struct RegistryComponent {
    pub registry_md_path: String,
    pub registry_md_content: String,
    pub component_name: String,
}

impl RegistryComponent {
    pub async fn fetch_from_registry(component_name: String) -> CliResult<RegistryComponent> {
        let registry_md_content = RustUIClient::fetch_styles_default(&component_name).await?;
        let component_type = ComponentType::from_component_name(&component_name);
        let registry_md_path = format!("{}/{}.rs", component_type.to_path(), component_name);

        Ok(RegistryComponent { registry_md_path, registry_md_content, component_name })
    }

    pub async fn then_write_to_file_to(self, force: bool, base_path: &str, rtl: bool) -> CliResult<WriteOutcome> {
        let RegistryComponent { registry_md_path, registry_md_content, component_name } = self;
        let components_base_path = base_path.to_string();
        let full_path_component = std::path::Path::new(&components_base_path).join(&registry_md_path);

        let full_path_component_without_name_rs = full_path_component
            .parent()
            .ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?
            .to_str()
            .ok_or_else(|| CliError::file_operation("Failed to convert path to string"))?
            .to_string();

        let content = if rtl {
            apply_rtl_transforms(&registry_md_content)
        } else {
            registry_md_content
        };
        let outcome = write_component_file(&full_path_component, &content, force)?;

        if outcome == WriteOutcome::Skipped {
            return Ok(WriteOutcome::Skipped);
        }

        write_component_name_in_mod_rs_if_not_exists(
            component_name,
            full_path_component_without_name_rs,
        )?;

        Ok(WriteOutcome::Written)
    }
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Write a component file to disk. If the file already exists and `force` is
/// false, prompt the user. Returns whether the file was written or skipped.
pub fn write_component_file(path: &Path, content: &str, force: bool) -> CliResult<WriteOutcome> {
    if path.exists() && !force {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{file_name} already exists. Overwrite?"))
            .default(false)
            .interact()
            .map_err(|err| CliError::validation(&format!("Failed to get user input: {err}")))?;

        if !overwrite {
            return Ok(WriteOutcome::Skipped);
        }
    }

    let dir = path.parent().ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
    std::fs::create_dir_all(dir).map_err(|_| CliError::directory_create_failed())?;
    std::fs::write(path, content).map_err(|_| CliError::file_write_failed())?;

    Ok(WriteOutcome::Written)
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    fn temp_file(dir: &TempDir, name: &str) -> std::path::PathBuf {
        dir.path().join(name)
    }

    #[test]
    fn write_new_file_returns_written() {
        let dir = TempDir::new().unwrap();
        let path = temp_file(&dir, "button.rs");

        let outcome = write_component_file(&path, "// button", true).unwrap();

        assert_eq!(outcome, WriteOutcome::Written);
        assert!(path.exists());
    }

    #[test]
    fn written_content_is_correct() {
        let dir = TempDir::new().unwrap();
        let path = temp_file(&dir, "button.rs");

        write_component_file(&path, "// button content", true).unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "// button content");
    }

    #[test]
    fn force_true_overwrites_existing_file() {
        let dir = TempDir::new().unwrap();
        let path = temp_file(&dir, "button.rs");
        fs::write(&path, "// old").unwrap();

        let outcome = write_component_file(&path, "// new", true).unwrap();

        assert_eq!(outcome, WriteOutcome::Written);
        assert_eq!(fs::read_to_string(&path).unwrap(), "// new");
    }

    #[test]
    fn write_creates_nested_parent_dirs() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("ui").join("nested").join("button.rs");

        let outcome = write_component_file(&path, "// button", true).unwrap();

        assert_eq!(outcome, WriteOutcome::Written);
        assert!(path.exists());
    }

    #[test]
    fn rtl_false_writes_content_unchanged() {
        let dir = TempDir::new().unwrap();
        let path = temp_file(&dir, "button.rs");

        write_component_file(&path, "class=\"ml-4\"", false).unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "class=\"ml-4\"");
    }

    #[test]
    fn rtl_true_applies_transforms_before_write() {
        use super::super::rtl::apply_rtl_transforms;
        let content = "class=\"ml-4 pl-2\"";
        let transformed = apply_rtl_transforms(content);
        assert!(transformed.contains("ms-4"));
        assert!(transformed.contains("ps-2"));
    }

    #[test]
    fn new_file_with_force_false_returns_written() {
        // force=false on a non-existing file: no prompt needed, just writes
        let dir = TempDir::new().unwrap();
        let path = temp_file(&dir, "badge.rs");

        let outcome = write_component_file(&path, "// badge", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Written);
        assert!(path.exists());
    }

    // --- write_component_name_in_mod_rs_if_not_exists ---

    #[test]
    fn creates_mod_rs_with_pub_mod_entry() {
        let dir = TempDir::new().unwrap();
        let subdir = dir.path().join("ui");
        fs::create_dir_all(&subdir).unwrap();

        write_component_name_in_mod_rs_if_not_exists(
            "button".to_string(),
            subdir.to_str().unwrap().to_string(),
        )
        .unwrap();

        let mod_rs = fs::read_to_string(subdir.join("mod.rs")).unwrap();
        assert!(mod_rs.contains("pub mod button;"));
    }

    #[test]
    fn skips_if_component_already_in_mod_rs() {
        let dir = TempDir::new().unwrap();
        let subdir = dir.path().join("ui");
        fs::create_dir_all(&subdir).unwrap();
        fs::write(subdir.join("mod.rs"), "pub mod button;\n").unwrap();

        write_component_name_in_mod_rs_if_not_exists(
            "button".to_string(),
            subdir.to_str().unwrap().to_string(),
        )
        .unwrap();

        let mod_rs = fs::read_to_string(subdir.join("mod.rs")).unwrap();
        assert_eq!(mod_rs.matches("pub mod button;").count(), 1);
    }

    #[test]
    fn appends_new_component_to_existing_mod_rs() {
        let dir = TempDir::new().unwrap();
        let subdir = dir.path().join("ui");
        fs::create_dir_all(&subdir).unwrap();
        fs::write(subdir.join("mod.rs"), "pub mod button;\n").unwrap();

        write_component_name_in_mod_rs_if_not_exists(
            "badge".to_string(),
            subdir.to_str().unwrap().to_string(),
        )
        .unwrap();

        let mod_rs = fs::read_to_string(subdir.join("mod.rs")).unwrap();
        assert!(mod_rs.contains("pub mod button;"));
        assert!(mod_rs.contains("pub mod badge;"));
    }
}

fn write_component_name_in_mod_rs_if_not_exists(
    component_name: String,
    full_path_component_without_name_rs: String,
) -> CliResult<()> {
    let mod_rs_path = std::path::Path::new(&full_path_component_without_name_rs).join("mod.rs");

    // Create the directory if it doesn't exist
    let dir =
        mod_rs_path.parent().ok_or_else(|| CliError::file_operation("Failed to get parent directory"))?;
    std::fs::create_dir_all(dir).map_err(|_| CliError::directory_create_failed())?;

    // Check if the mod.rs file already exists
    let mut mod_rs_content = String::new();
    if mod_rs_path.exists() {
        mod_rs_content = std::fs::read_to_string(&mod_rs_path).map_err(|_| CliError::file_read_failed())?;
    }

    // Check if the component already exists
    if mod_rs_content.contains(&component_name) {
        return Ok(());
    }

    // Append the component name to mod.rs
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mod_rs_path)
        .map_err(|_| CliError::file_operation("Failed to open mod.rs file"))?;

    // Write the new component name
    writeln!(mod_rs_file, "pub mod {component_name};").map_err(|_| CliError::file_write_failed())?;
    Ok(())
}
