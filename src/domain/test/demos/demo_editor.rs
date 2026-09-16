use dioxus::prelude::*;
use registry::ui::editor::Editor;

const DEFAULT_EDITOR_MARKDOWN: &str = r#"# Explore the rich text editor with shadcn-style components 📝

This is a powerful editor that supports many features:

- Rich text formatting with **bold**, *italic*, and <u>underline</u>
- Different heading levels
- Lists (ordered and unordered)
- Text formatting controls
- HTML output for saving or previewing content

## Try It Out!

Use the toolbar above to format your content.

You can also add links like [this one](https://example.com).

![Placeholder image](https://placehold.co/600x200)

```javascript
// Example code block
const greeting = "Hello, World!";
console.log(greeting);
```"#;

#[component]
pub fn DemoEditor() -> Element {
    let mut html = use_signal(|| crate::markdown::markdown_to_html(DEFAULT_EDITOR_MARKDOWN));

    rsx! {
        div { class: "flex flex-col gap-3",
            Editor {
                initial_html: html(),
                placeholder: "Write here...",
                on_change: move |next| html.set(next),
            }
            div { class: "rounded-md border bg-muted/30 p-3",
                p { class: "mb-2 text-xs font-medium text-muted-foreground", "HTML output" }
                pre { class: "max-h-48 overflow-auto whitespace-pre-wrap break-words text-xs", "{html}" }
            }
        }
    }
}
