use std::collections::HashSet;
use std::path::Path;

/// Scan the components directory and return a set of installed component names
pub fn get_installed_components(base_path: &str) -> HashSet<String> {
    let mut installed = HashSet::new();
    let base = Path::new(base_path);

    if !base.exists() {
        return installed;
    }

    // Scan subdirectories: ui/, demos/, hooks/, extensions/
    let subdirs = ["ui", "demos", "hooks", "extensions"];

    for subdir in subdirs {
        let dir_path = base.join(subdir);
        if let Ok(entries) = std::fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && path.extension().is_some_and(|ext| ext == "rs")
                    && let Some(stem) = path.file_stem()
                {
                    let name = stem.to_string_lossy().to_string();
                    // Skip mod.rs
                    if name != "mod" {
                        installed.insert(name);
                    }
                }
            }
        }
    }

    installed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn create_temp_dir(test_name: &str) -> std::path::PathBuf {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("ui_cli_{}_{}", test_name, id));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn returns_empty_for_nonexistent_path() {
        let result = get_installed_components("/nonexistent/path/12345");
        assert!(result.is_empty());
    }

    #[test]
    fn returns_empty_for_empty_directory() {
        let temp_dir = create_temp_dir("empty");
        let result = get_installed_components(temp_dir.to_str().unwrap());
        assert!(result.is_empty());
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn finds_components_in_ui_subdir() {
        let temp_dir = create_temp_dir("ui_subdir");
        let ui_dir = temp_dir.join("ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("button.rs"), "// button").unwrap();
        fs::write(ui_dir.join("card.rs"), "// card").unwrap();

        let result = get_installed_components(temp_dir.to_str().unwrap());
        assert!(result.contains("button"));
        assert!(result.contains("card"));
        assert_eq!(result.len(), 2);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn skips_mod_rs() {
        let temp_dir = create_temp_dir("mod_rs");
        let ui_dir = temp_dir.join("ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("mod.rs"), "// mod").unwrap();
        fs::write(ui_dir.join("button.rs"), "// button").unwrap();

        let result = get_installed_components(temp_dir.to_str().unwrap());
        assert!(!result.contains("mod"));
        assert!(result.contains("button"));
        assert_eq!(result.len(), 1);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn finds_components_across_subdirs() {
        let temp_dir = create_temp_dir("across_subdirs");
        fs::create_dir_all(temp_dir.join("ui")).unwrap();
        fs::create_dir_all(temp_dir.join("demos")).unwrap();
        fs::create_dir_all(temp_dir.join("hooks")).unwrap();

        fs::write(temp_dir.join("ui/button.rs"), "").unwrap();
        fs::write(temp_dir.join("demos/demo_button.rs"), "").unwrap();
        fs::write(temp_dir.join("hooks/use_click.rs"), "").unwrap();

        let result = get_installed_components(temp_dir.to_str().unwrap());
        assert!(result.contains("button"));
        assert!(result.contains("demo_button"));
        assert!(result.contains("use_click"));
        assert_eq!(result.len(), 3);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn ignores_non_rs_files() {
        let temp_dir = create_temp_dir("non_rs");
        let ui_dir = temp_dir.join("ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("button.rs"), "").unwrap();
        fs::write(ui_dir.join("readme.md"), "").unwrap();
        fs::write(ui_dir.join("style.css"), "").unwrap();

        let result = get_installed_components(temp_dir.to_str().unwrap());
        assert_eq!(result.len(), 1);
        assert!(result.contains("button"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
