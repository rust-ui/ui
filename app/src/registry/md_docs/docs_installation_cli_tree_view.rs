use app_components::{FileRendererHighlight, Folder, TARGET_FILE_RENDERER_HIGHLIGHT, Tree};
use leptos::prelude::*;
use markdown_crate::highlight_language::HighlightLanguage;
use markdown_crate::syntect_highlighter_code::SyntectHighlighterCode;
use registry::ui::card::{Card, CardContent};

#[component]
pub fn DocsInstallationCliTreeView() -> impl IntoView {
    view! {
        <div class="flex gap-4 w-full max-w-4xl">
            <Tree>
                <Folder name="src" open=true>
                    <FileRendererHighlight name="main.rs" content=MAIN_RS language="rust" />
                    <Folder name="components">
                        <FileRendererHighlight name="mod.rs" content="// Component exports" language="rust" />
                    </Folder>
                </Folder>

                <Folder name="style" open=true>
                    <FileRendererHighlight name="tailwind.css" content=TAILWIND_CSS language="css" />
                </Folder>

                <FileRendererHighlight name="Cargo.toml" content=CARGO_TOML language="toml" />
                <FileRendererHighlight name="package.json" content=PACKAGE_JSON checked=true language="json" />
            </Tree>

            <Card class="flex-1">
                <CardContent>
                    <div id=TARGET_FILE_RENDERER_HIGHLIGHT>
                        <h3 class="mb-2 font-semibold">package.json</h3>
                        <SyntectHighlighterCode code=PACKAGE_JSON language=HighlightLanguage::Json />
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const MAIN_RS: &str = r#"use leptos::prelude::*;

mod components;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div class="p-8">
                <h1>"Hello, Leptos!"</h1>
            </div>
        }
    });
}"#;

const CARGO_TOML: &str = r#"[package]
name = "my-leptos-app"
version = "0.1.0"
edition = "2024"

[dependencies]
leptos = { features = ["csr"] }
tw_merge = { features = ["variant"] }
icons = { features = ["leptos"] }
leptos_ui = {}
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

:root {
  --radius: 0.625rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.145 0 0);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.145 0 0);
  --primary: oklch(0.205 0 0);
  --primary-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.97 0 0);
  --secondary-foreground: oklch(0.205 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --accent: oklch(0.97 0 0);
  --accent-foreground: oklch(0.205 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);
  --ring: oklch(0.708 0 0);
}

.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.205 0 0);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.205 0 0);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.922 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.556 0 0);
}


@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --radius-sm: calc(var(--radius) - 4px);
  --radius-md: calc(var(--radius) - 2px);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 4px);
}

@layer base {
  * {
    @apply border-border outline-ring/50;
  }
  body {
    @apply bg-background text-foreground;
  }

  button:not(:disabled),
  [role="button"]:not(:disabled) {
    cursor: pointer;
  }

  dialog {
    margin: auto;
  }
}
"#;
