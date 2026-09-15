use ui_cli::shared::rust_ui_client::RustUIClient;

#[tokio::test]
async fn test_fetch_tree_md() {
    let result = RustUIClient::fetch_tree_md().await;

    assert!(result.is_ok(), "Failed to fetch tree.md: {:?}", result.err());

    let content = result.unwrap();
    assert!(!content.is_empty(), "tree.md content should not be empty");
}

#[tokio::test]
async fn test_fetch_styles_default_alert() {
    let result = RustUIClient::fetch_styles_default("alert").await;

    assert!(result.is_ok(), "Failed to fetch alert.md: {:?}", result.err());

    let rust_code = result.unwrap();
    assert!(!rust_code.is_empty(), "Extracted Rust code from alert.md should not be empty");
    // Basic sanity check that it contains Rust code
    assert!(
        rust_code.contains("fn") || rust_code.contains("use") || rust_code.contains("pub"),
        "Content should contain Rust code"
    );
}

#[tokio::test]
async fn test_fetch_styles_default_button() {
    let result = RustUIClient::fetch_styles_default("button").await;

    assert!(result.is_ok(), "Failed to fetch button.md: {:?}", result.err());

    let rust_code = result.unwrap();
    assert!(!rust_code.is_empty(), "Extracted Rust code from button.md should not be empty");
}

#[tokio::test]
async fn test_fetch_nonexistent_component() {
    let result = RustUIClient::fetch_styles_default("nonexistent_component_xyz").await;

    // Should fail for nonexistent components
    assert!(result.is_err(), "Should fail when fetching nonexistent component");
}
