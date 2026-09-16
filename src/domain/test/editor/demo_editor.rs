use dioxus::prelude::*;

use crate::domain::test::editor::editor::Editor;
use crate::markdown::markdown_to_html;

const DEFAULT_EDITOR_MARKDOWN: &str = r#"# Explore the rich text editor with shadcn-style components 📝

This is a powerful editor that supports many features:

- Rich text formatting with **bold**, *italic*, and <u>underline</u>
- Different heading levels
- Lists (ordered and unordered)
- Text formatting controls
- Content stays editable in place

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
    rsx! {
        div { class: "flex flex-col gap-3",
            Editor {
                initial_html: markdown_to_html(DEFAULT_EDITOR_MARKDOWN),
                placeholder: "Write here...",
            }
        }
    }
}
