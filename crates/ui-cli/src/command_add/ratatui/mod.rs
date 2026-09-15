mod app;
mod crossterm;
mod header;
mod tabs;
mod widgets;

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use crate::shared::cli_error::{CliError, CliResult};

/// Map of component name to its dependencies
pub type DependencyMap = HashMap<String, Vec<String>>;

/// Run the ratatui TUI for adding components
/// Returns the selected components when user confirms
pub fn run_tui(
    components: Vec<String>,
    installed: HashSet<String>,
    dependencies: DependencyMap,
) -> CliResult<Vec<String>> {
    let tick_rate = Duration::from_millis(250);
    crossterm::run(tick_rate, components, installed, dependencies)
        .map_err(|err| CliError::Io { source: std::io::Error::other(err.to_string()) })
}
