use std::path::{Path, PathBuf};

use cargo_toml::{Dependency, Manifest};

use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::framework::Framework;

/// Information about the workspace and target crate
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceInfo {
    /// Whether we're in a workspace
    pub is_workspace: bool,
    /// The workspace root directory (if in a workspace)
    pub workspace_root: Option<PathBuf>,
    /// The target crate name where components should be installed
    pub target_crate: Option<String>,
    /// The path to the target crate directory
    pub target_crate_path: Option<PathBuf>,
    /// The base path for components relative to current working directory
    pub components_base_path: String,
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            is_workspace: false,
            workspace_root: None,
            target_crate: None,
            target_crate_path: None,
            components_base_path: "src/components".to_string(),
        }
    }
}

/// Analyzes the current directory to detect workspace structure and find the appropriate
/// crate for installing components.
pub fn analyze_workspace() -> CliResult<WorkspaceInfo> {
    let current_dir = std::env::current_dir()?;
    analyze_workspace_from_path(&current_dir)
}

/// Analyzes workspace from a specific path (useful for testing)
pub fn analyze_workspace_from_path(start_path: &Path) -> CliResult<WorkspaceInfo> {
    // First, check if we're in a workspace member directory
    let local_cargo_toml = start_path.join("Cargo.toml");

    if !local_cargo_toml.exists() {
        return Err(CliError::file_operation("Cargo.toml not found in current directory"));
    }

    let local_manifest = load_cargo_manifest(&local_cargo_toml)?
        .ok_or_else(|| CliError::file_operation("Failed to parse Cargo.toml"))?;

    // Check if this is a workspace root
    if local_manifest.workspace.is_some() {
        return analyze_from_workspace_root(start_path, &local_manifest);
    }

    // Check if we're in a workspace member by looking for workspace root
    if let Some(workspace_root) = find_workspace_root(start_path)? {
        return analyze_from_workspace_member(start_path, &workspace_root);
    }

    // Not in a workspace - simple single-crate project
    if detect_framework_in_manifest(&local_manifest).is_none() {
        return Err(CliError::config("No supported framework (leptos/dioxus) found in Cargo.toml"));
    }

    Ok(WorkspaceInfo {
        is_workspace: false,
        workspace_root: None,
        target_crate: local_manifest.package.as_ref().map(|p| p.name.clone()),
        target_crate_path: Some(start_path.to_path_buf()),
        components_base_path: "src/components".to_string(),
    })
}

