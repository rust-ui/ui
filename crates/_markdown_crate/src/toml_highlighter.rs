pub fn highlight_toml_manually(code: &str) -> String {
    let mut html_output = String::new();

    for line in code.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            html_output.push_str(&html_escape::encode_text(line));
            html_output.push('\n');
            continue;
        }

        // Check for section headers like [package], [dependencies]
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            html_output
                .push_str(&format!("<span class=\"syntax__keyword\">{}</span>\n", html_escape::encode_text(line)));
        }
        // Check for comments
        else if trimmed.starts_with('#') {
            html_output
                .push_str(&format!("<span class=\"syntax__comment\">{}</span>\n", html_escape::encode_text(line)));
        }
        // Check for key-value pairs
        else if trimmed.contains('=') {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key_part = parts[0];
                let value_part = parts[1];

                // Highlight the key
                html_output.push_str(&html_escape::encode_text(key_part));
                html_output.push('=');

                // Highlight strings in quotes
                let value_trimmed = value_part.trim();
                if (value_trimmed.starts_with('"') && value_trimmed.ends_with('"'))
                    || (value_trimmed.starts_with('\'') && value_trimmed.ends_with('\''))
                    || value_trimmed.contains('"')
                {
                    html_output.push_str(&format!(
                        "<span class=\"syntax__string\">{}</span>\n",
                        html_escape::encode_text(value_part)
                    ));
                } else {
                    html_output.push_str(&html_escape::encode_text(value_part));
                    html_output.push('\n');
                }
            } else {
                html_output.push_str(&html_escape::encode_text(line));
                html_output.push('\n');
            }
        }
        // Regular text
        else {
            html_output.push_str(&html_escape::encode_text(line));
            html_output.push('\n');
        }
    }

    // Remove trailing newline if it was added
    if html_output.ends_with('\n') {
        html_output.pop();
    }

    html_output
}
