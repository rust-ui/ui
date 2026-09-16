//! Regex-based Tailwind design-system linter, config-driven by `lint_policy.toml`.
//! Scans `class: "..."` literals in `rsx!` blocks and `tw_merge!` base classes.
//!
//! Rules and policy shape adapted from <https://github.com/shadcn-ui/lint>
//! (an ESLint/Oxlint plugin for React/JSX) to a plain regex scan over Rust
//! source, since there's no JSX AST here. Same categories/no-restyle/
//! no-arbitrary-values/no-raw-colors concepts, different implementation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Policy {
    categories: HashMap<String, Vec<String>>,
    no_restyle: NoRestyle,
    no_arbitrary_values: Toggle,
    no_raw_colors: RawColors,
}

#[derive(Deserialize)]
struct Toggle {
    enabled: bool,
}

#[derive(Deserialize)]
struct RawColors {
    enabled: bool,
    allowed_tokens: Vec<String>,
}

#[derive(Deserialize)]
struct NoRestyle {
    allow: Vec<String>,
    #[serde(default)]
    contracts: Vec<Contract>,
}

#[derive(Deserialize)]
struct Contract {
    pattern: String,
    allow: Vec<String>,
}

struct Finding {
    file: PathBuf,
    line: usize,
    rule: &'static str,
    message: String,
}

fn main() -> ExitCode {
    let policy_path = Path::new("lint_policy.toml");
    let target = Path::new("app_crates/registry/src");

    let policy_src = match fs::read_to_string(policy_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("failed to read {}: {err}", policy_path.display());
            return ExitCode::FAILURE;
        }
    };
    let policy: Policy = match toml::from_str(&policy_src) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("failed to parse {}: {err}", policy_path.display());
            return ExitCode::FAILURE;
        }
    };

    let base_re = match Regex::new(r#"tw_merge!\(\s*"([^"]*)""#) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("bad base-class regex: {err}");
            return ExitCode::FAILURE;
        }
    };
    let call_re = match Regex::new(r#"\b([A-Z][A-Za-z0-9_]*)\s*\{[^{}]{0,400}?class:\s*"([^"]*)""#) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("bad call-site regex: {err}");
            return ExitCode::FAILURE;
        }
    };

    let mut findings = Vec::new();
    for file in collect_rs_files(target) {
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };
        lint_file(&file, &content, &policy, &base_re, &call_re, &mut findings);
    }

    if findings.is_empty() {
        println!("tailwind lint: no violations found");
        return ExitCode::SUCCESS;
    }

    for f in &findings {
        println!("{}:{}: [{}] {}", f.file.display(), f.line, f.rule, f.message);
    }
    println!("\ntailwind lint: {} violation(s)", findings.len());
    ExitCode::FAILURE
}

fn collect_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(collect_rs_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out
}

fn line_of(content: &str, byte_offset: usize) -> usize {
    content.get(..byte_offset).map_or(1, |s| s.matches('\n').count() + 1)
}

fn classify<'a>(policy: &'a Policy, class: &str) -> Option<&'a str> {
    // Longest matching prefix wins, so `text-sm` (typography) beats the
    // broader `text-` (color) prefix instead of racing on HashMap order.
    policy
        .categories
        .iter()
        .filter_map(|(category, prefixes)| {
            prefixes
                .iter()
                .filter(|p| class.starts_with(p.as_str()))
                .map(String::len)
                .max()
                .map(|len| (len, category.as_str()))
        })
        .max_by_key(|(len, _)| *len)
        .map(|(_, category)| category)
}

fn resolve_allow<'a>(no_restyle: &'a NoRestyle, component: &str) -> &'a [String] {
    for contract in &no_restyle.contracts {
        let Ok(re) = Regex::new(&contract.pattern) else {
            continue;
        };
        if re.is_match(component) {
            return &contract.allow;
        }
    }
    &no_restyle.allow
}

#[allow(clippy::too_many_arguments)]
fn check_classes(
    file: &Path,
    line: usize,
    classes: &str,
    policy: &Policy,
    allow: Option<&[String]>,
    findings: &mut Vec<Finding>,
) {
    for class in classes.split_whitespace() {
        if policy.no_arbitrary_values.enabled && class.contains('[') && class.contains(']') {
            findings.push(Finding {
                file: file.to_path_buf(),
                line,
                rule: "no-arbitrary-values",
                message: format!("`{class}` uses an arbitrary value, add a design token instead"),
            });
        }

        let category = classify(policy, class);

        if policy.no_raw_colors.enabled
            && category == Some("color")
            && !policy.no_raw_colors.allowed_tokens.iter().any(|t| class.contains(t.as_str()))
        {
            findings.push(Finding {
                file: file.to_path_buf(),
                line,
                rule: "no-raw-colors",
                message: format!(
                    "`{class}` is a raw color, use a theme token (bg-primary, text-muted-foreground, ...)"
                ),
            });
        }

        if let (Some(allow), Some(cat)) = (allow, category) {
            if !allow.iter().any(|a| a == cat) {
                findings.push(Finding {
                    file: file.to_path_buf(),
                    line,
                    rule: "no-restyle",
                    message: format!("`{class}` ({cat}) not allowed on this component from call site"),
                });
            }
        }
    }
}

fn lint_file(
    file: &Path,
    content: &str,
    policy: &Policy,
    base_re: &Regex,
    call_re: &Regex,
    findings: &mut Vec<Finding>,
) {
    for caps in base_re.captures_iter(content) {
        let Some(m) = caps.get(1) else { continue };
        let line = line_of(content, m.start());
        check_classes(file, line, m.as_str(), policy, None, findings);
    }

    for caps in call_re.captures_iter(content) {
        let (Some(component), Some(classes)) = (caps.get(1), caps.get(2)) else {
            continue;
        };
        let allow = resolve_allow(&policy.no_restyle, component.as_str());
        let line = line_of(content, classes.start());
        check_classes(file, line, classes.as_str(), policy, Some(allow), findings);
    }
}
