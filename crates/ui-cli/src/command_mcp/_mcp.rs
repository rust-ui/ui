use clap::{Arg, ArgMatches, Command};
use dialoguer::Select;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;

use super::config::{McpClient, write_mcp_config};
use super::tools;
use crate::shared::cli_error::{CliError, CliResult};

/* ========================================================== */
/*                      🔧 CLAP COMMANDS 🔧                   */
/* ========================================================== */

pub fn command_mcp() -> Command {
    Command::new("mcp")
        .about("Start the MCP server or write editor config")
        .subcommand(
            Command::new("init")
                .about("Write MCP config for your editor")
                .arg(
                    Arg::new("client")
                        .long("client")
                        .value_parser(["claude", "cursor", "vscode", "opencode"])
                        .help("Editor client (claude, cursor, vscode, opencode)"),
                ),
        )
}

/* ========================================================== */
/*                      🦀 PROCESS FNS 🦀                     */
/* ========================================================== */

pub async fn process_mcp_server() -> CliResult<()> {
    let transport = rmcp::transport::stdio();
    let server = RustUiMcpServer::new()
        .serve(transport)
        .await
        .map_err(|e| CliError::file_operation(&e.to_string()))?;
    server
        .waiting()
        .await
        .map_err(|e| CliError::file_operation(&e.to_string()))?;
    Ok(())
}

pub fn process_mcp_init(matches: &ArgMatches) -> CliResult<()> {
    let client_name = match matches.get_one::<String>("client") {
        Some(s) => s.clone(),
        None => {
            let names = McpClient::all_names();
            let labels = ["Claude Code", "Cursor", "VS Code", "OpenCode"];
            let idx = Select::new()
                .with_prompt("Which editor are you using?")
                .items(labels)
                .default(0)
                .interact()
                .map_err(|e| CliError::file_operation(&e.to_string()))?;
            names.get(idx).copied().unwrap_or("claude").to_string()
        }
    };

    let client =
        McpClient::from_str(&client_name).ok_or_else(|| CliError::validation("Unknown client"))?;

    let label = client.label();
    let cwd = std::env::current_dir()?;
    let config_path = write_mcp_config(client, &cwd)?;

    println!("Configured rust-ui MCP server for {label}.");
    println!("Config written to: {config_path}");
    println!();
    println!("Restart your editor to load the MCP server.");
    Ok(())
}

/* ========================================================== */
/*                     🛠  MCP SERVER 🛠                       */
/* ========================================================== */

#[derive(Debug, Deserialize, JsonSchema)]
struct CategoryFilter {
    /// Optional category name to filter components (e.g. "ui", "demos")
    category: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchQuery {
    /// Search query string (case-insensitive, partial match)
    query: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ComponentName {
    /// Component name (e.g. "button", "accordion", "demo_button")
    name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AddComponents {
    /// One or more component names to install
    components: Vec<String>,
}

#[derive(Debug, Clone)]
struct RustUiMcpServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router(router = tool_router)]
impl RustUiMcpServer {
    pub fn new() -> Self {
        Self { tool_router: Self::tool_router() }
    }

    #[tool(description = "List all available rust-ui components, optionally filtered by category")]
    async fn list_components(&self, params: Parameters<CategoryFilter>) -> String {
        tools::list_components(params.0.category).await.unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "Search for rust-ui components by name (case-insensitive partial match)")]
    async fn search_components(&self, params: Parameters<SearchQuery>) -> String {
        tools::search_components(&params.0.query).await.unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "View the full Rust source code of a component from the registry")]
    async fn view_component(&self, params: Parameters<ComponentName>) -> String {
        tools::view_component(&params.0.name).await.unwrap_or_else(|e| format!("Error: {e}"))
    }

    #[tool(description = "Get the 'ui add' command to install one or more components into your project")]
    async fn get_add_command(&self, params: Parameters<AddComponents>) -> String {
        format!("ui add {}", params.0.components.join(" "))
    }

    #[tool(description = "Checklist to verify after adding rust-ui components (imports, Cargo.toml, Tailwind, etc.)")]
    fn get_audit_checklist(&self) -> String {
        tools::audit_checklist()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for RustUiMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "rust-ui component registry. Use list_components to browse, \
             search_components to find, view_component to inspect source, \
             get_add_command to get the install command, \
             get_audit_checklist after installing.",
        )
    }
}
