pub fn extract_rust_code_from_markdown(markdown: &str) -> Option<String> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut in_rust_block = false;
    let mut rust_code_lines = Vec::new();

    for line in lines {
        if line.trim() == "```rust" {
            in_rust_block = true;
            continue;
        }

        if in_rust_block && line.trim() == "```" {
            break;
        }

        if in_rust_block {
            rust_code_lines.push(line);
        }
    }

    if rust_code_lines.is_empty() {
        None
    } else {
        Some(rust_code_lines.join("\n"))
    }
}
