use serde::de::DeserializeOwned;

pub fn extract_frontmatter(md_file_content: &str) -> Option<String> {
    let trimmed = md_file_content.trim_start();

    if trimmed.starts_with("+++") {
        if let Some(content_after_prefix) = trimmed.strip_prefix("+++") {
            if let Some(end_index) = content_after_prefix.find("+++") {
                let frontmatter_content = content_after_prefix[..end_index].trim();
                return Some(frontmatter_content.to_string());
            }
        }
    }

    None
}

pub fn extract_content_without_frontmatter(raw_content: &str) -> &str {
    let trimmed = raw_content.trim_start();

    if trimmed.starts_with("+++") {
        if let Some(content_after_prefix) = trimmed.strip_prefix("+++") {
            if let Some(end_index) = content_after_prefix.find("+++") {
                let remaining_content = &content_after_prefix[end_index + 3..];
                return remaining_content;
            }
        }
    }

    raw_content
}

pub fn parse_frontmatter_from_content<T: DeserializeOwned>(content: &str) -> Result<T, String> {
    // Parse frontmatter manually since gray_matter API is complex for our needs
    if !content.starts_with("+++\n") {
        return Err("Frontmatter is missing or does not start with +++".to_string());
    }

    // Find the end of frontmatter
    let content_after_start = &content[4..]; // Skip initial "+++\n"
    let end_pos = content_after_start.find("\n+++\n").ok_or("Missing closing +++".to_string())?;

    let toml_content = &content_after_start[..end_pos];

    // Try to parse TOML frontmatter with toml
    toml::from_str::<T>(toml_content).map_err(|err| format!("Failed to parse frontmatter: {err}"))
}
