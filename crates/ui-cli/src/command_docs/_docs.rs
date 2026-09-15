use std::process::Command as ProcessCommand;

use clap::{Arg, ArgMatches, Command};

use crate::shared::cli_error::{CliError, CliResult};

const DOCS_URL: &str = "https://rust-ui.com";
const DOCS_COMPONENTS_BASE: &str = "https://rust-ui.com/docs/components";

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_docs() -> Command {
    Command::new("docs")
        .about("Open the rust-ui documentation in your browser")
        .arg(
            Arg::new("components")
                .help("Component name(s) to get docs for")
                .required(false)
                .num_args(1..),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output component URL(s) as JSON instead of opening browser")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn process_docs(matches: &ArgMatches) -> CliResult<()> {
    let components: Vec<String> =
        matches.get_many::<String>("components").unwrap_or_default().cloned().collect();
    let json = matches.get_flag("json");

    if components.is_empty() {
        // No component specified — open homepage
        println!("Opening {DOCS_URL} ...");
        return open_url(DOCS_URL);
    }

    let urls: Vec<(String, String)> = components
        .iter()
        .map(|name| {
            let slug = name.replace('_', "-");
            let url = format!("{DOCS_COMPONENTS_BASE}/{slug}");
            (name.clone(), url)
        })
        .collect();

    if json {
        let items: Vec<serde_json::Value> = urls
            .iter()
            .map(|(name, url)| serde_json::json!({ "component": name, "url": url }))
            .collect();
        let output = if items.len() == 1 {
            serde_json::to_string_pretty(&items[0])
        } else {
            serde_json::to_string_pretty(&items)
        }
        .map_err(|e| CliError::file_operation(&e.to_string()))?;
        println!("{output}");
    } else {
        for (name, url) in &urls {
            println!("{name}\n  {url}");
        }
    }

    Ok(())
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                          */
/* ========================================================== */

/// Opens the given URL in the system default browser.
pub fn open_url(url: &str) -> CliResult<()> {
    let (program, args) = browser_command(url);

    let status = ProcessCommand::new(program)
        .args(&args)
        .status()
        .map_err(|_| CliError::validation("Failed to launch browser"))?;

    if status.success() {
        Ok(())
    } else {
        Err(CliError::validation("Browser command exited with a non-zero status"))
    }
}

/// Returns the platform command and args needed to open `url`.
pub fn browser_command(url: &str) -> (&'static str, Vec<String>) {
    #[cfg(target_os = "macos")]
    return ("open", vec![url.to_string()]);

    #[cfg(target_os = "linux")]
    return ("xdg-open", vec![url.to_string()]);

    #[cfg(target_os = "windows")]
    return ("cmd", vec!["/c".to_string(), "start".to_string(), url.to_string()]);

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return ("xdg-open", vec![url.to_string()]);
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docs_url_is_https() {
        assert!(DOCS_URL.starts_with("https://"));
    }

    #[test]
    fn docs_url_has_no_trailing_slash() {
        assert!(!DOCS_URL.ends_with('/'));
    }

    #[test]
    fn browser_command_includes_url() {
        let (_, args) = browser_command("https://example.com");
        assert!(args.iter().any(|a| a.contains("https://example.com")));
    }

    #[test]
    fn browser_command_program_is_non_empty() {
        let (program, _) = browser_command(DOCS_URL);
        assert!(!program.is_empty());
    }

    #[test]
    fn docs_with_component_prints_url() {
        let m = command_docs().try_get_matches_from(["docs", "button"]).unwrap();
        let components: Vec<String> =
            m.get_many::<String>("components").unwrap_or_default().cloned().collect();
        assert_eq!(components, vec!["button"]);
        let slug = components[0].replace('_', "-");
        assert_eq!(format!("{DOCS_COMPONENTS_BASE}/{slug}"), "https://rust-ui.com/docs/components/button");
    }

    #[test]
    fn docs_normalizes_underscore_to_hyphen() {
        let name = "button_group";
        let slug = name.replace('_', "-");
        assert_eq!(slug, "button-group");
        assert_eq!(
            format!("{DOCS_COMPONENTS_BASE}/{slug}"),
            "https://rust-ui.com/docs/components/button-group"
        );
    }

    #[test]
    fn docs_json_flag_is_registered() {
        let m = command_docs().try_get_matches_from(["docs", "button", "--json"]).unwrap();
        assert!(m.get_flag("json"));
    }

    #[test]
    fn docs_multiple_components_accepted() {
        let m = command_docs().try_get_matches_from(["docs", "button", "badge", "card"]).unwrap();
        let components: Vec<String> =
            m.get_many::<String>("components").unwrap_or_default().cloned().collect();
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn docs_no_arg_has_no_components() {
        let m = command_docs().try_get_matches_from(["docs"]).unwrap();
        let components: Vec<String> =
            m.get_many::<String>("components").unwrap_or_default().cloned().collect();
        assert!(components.is_empty());
    }
}
