use dioxus::prelude::*;
use registry::ui::card::{Card, CardContent};

use crate::markdown::highlight_code::highlight_code;

const MAIN_RS: &str = r#"use dioxus::prelude::*;

mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "p-8",
            h1 { "Hello, Dioxus!" }
        }
    }
}"#;

const CARGO_TOML: &str = r#"[package]
name = "my-dioxus-app"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { version = "0.7.0-rc.0", features = ["web"] }
tw_merge = { features = ["variant"] }
icons = { features = ["dioxus"] }
registry = {}
"#;

const PACKAGE_JSON: &str = r#"{
  "type": "module",
  "dependencies": {
    "@tailwindcss/cli": "^4.1.13",
    "tailwindcss": "^4.1.13",
    "tw-animate-css": "^1.3.8"
  }
}"#;

const TAILWIND_CSS: &str = r#"@import "tailwindcss";
@import "tw-animate-css";

@layer base {
  * {
    @apply border-border outline-ring/50;
  }

  body {
    @apply bg-background text-foreground;
  }
}"#;

#[component]
pub fn DocsInstallationCliTreeView() -> Element {
    let mut selected = use_signal(|| "package.json".to_string());

    let file_content = move || match selected().as_str() {
        "main.rs" => ("main.rs", highlight_code(MAIN_RS, Some("rust"), None)),
        "tailwind.css" => ("tailwind.css", highlight_code(TAILWIND_CSS, Some("css"), None)),
        "Cargo.toml" => ("Cargo.toml", highlight_code(CARGO_TOML, Some("toml"), None)),
        _ => ("package.json", highlight_code(PACKAGE_JSON, Some("json"), None)),
    };

    let file_button = |name: &'static str, indent: &'static str| {
        rsx! {
            button {
                class: "w-full rounded-md px-2 py-1 text-left text-sm hover:bg-muted",
                onclick: move |_| selected.set(name.to_string()),
                span { class: "font-mono text-xs text-muted-foreground", "{indent}" }
                span { "{name}" }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full lg:flex-row lg:items-start",
            div { class: "w-full rounded-xl border p-3 lg:max-w-[280px]",
                p { class: "mb-2 text-sm font-semibold", "Project tree" }
                div { class: "space-y-1",
                    p { class: "px-2 py-1 text-sm font-medium", "src/" }
                    {file_button("main.rs", "  ")}
                    p { class: "px-2 py-1 text-sm font-medium", "style/" }
                    {file_button("tailwind.css", "  ")}
                    {file_button("Cargo.toml", "")}
                    {file_button("package.json", "")}
                }
            }

            Card { class: "flex-1",
                CardContent {
                    {
                        let (name, highlighted) = file_content();
                        rsx! {
                            h3 { class: "mb-2 font-semibold", "{name}" }
                            pre {
                                class: "overflow-x-auto rounded-xl bg-muted py-3.5 px-4 text-xs",
                                dangerous_inner_html: "{highlighted}",
                            }
                        }
                    }
                }
            }
        }
    }
}