/// Analyze when running from workspace root
fn analyze_from_workspace_root(workspace_root: &Path, manifest: &Manifest) -> CliResult<WorkspaceInfo> {
    let workspace =
        manifest.workspace.as_ref().ok_or_else(|| CliError::config("Expected workspace manifest"))?;

    // If [workspace] has no members but manifest has [package], it's a single-crate
    // using [workspace] only to prevent parent workspace lookup.
    if workspace.members.is_empty() && manifest.package.is_some() {
        if detect_framework_in_manifest(manifest).is_none() {
            return Err(CliError::config("No supported framework (leptos/dioxus) found in Cargo.toml"));
        }
        let crate_name = manifest.package.as_ref().map(|p| p.name.clone());
        return Ok(WorkspaceInfo {
            is_workspace: false,
            workspace_root: None,
            target_crate: crate_name,
            target_crate_path: Some(workspace_root.to_path_buf()),
            components_base_path: "src/components".to_string(),
        });
    }

    // Find workspace member with a supported framework
    let members = expand_workspace_members(workspace_root, &workspace.members)?;

    for member_path in &members {
        let member_cargo_toml = member_path.join("Cargo.toml");
        if let Some(member_manifest) = load_cargo_manifest(&member_cargo_toml)?
            && detect_framework_in_manifest(&member_manifest).is_some()
        {
            let crate_name = member_manifest
                .package
                .as_ref()
                .map(|p| p.name.clone())
                .or_else(|| member_path.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_default();

            let relative_path = member_path.strip_prefix(workspace_root).unwrap_or(member_path);

            return Ok(WorkspaceInfo {
                is_workspace: true,
                workspace_root: Some(workspace_root.to_path_buf()),
                target_crate: Some(crate_name),
                target_crate_path: Some(member_path.clone()),
                components_base_path: format!("{}/src/components", relative_path.display()),
            });
        }
    }

    // Check workspace.dependencies for supported frameworks
    if detect_framework_in_workspace(workspace).is_some() {
        for member_path in &members {
            let member_cargo_toml = member_path.join("Cargo.toml");
            if let Some(member_manifest) = load_cargo_manifest(&member_cargo_toml)?
                && inherited_supported_framework(&member_manifest).is_some()
            {
                let crate_name = member_manifest
                    .package
                    .as_ref()
                    .map(|p| p.name.clone())
                    .or_else(|| member_path.file_name().map(|n| n.to_string_lossy().to_string()))
                    .unwrap_or_default();

                let relative_path = member_path.strip_prefix(workspace_root).unwrap_or(member_path);

                return Ok(WorkspaceInfo {
                    is_workspace: true,
                    workspace_root: Some(workspace_root.to_path_buf()),
                    target_crate: Some(crate_name),
                    target_crate_path: Some(member_path.clone()),
                    components_base_path: format!("{}/src/components", relative_path.display()),
                });
            }
        }
    }

    Err(CliError::config(
        "No workspace member with a supported framework dependency found. Please run from a crate directory with leptos or dioxus installed.",
    ))
}

/// Analyze when running from a workspace member directory
fn analyze_from_workspace_member(member_path: &Path, workspace_root: &Path) -> CliResult<WorkspaceInfo> {
    let member_cargo_toml = member_path.join("Cargo.toml");
    let member_manifest = load_cargo_manifest(&member_cargo_toml)?
        .ok_or_else(|| CliError::file_operation("Failed to parse member Cargo.toml"))?;

    let has_supported_framework = detect_framework_in_manifest(&member_manifest).is_some();

    // Also check workspace.dependencies
    let workspace_cargo_toml = workspace_root.join("Cargo.toml");
    let workspace_has_supported_framework = if let Some(ws_manifest) = load_cargo_manifest(&workspace_cargo_toml)? {
        ws_manifest
            .workspace
            .as_ref()
            .and_then(detect_framework_in_workspace)
            .is_some()
    } else {
        false
    };

    if !has_supported_framework && !workspace_has_supported_framework {
        return Err(CliError::config("No supported framework (leptos/dioxus) found in this crate or workspace"));
    }

    let crate_name = member_manifest
        .package
        .as_ref()
        .map(|p| p.name.clone())
        .or_else(|| member_path.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_default();

    Ok(WorkspaceInfo {
        is_workspace: true,
        workspace_root: Some(workspace_root.to_path_buf()),
        target_crate: Some(crate_name),
        target_crate_path: Some(member_path.to_path_buf()),
        // When running from member, components go in local src/components
        components_base_path: "src/components".to_string(),
    })
}

/// Find workspace root by walking up the directory tree
fn find_workspace_root(start_path: &Path) -> CliResult<Option<PathBuf>> {
    let mut current = start_path.parent();

    while let Some(dir) = current {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists()
            && let Some(manifest) = load_cargo_manifest(&cargo_toml)?
            && manifest.workspace.is_some()
        {
            return Ok(Some(dir.to_path_buf()));
        }
        current = dir.parent();
    }

    Ok(None)
}

/// Expand workspace member patterns (handles globs like "crates/*")
fn expand_workspace_members(workspace_root: &Path, members: &[String]) -> CliResult<Vec<PathBuf>> {
    let mut result = Vec::new();

    for member in members {
        if member.contains('*') {
            // Handle glob pattern
            let pattern = workspace_root.join(member);
            let pattern_str = pattern.to_string_lossy();

            if let Ok(paths) = glob::glob(&pattern_str) {
                for path in paths.flatten() {
                    if path.is_dir() && path.join("Cargo.toml").exists() {
                        result.push(path);
                    }
                }
            }
        } else {
            let member_path = workspace_root.join(member);
            if member_path.is_dir() && member_path.join("Cargo.toml").exists() {
                result.push(member_path);
            }
        }
    }

    Ok(result)
}

pub fn detect_framework() -> CliResult<Framework> {
    let current_dir = std::env::current_dir()?;
    detect_framework_from_path(&current_dir)
}

pub fn detect_framework_from_path(start_path: &Path) -> CliResult<Framework> {
    let local_cargo_toml = start_path.join("Cargo.toml");
    if let Some(local_manifest) = load_cargo_manifest(&local_cargo_toml)?
        && let Some(framework) = detect_framework_in_manifest(&local_manifest)
    {
        return Ok(framework);
    }

    if let Some(workspace_root) = find_workspace_root(start_path)? {
        let workspace_cargo_toml = workspace_root.join("Cargo.toml");
        if let Some(workspace_manifest) = load_cargo_manifest(&workspace_cargo_toml)? {
            if let Some(framework) = workspace_manifest
                .workspace
                .as_ref()
                .and_then(detect_framework_in_workspace)
            {
                return Ok(framework);
            }

            let members = workspace_manifest
                .workspace
                .as_ref()
                .map(|workspace| expand_workspace_members(&workspace_root, &workspace.members))
                .transpose()?
                .unwrap_or_default();

            for member_path in members {
                let member_cargo_toml = member_path.join("Cargo.toml");
                if let Some(member_manifest) = load_cargo_manifest(&member_cargo_toml)?
                    && let Some(framework) = detect_framework_in_manifest(&member_manifest)
                {
                    return Ok(framework);
                }
            }
        }
    }

    Err(CliError::config("No supported framework (leptos/dioxus) found in Cargo.toml"))
}

pub fn check_framework_dependency(framework: Framework) -> CliResult<bool> {
    match detect_framework() {
        Ok(found) => Ok(found == framework),
        Err(e) => {
            let err_msg = format!("{e}");
            if err_msg.contains("No supported framework") { Ok(false) } else { Err(e) }
        }
    }
}

/// Gets the tailwind input file path from Cargo.toml metadata.
/// Reads from `[[workspace.metadata.leptos]]` or `[package.metadata.leptos]`.
/// Returns an error if not found - user must add Leptos metadata to Cargo.toml.
pub fn get_tailwind_input_file() -> CliResult<String> {
    let current_dir = std::env::current_dir()?;
    get_tailwind_input_file_from_path(&current_dir)
}

pub fn get_tailwind_input_file_for_framework(framework: Framework) -> CliResult<String> {
    match framework {
        Framework::Leptos => get_tailwind_input_file(),
        Framework::Dioxus => {
            let current_dir = std::env::current_dir()?;
            detect_dioxus_tailwind_input_file(&current_dir)
        }
    }
}

/// Gets the tailwind input file from a specific path (useful for testing)
pub fn get_tailwind_input_file_from_path(start_path: &Path) -> CliResult<String> {
    // First try the local Cargo.toml
    let local_cargo_toml = start_path.join("Cargo.toml");
    if let Some(manifest) = load_cargo_manifest(&local_cargo_toml)?
        && let Some(tailwind_file) = extract_tailwind_from_manifest(&manifest)
    {
        return Ok(tailwind_file);
    }

    // If not found, try to find workspace root and read from there
    if let Some(workspace_root) = find_workspace_root(start_path)?
        && let Some(manifest) = load_cargo_manifest(&workspace_root.join("Cargo.toml"))?
        && let Some(tailwind_file) = extract_tailwind_from_manifest(&manifest)
    {
        return Ok(tailwind_file);
    }

    Err(CliError::config(
        "Missing `tailwind-input-file` in Cargo.toml. \
        Please add Leptos metadata to your Cargo.toml:\n\n\
        [package.metadata.leptos]\n\
        tailwind-input-file = \"style/tailwind.css\"\n\n\
        Or for workspaces:\n\n\
        [[workspace.metadata.leptos]]\n\
        tailwind-input-file = \"style/tailwind.css\"",
    ))
}

/// Extracts tailwind-input-file from a parsed Manifest
fn extract_tailwind_from_manifest(manifest: &Manifest) -> Option<String> {
    // Try workspace.metadata.leptos (array of tables stored as Value)
    if let Some(workspace) = &manifest.workspace
        && let Some(metadata) = &workspace.metadata
        && let Some(leptos_value) = metadata.get("leptos")
    {
        // [[workspace.metadata.leptos]] is an array
        if let Some(array) = leptos_value.as_array()
            && let Some(first) = array.first()
                && let Some(tailwind) = first.get("tailwind-input-file")
                    && let Some(value) = tailwind.as_str() {
                        return Some(value.to_string());
                    }
        // [workspace.metadata.leptos] could also be a table
        if let Some(tailwind) = leptos_value.get("tailwind-input-file")
            && let Some(value) = tailwind.as_str() {
                return Some(value.to_string());
            }
    }

    // Try package.metadata.leptos (single table)
    if let Some(package) = &manifest.package
        && let Some(metadata) = &package.metadata
        && let Some(leptos) = metadata.get("leptos")
        && let Some(tailwind) = leptos.get("tailwind-input-file")
        && let Some(value) = tailwind.as_str()
    {
        return Some(value.to_string());
    }

    None
}

fn detect_framework_in_manifest(manifest: &Manifest) -> Option<Framework> {
    if manifest.dependencies.contains_key("dioxus") {
        Some(Framework::Dioxus)
    } else if manifest.dependencies.contains_key("leptos") {
        Some(Framework::Leptos)
    } else {
        None
    }
}

fn detect_framework_in_workspace(workspace: &cargo_toml::Workspace<toml::Value>) -> Option<Framework> {
    if workspace.dependencies.contains_key("dioxus") {
        Some(Framework::Dioxus)
    } else if workspace.dependencies.contains_key("leptos") {
        Some(Framework::Leptos)
    } else {
        None
    }
}

fn inherited_supported_framework(manifest: &Manifest) -> Option<Framework> {
    for framework in [Framework::Dioxus, Framework::Leptos] {
        let dep_name = match framework {
            Framework::Leptos => "leptos",
            Framework::Dioxus => "dioxus",
        };

        if let Some(dep) = manifest.dependencies.get(dep_name)
            && matches!(dep, Dependency::Inherited(_))
        {
            return Some(framework);
        }
    }

    None
}

fn detect_dioxus_tailwind_input_file(start_path: &Path) -> CliResult<String> {
    if let Some(found) = find_dioxus_tailwind_in_dir(start_path, start_path) {
        return Ok(found);
    }

    if let Ok(workspace_info) = analyze_workspace() {
        if let Some(target_crate_path) = workspace_info.target_crate_path
            && let Some(found) = find_dioxus_tailwind_in_dir(start_path, &target_crate_path)
        {
            return Ok(found);
        }

        if let Some(workspace_root) = workspace_info.workspace_root
            && let Some(found) = find_dioxus_tailwind_in_dir(start_path, &workspace_root)
        {
            return Ok(found);
        }
    }

    Err(CliError::config(
        "Missing Dioxus Tailwind input file. Expected one of: style/tailwind.css, assets/tailwind.css, tailwind.css",
    ))
}

fn find_dioxus_tailwind_in_dir(base_dir: &Path, search_dir: &Path) -> Option<String> {
    for candidate in ["style/tailwind.css", "assets/tailwind.css", "tailwind.css"] {
        let full_path = search_dir.join(candidate);
        if full_path.exists() {
            let relative = full_path.strip_prefix(base_dir).ok().unwrap_or(&full_path);
            return Some(relative.to_string_lossy().to_string());
        }
    }
    None
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Helper function to load a Cargo.toml manifest from a path
/// Loads and parses a Cargo.toml file into a Manifest struct.
/// Returns None if the file doesn't exist, Ok(Manifest) if parsed successfully.
pub fn load_cargo_manifest(cargo_toml_path: &Path) -> CliResult<Option<Manifest>> {
    if !cargo_toml_path.exists() {
        return Ok(None);
    }

    // Try to load with full workspace resolution first
    match Manifest::from_path(cargo_toml_path) {
        Ok(manifest) => Ok(Some(manifest)),
        Err(_) => {
            // If workspace resolution fails (e.g., in tests), try parsing without workspace resolution
            let contents = std::fs::read_to_string(cargo_toml_path)?;
            let manifest = Manifest::from_slice(contents.as_bytes())?;
            Ok(Some(manifest))
        }
    }
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    /// Helper to create a Cargo.toml with given content
    fn write_cargo_toml(dir: &Path, content: &str) {
        fs::write(dir.join("Cargo.toml"), content).unwrap();
    }

    /// Helper to create a minimal src directory
    fn create_src_dir(dir: &Path) {
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("src/lib.rs"), "").unwrap();
    }

    #[test]
    fn test_single_crate_with_leptos() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.7"
"#,
        );
        create_src_dir(root);

        let info = analyze_workspace_from_path(root).unwrap();

        assert!(!info.is_workspace);
        assert_eq!(info.target_crate, Some("my-app".to_string()));
        assert_eq!(info.components_base_path, "src/components");
    }

    #[test]
    fn test_single_crate_without_leptos() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1"
"#,
        );
        create_src_dir(root);

        let result = analyze_workspace_from_path(root);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("supported framework"));
    }

    #[test]
    fn test_workspace_with_leptos_member() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app", "server"]
