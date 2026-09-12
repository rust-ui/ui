use std::fmt::Write as _;

#[must_use]
pub fn highlight_toml_manually(code: &str) -> String {
    let mut html_output = String::new();

    for line in code.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            html_output.push_str(&html_escape::encode_text(line));
            html_output.push('\n');
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let _ = writeln!(
                html_output,
                "<span class=\"syntax__keyword\">{}</span>",
                html_escape::encode_text(line)
            );
        } else if trimmed.starts_with('#') {
            let _ = writeln!(
                html_output,
                "<span class=\"syntax__comment\">{}</span>",
                html_escape::encode_text(line)
            );
        } else if trimmed.contains('=') {
            if let Some((key_part, value_part)) = line.split_once('=') {
                html_output.push_str(&html_escape::encode_text(key_part));
                html_output.push('=');

                let value_trimmed = value_part.trim();
                if (value_trimmed.starts_with('"') && value_trimmed.ends_with('"'))
                    || (value_trimmed.starts_with('\'') && value_trimmed.ends_with('\''))
                    || value_trimmed.contains('"')
                {
                    let _ = writeln!(
                        html_output,
                        "<span class=\"syntax__string\">{}</span>",
                        html_escape::encode_text(value_part)
                    );
                } else {
                    html_output.push_str(&html_escape::encode_text(value_part));
                    html_output.push('\n');
                }
            } else {
                html_output.push_str(&html_escape::encode_text(line));
                html_output.push('\n');
            }
        } else {
            html_output.push_str(&html_escape::encode_text(line));
            html_output.push('\n');
        }
    }

    if html_output.ends_with('\n') {
        html_output.pop();
    }

    html_output
}
