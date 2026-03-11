use app_domain::markdown_config::RegistryEntry;
use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json};
use markdown_crate::MdFile;

/// * We need to specify input & output for mobile targets (Android, iOS) to work. DO NOT CHANGE.
#[server(ReadMdFileTyped, "/api", input = GetUrl, output = Json)]
pub async fn read_md_file_typed(path: String) -> Result<MdFile<RegistryEntry>, ServerFnError> {
    let site_root = std::env::var("LEPTOS_SITE_ROOT")
        .map_err(|_| ServerFnError::new("LEPTOS_SITE_ROOT environment variable not set"))?;

    // Strip "public/" prefix since cargo leptos build copies public/* to site root
    let relative_path = path.strip_prefix("public/").unwrap_or(&path);
    let full_path = format!("{}/{}", site_root, relative_path);

    let raw_content = std::fs::read_to_string(&full_path)
        .map_err(|err| ServerFnError::new(format!("Failed to read MDX file from {full_path}: {err}")))?;

    MdFile::parse_md_file(&raw_content).map_err(|err| ServerFnError::new(format!("Failed to parse MDX content: {err}")))
}

/// * We need to specify input & output for mobile targets (Android, iOS) to work. DO NOT CHANGE.
#[server(ReadRawMdFile, "/api", input = GetUrl, output = Json)]
pub async fn read_raw_md_file(path: String) -> Result<String, ServerFnError> {
    let site_root = std::env::var("LEPTOS_SITE_ROOT")
        .map_err(|_| ServerFnError::new("LEPTOS_SITE_ROOT environment variable not set"))?;

    // Strip "public/" prefix since cargo leptos build copies public/* to site root
    let relative_path = path.strip_prefix("public/").unwrap_or(&path);
    let full_path = format!("{}/{}", site_root, relative_path);

    std::fs::read_to_string(&full_path)
        .map_err(|err| ServerFnError::new(format!("Failed to read MD file from {full_path}: {err}")))
}