"#,
        );

        // Create app member with leptos
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        write_cargo_toml(
            &app_dir,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.7"
"#,
        );
        create_src_dir(&app_dir);

        // Create server member without leptos
        let server_dir = root.join("server");
        fs::create_dir_all(&server_dir).unwrap();
        write_cargo_toml(
            &server_dir,
            r#"
[package]
name = "my-server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
"#,
        );
        create_src_dir(&server_dir);

        // Test from workspace root
        let info = analyze_workspace_from_path(root).unwrap();

        assert!(info.is_workspace);
        assert_eq!(info.target_crate, Some("my-app".to_string()));
        assert_eq!(info.components_base_path, "app/src/components");
    }

    #[test]
    fn test_workspace_from_member_directory() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["frontend"]
"#,
        );

        // Create frontend member with leptos
        let frontend_dir = root.join("frontend");
        fs::create_dir_all(&frontend_dir).unwrap();
        write_cargo_toml(
            &frontend_dir,
            r#"
[package]
name = "frontend"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.7"
"#,
        );
        create_src_dir(&frontend_dir);

        // Test from member directory
        let info = analyze_workspace_from_path(&frontend_dir).unwrap();

        assert!(info.is_workspace);
        assert_eq!(info.target_crate, Some("frontend".to_string()));
        // When running from member, path is relative to member
        assert_eq!(info.components_base_path, "src/components");
    }

    #[test]
    fn test_workspace_with_workspace_dependencies() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root with workspace.dependencies
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[workspace.dependencies]
leptos = "0.7"
"#,
        );

        // Create app member that inherits leptos
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        write_cargo_toml(
            &app_dir,
            r#"
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos.workspace = true
"#,
        );
        create_src_dir(&app_dir);

        let info = analyze_workspace_from_path(root).unwrap();

        assert!(info.is_workspace);
        assert_eq!(info.target_crate, Some("my-app".to_string()));
    }

    #[test]
    fn test_workspace_no_leptos_member() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["server"]
