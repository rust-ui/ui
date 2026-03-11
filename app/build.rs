use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current date for footer using system date command
    let output = Command::new("date").args(["+%Y-%m-%d"]).output()?;
    let build_date = String::from_utf8(output.stdout)?.trim().to_owned();
    println!("cargo:rustc-env=BUILD_DATE={build_date}");

    // Try to read the CSS file and hash its content
    // In dev mode, this might not exist yet, so use a default hash
    let css_content = std::fs::read_to_string("../target/site/pkg/deploy_rust_ui.css")
        .or_else(|_| std::fs::read_to_string("target/site/pkg/deploy_rust_ui.css"))
        .unwrap_or_else(|_| String::from("default"));

    // Generate hash from CSS content
    let mut hasher = DefaultHasher::new();
    css_content.hash(&mut hasher);
    let hash = hasher.finish();

    // Convert to hex string
    let css_hash = format!("{hash:x}");

    // Set as environment variable for use in code via env!("CSS_HASH")
    println!("cargo:rustc-env=CSS_HASH={css_hash}");

    // Rerun if CSS file changes
    println!("cargo:rerun-if-changed=../target/site/pkg/deploy_rust_ui.css");
    println!("cargo:rerun-if-changed=target/site/pkg/deploy_rust_ui.css");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
