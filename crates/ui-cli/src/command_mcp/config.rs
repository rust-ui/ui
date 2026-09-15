use std::fs;
use std::path::Path;

use crate::shared::cli_error::CliResult;

pub enum McpClient {
    Claude,
    Cursor,
    VsCode,
    OpenCode,
}

impl McpClient {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "claude" => Some(Self::Claude),
            "cursor" => Some(Self::Cursor),
            "vscode" => Some(Self::VsCode),
            "opencode" => Some(Self::OpenCode),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Cursor => "Cursor",
            Self::VsCode => "VS Code",
            Self::OpenCode => "OpenCode",
        }
    }

    pub fn config_path(&self) -> &'static str {
        match self {
            Self::Claude => ".mcp.json",
            Self::Cursor => ".cursor/mcp.json",
            Self::VsCode => ".vscode/mcp.json",
            Self::OpenCode => "opencode.json",
        }
    }

    pub fn all_names() -> &'static [&'static str] {
        &["claude", "cursor", "vscode", "opencode"]
    }
}

pub fn write_mcp_config(client: McpClient, cwd: &Path) -> CliResult<String> {
    let config_path = cwd.join(client.config_path());

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let new_config = match client {
        McpClient::Claude | McpClient::Cursor => serde_json::json!({
            "mcpServers": {
                "rust-ui": { "command": "ui", "args": ["mcp"] }
            }
        }),
        McpClient::VsCode => serde_json::json!({
            "servers": {
                "rust-ui": { "command": "ui", "args": ["mcp"] }
            }
        }),
        McpClient::OpenCode => serde_json::json!({
            "$schema": "https://opencode.ai/config.json",
            "mcp": {
                "rust-ui": {
                    "type": "local",
                    "command": ["ui", "mcp"],
                    "enabled": true
                }
            }
        }),
    };

    let merged = if config_path.exists() {
        let raw = fs::read_to_string(&config_path)?;
        let existing: serde_json::Value = serde_json::from_str(&raw).unwrap_or(serde_json::json!({}));
        merge_json(existing, new_config)
    } else {
        new_config
    };

    let content = serde_json::to_string_pretty(&merged)? + "\n";
    fs::write(&config_path, content)?;

    Ok(client.config_path().to_string())
}

/// Deep-merge two JSON objects. Object keys are merged recursively; all other
/// values are overwritten by the override.
fn merge_json(base: serde_json::Value, override_val: serde_json::Value) -> serde_json::Value {
    match (base, override_val) {
        (serde_json::Value::Object(mut a), serde_json::Value::Object(b)) => {
            for (k, v) in b {
                let merged = match a.remove(&k) {
                    Some(existing) => merge_json(existing, v),
                    None => v,
                };
                a.insert(k, merged);
            }
            serde_json::Value::Object(a)
        }
        (_, b) => b,
    }
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn read_json(dir: &TempDir, path: &str) -> serde_json::Value {
        let content = fs::read_to_string(dir.path().join(path)).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    #[test]
    fn claude_writes_mcp_json() {
        let dir = TempDir::new().unwrap();
        let result = write_mcp_config(McpClient::Claude, dir.path());
        assert!(result.is_ok());
        assert!(dir.path().join(".mcp.json").exists());
    }

    #[test]
    fn claude_config_has_correct_command() {
        let dir = TempDir::new().unwrap();
        write_mcp_config(McpClient::Claude, dir.path()).unwrap();
        let json = read_json(&dir, ".mcp.json");
        assert_eq!(json["mcpServers"]["rust-ui"]["command"], "ui");
        assert_eq!(json["mcpServers"]["rust-ui"]["args"][0], "mcp");
    }

    #[test]
    fn cursor_writes_to_cursor_subdir() {
        let dir = TempDir::new().unwrap();
        let result = write_mcp_config(McpClient::Cursor, dir.path());
        assert!(result.is_ok());
        assert!(dir.path().join(".cursor/mcp.json").exists());
    }

    #[test]
    fn vscode_uses_servers_key_not_mcp_servers() {
        let dir = TempDir::new().unwrap();
        write_mcp_config(McpClient::VsCode, dir.path()).unwrap();
        let json = read_json(&dir, ".vscode/mcp.json");
        assert!(json.get("servers").is_some());
        assert!(json.get("mcpServers").is_none());
    }

    #[test]
    fn opencode_has_schema_and_enabled_flag() {
        let dir = TempDir::new().unwrap();
        write_mcp_config(McpClient::OpenCode, dir.path()).unwrap();
        let json = read_json(&dir, "opencode.json");
        assert!(json.get("$schema").is_some());
        assert_eq!(json["mcp"]["rust-ui"]["enabled"], true);
    }

    #[test]
    fn merges_with_existing_config_without_overwriting_other_keys() {
        let dir = TempDir::new().unwrap();
        let existing = serde_json::json!({ "mcpServers": { "other-tool": { "command": "other" } } });
        fs::write(dir.path().join(".mcp.json"), serde_json::to_string_pretty(&existing).unwrap()).unwrap();

        write_mcp_config(McpClient::Claude, dir.path()).unwrap();

        let json = read_json(&dir, ".mcp.json");
        assert!(json["mcpServers"].get("other-tool").is_some(), "existing key must be preserved");
        assert!(json["mcpServers"].get("rust-ui").is_some(), "new key must be added");
    }

    #[test]
    fn merge_json_preserves_base_keys() {
        let base = serde_json::json!({ "a": 1, "b": 2 });
        let override_val = serde_json::json!({ "b": 99, "c": 3 });
        let merged = merge_json(base, override_val);
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["b"], 99);
        assert_eq!(merged["c"], 3);
    }

    #[test]
    fn merge_json_non_object_override_wins() {
        let base = serde_json::json!("old");
        let override_val = serde_json::json!("new");
        let merged = merge_json(base, override_val);
        assert_eq!(merged, serde_json::json!("new"));
    }
}