"#,
        );

        let server_dir = root.join("server");
        fs::create_dir_all(&server_dir).unwrap();
        write_cargo_toml(
            &server_dir,
            r#"
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
"#,
        );
        create_src_dir(&server_dir);

        let result = analyze_workspace_from_path(root);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("supported framework"));
    }

    #[test]
    fn test_no_cargo_toml() {
        let temp = TempDir::new().unwrap();
        let result = analyze_workspace_from_path(temp.path());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cargo.toml"));
    }

    #[test]
    fn test_workspace_info_default() {
        let info = WorkspaceInfo::default();

        assert!(!info.is_workspace);
        assert!(info.workspace_root.is_none());
        assert!(info.target_crate.is_none());
        assert_eq!(info.components_base_path, "src/components");
    }

    // ========== Single crate with [workspace] isolation Tests ==========

    #[test]
    fn test_single_crate_with_empty_workspace_section_and_leptos() {
        // Reproduces the `[workspace]` + `[package]` pattern used by program exercises
        // to prevent parent workspace lookup. Should be treated as a single crate.
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "w4_d19_resource_fetch_csr"
version = "0.1.0"
edition = "2024"

[workspace]

[dependencies]
leptos = { version = "0.8", features = ["csr", "tracing"] }
leptos_router = { version = "0.8", features = ["nightly"] }
"#,
        );
        create_src_dir(root);

        let info = analyze_workspace_from_path(root).unwrap();

        assert!(!info.is_workspace);
        assert_eq!(info.target_crate, Some("w4_d19_resource_fetch_csr".to_string()));
        assert_eq!(info.components_base_path, "src/components");
    }

    #[test]
    fn test_single_crate_with_empty_workspace_section_without_leptos() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "no-leptos"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
