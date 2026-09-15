use crate::shared::cli_error::{CliError, CliResult};
use crate::shared::framework::Framework;
use crate::shared::markdown_utils::extract_rust_code_from_markdown;

pub struct RustUIClient;

impl RustUIClient {
    fn tree_url(framework: Framework) -> String {
        format!("{}/tree.md", framework.registry_base_url())
    }

    fn component_url(component_name: &str, framework: Framework) -> String {
        format!("{}/styles/default/{component_name}.md", framework.registry_base_url())
    }

    fn js_file_url(path: &str, framework: Framework) -> String {
        format!("{}{path}", framework.site_url())
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
    pub async fn fetch_tree_md(framework: Framework) -> CliResult<String> {
        let response = Self::fetch_response(&Self::tree_url(framework)).await?;
        let content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        if content.is_empty() {
            return Err(CliError::registry_request_failed());
        }

        Ok(content)
    }

    pub async fn fetch_styles_default(component_name: &str, framework: Framework) -> CliResult<String> {
        let response = Self::fetch_response(&Self::component_url(component_name, framework)).await?;
        let markdown_content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        extract_rust_code_from_markdown(&markdown_content).ok_or_else(CliError::registry_component_missing)
    }

    /// Fetch a JS file from the site (e.g., /app_components/lazy_load_sonner.js)
    pub async fn fetch_js_file(path: &str, framework: Framework) -> CliResult<String> {
        let response = Self::fetch_response(&Self::js_file_url(path, framework)).await?;
        let content = response.text().await.map_err(|_| CliError::registry_request_failed())?;

        if content.is_empty() {
            return Err(CliError::registry_request_failed());
        }

        Ok(content)
    }
}
