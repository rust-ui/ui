use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::markdown_utils::extract_rust_code_from_markdown;

pub struct RustUIClient;

impl RustUIClient {
    const BASE_URL: &str = "https://www.rust-ui.com/registry";
    const SITE_URL: &str = "https://www.rust-ui.com";

    // URL builders - centralized URL construction
    fn tree_url() -> String {
        format!("{}/tree.md", Self::BASE_URL)
    }

    fn component_url(component_name: &str) -> String {
        format!("{}/styles/default/{component_name}.md", Self::BASE_URL)
    }

    fn js_file_url(path: &str) -> String {
        format!("{}{path}", Self::SITE_URL)
    }

    // Consolidated HTTP fetch method
    async fn fetch_response(url: &str) -> CliResult<reqwest::Response> {
        let response = reqwest::get(url).await.map_err(|_| CliError::registry_request_failed())?;

        if !response.status().is_success() {
            return Err(CliError::registry_request_failed());
        }

        Ok(response)
    }

    // Public API methods
    pub async fn fetch_tree_md() -> CliResult<String> {
        let response = Self::fetch_response(&Self::tree_url()).await?;
        let content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        if content.is_empty() {
            return Err(CliError::registry_request_failed());
        }

        Ok(content)
    }

    pub async fn fetch_styles_default(component_name: &str) -> CliResult<String> {
        let response = Self::fetch_response(&Self::component_url(component_name)).await?;
        let markdown_content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        extract_rust_code_from_markdown(&markdown_content).ok_or_else(CliError::registry_component_missing)
    }

    /// Fetch a JS file from the site (e.g., /hooks/lazy_load_sonner.js)
    pub async fn fetch_js_file(path: &str) -> CliResult<String> {
        let response = Self::fetch_response(&Self::js_file_url(path)).await?;
        let content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        if content.is_empty() {
            return Err(CliError::registry_request_failed());
        }

        Ok(content)
    }
}