serde = "1"
"#,
        );
        create_src_dir(root);

        let result = analyze_workspace_from_path(root);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("supported framework"));
    }

    // ========== Tailwind Input File Tests ==========

    #[test]
    fn test_get_tailwind_from_workspace_metadata() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[[workspace.metadata.leptos]]
name = "my-app"
tailwind-input-file = "style/main.css"
"#,
        );

        let result = get_tailwind_input_file_from_path(root).unwrap();
        assert_eq!(result, "style/main.css");
    }

    #[test]
    fn test_get_tailwind_from_package_metadata() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"

[package.metadata.leptos]
tailwind-input-file = "assets/tailwind.css"
"#,
        );

        let result = get_tailwind_input_file_from_path(root).unwrap();
        assert_eq!(result, "assets/tailwind.css");
    }

    #[test]
    fn test_get_tailwind_from_workspace_root_when_in_member() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root with tailwind config
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[[workspace.metadata.leptos]]
name = "my-app"
tailwind-input-file = "style/global.css"
"#,
        );

        // Create member without tailwind config
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        write_cargo_toml(
            &app_dir,
            r#"
[package]
name = "app"
version = "0.1.0"

[dependencies]
leptos = "0.7"
"#,
        );

        // Should find tailwind from workspace root
        let result = get_tailwind_input_file_from_path(&app_dir).unwrap();
        assert_eq!(result, "style/global.css");
    }

    #[test]
    fn test_get_tailwind_missing_returns_error() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
