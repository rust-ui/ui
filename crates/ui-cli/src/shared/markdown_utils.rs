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

    if rust_code_lines.is_empty() { None } else { Some(rust_code_lines.join("\n")) }
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_when_no_rust_block() {
        assert_eq!(extract_rust_code_from_markdown("just prose\nno code"), None);
    }

    #[test]
    fn returns_none_for_empty_input() {
        assert_eq!(extract_rust_code_from_markdown(""), None);
    }

    #[test]
    fn returns_none_for_empty_rust_block() {
        assert_eq!(extract_rust_code_from_markdown("```rust\n```"), None);
    }

    #[test]
    fn extracts_code_from_rust_block() {
        let md = "# Title\n\n```rust\nfn main() {}\n```\n\nsome prose";
        assert_eq!(extract_rust_code_from_markdown(md), Some("fn main() {}".to_string()));
    }

    #[test]
    fn returns_only_first_rust_block() {
        let md = "```rust\nfn first() {}\n```\n```rust\nfn second() {}\n```";
        assert_eq!(extract_rust_code_from_markdown(md), Some("fn first() {}".to_string()));
    }

    #[test]
    fn ignores_non_rust_fenced_blocks() {
        assert_eq!(extract_rust_code_from_markdown("```toml\nkey = \"value\"\n```"), None);
    }

    #[test]
    fn preserves_multiline_code() {
        let md = "```rust\nuse leptos::*;\n\nfn foo() {}\n```";
        let result = extract_rust_code_from_markdown(md).unwrap();
        assert!(result.contains("use leptos::*;"));
        assert!(result.contains("fn foo() {}"));
    }
}
