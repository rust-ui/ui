---
title: "Use Breadcrumb"
name: "use_breadcrumb"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_breadcrumb.rs"
---

# Use Breadcrumb

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_breadcrumb
```

## Component Code

```rust
use heck::ToTitleCase;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

const LAST_SEGMENT_INDEX: usize = 1;

// Custom hook for breadcrumb navigation starting from a specific segment (inclusive)
pub fn use_breadcrumb_from_segment(start_segment: &str) -> Memo<Vec<(String, String, bool)>> {
    build_breadcrumb_items(start_segment, true)
}

// Custom hook for breadcrumb navigation starting after a specific segment (exclusive)
pub fn use_breadcrumb_after_segment(start_segment: &str) -> Memo<Vec<(String, String, bool)>> {
    build_breadcrumb_items(start_segment, false)
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn build_breadcrumb_items(start_segment: &str, inclusive: bool) -> Memo<Vec<(String, String, bool)>> {
    let location = use_location();
    let start_segment = start_segment.to_string();

    Memo::new(move |_| {
        let path = location.pathname.get();
        let segments: Vec<String> = path.split('/').filter(|segment| !segment.is_empty()).map(String::from).collect();

        segments
            .iter()
            .position(|segment| segment == &start_segment)
            .map(|start_idx| {
                let actual_start_idx = if inclusive { start_idx } else { start_idx + 1 };

                if actual_start_idx >= segments.len() {
                    return Vec::new();
                }

                segments
                    .iter()
                    .enumerate()
                    .skip(actual_start_idx)
                    .map(|(i, segment)| {
                        let path = segments.get(..=i).map(|s| s.join("/")).unwrap_or_default();
                        (segment.to_title_case(), format!("/{path}"), i == segments.len() - LAST_SEGMENT_INDEX)
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}
```