leptos = "0.7"
"#,
        );

        let result = get_tailwind_input_file_from_path(root);
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("tailwind-input-file"));
        assert!(err_msg.contains("Cargo.toml"));
    }

    #[test]
    fn test_get_tailwind_no_cargo_toml_returns_error() {
        let temp = TempDir::new().unwrap();

        let result = get_tailwind_input_file_from_path(temp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_tailwind_prefers_local_over_workspace() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create workspace root with one tailwind config
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[[workspace.metadata.leptos]]
name = "workspace-app"
tailwind-input-file = "style/workspace.css"
"#,
        );

        // Create member with its own tailwind config
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir).unwrap();
        write_cargo_toml(
            &app_dir,
            r#"
[package]
name = "app"
version = "0.1.0"

[package.metadata.leptos]
tailwind-input-file = "style/local.css"
"#,
        );

        // Should prefer local config
        let result = get_tailwind_input_file_from_path(&app_dir).unwrap();
        assert_eq!(result, "style/local.css");
    }

    #[test]
    fn test_get_tailwind_multiple_leptos_entries_uses_first() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Multiple [[workspace.metadata.leptos]] entries - should use first
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[[workspace.metadata.leptos]]
name = "first-app"
tailwind-input-file = "style/first.css"

[[workspace.metadata.leptos]]
name = "second-app"
tailwind-input-file = "style/second.css"
"#,
        );

        let result = get_tailwind_input_file_from_path(root).unwrap();
        assert_eq!(result, "style/first.css");
    }

    #[test]
    fn test_get_tailwind_workspace_single_table_format() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // [workspace.metadata.leptos] as single table (not array)
        write_cargo_toml(
            root,
            r#"
[workspace]
members = ["app"]

[workspace.metadata.leptos]
tailwind-input-file = "style/single.css"
"#,
        );

        let result = get_tailwind_input_file_from_path(root).unwrap();
        assert_eq!(result, "style/single.css");
    }

    #[test]
    fn test_get_tailwind_metadata_exists_but_no_leptos_key() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"

[package.metadata]
some-other-tool = { key = "value" }
"#,
        );

        let result = get_tailwind_input_file_from_path(root);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_tailwind_leptos_exists_but_no_tailwind_key() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"

[package.metadata.leptos]
name = "my-app"
site-root = "target/site"
"#,
        );

        let result = get_tailwind_input_file_from_path(root);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_tailwind_empty_value() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        write_cargo_toml(
            root,
            r#"
[package]
name = "my-app"
version = "0.1.0"

[package.metadata.leptos]
tailwind-input-file = ""
"#,
        );

        // Empty string is technically valid - returns empty
        let result = get_tailwind_input_file_from_path(root).unwrap();
        assert_eq!(result, "");
    }
}
