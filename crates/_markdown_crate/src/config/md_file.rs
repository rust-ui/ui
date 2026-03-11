use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdFile<T> {
    pub frontmatter: T,
    pub content: String,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

// Generic implementation for any frontmatter type
impl<T: DeserializeOwned> MdFile<T> {
    pub fn parse_md_file(raw_content: &str) -> Result<MdFile<T>, String> {
        let content = raw_content.trim();

        // Check if content starts with frontmatter delimiter
        if !content.starts_with("+++") {
            return Err("No frontmatter found".to_string());
        }

        // Find the end of frontmatter
        let frontmatter_end = content[3..].find("+++").ok_or("Invalid frontmatter: missing closing delimiter")?;

        let frontmatter_toml = &content[3..frontmatter_end + 3];
        let md_content = &content[frontmatter_end + 6..];

        // Parse the TOML frontmatter
        let frontmatter: T =
            toml::from_str(frontmatter_toml).map_err(|err| format!("Failed to parse frontmatter: {err}"))?;

        Ok(MdFile { frontmatter, content: md_content.trim().to_string() })
    }
}
