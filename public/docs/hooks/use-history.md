+++
title = "Use History"
description = "Undo/redo history stack for URL-based state, with keyboard shortcuts (⌘Z / ⌘⇧Z)."
tags = ["utils"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticUseHistory />



## Installation

<StaticInstallUseHistory />



## Components

- **`UseHistory::init()`**: Initialize the stack at the page root — seeds with current URL and registers keyboard shortcuts
- **`use_history()`**: Access the context from any child component



## Usage

```rust
use crate::components::hooks::use_history::{UseHistory, use_history};
```

```rust
// In the page component (top level):
let _ = UseHistory::init();

// In any child:
let history = use_history();
history.push("?color=red".to_string());

view! {
    <button on:click=move |_| history.go_back()>"Undo"</button>
    <button on:click=move |_| history.go_forward()>"Redo"</button>
}
```



## API

| Method | Returns | Description |
|--------|---------|-------------|
| `push(url)` | `()` | Append a URL, truncating any forward history |
| `go_back()` | `()` | Navigate one step back |
| `go_forward()` | `()` | Navigate one step forward |
| `can_go_back()` | `Signal<bool>` | `true` when undo is available |
| `can_go_forward()` | `Signal<bool>` | `true` when redo is available |
| `position()` | `Signal<usize>` | Current 1-based position in stack |
| `total()` | `Signal<usize>` | Total number of states |



## Examples

### Default

Click a color to push it onto the history stack. Use the buttons or keyboard shortcuts to undo/redo.

<StaticUseHistory />



## See Also

- [Use Media Query](/docs/hooks/use-media-query)
- [Use Is Mobile](/docs/hooks/use-is-mobile)
