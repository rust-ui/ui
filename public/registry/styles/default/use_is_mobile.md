---
title: "Use Is Mobile"
name: "use_is_mobile"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_is_mobile.rs"
---

# Use Is Mobile

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_is_mobile
```

## Component Code

```rust
use leptos::prelude::*;

use super::use_media_query::use_media_query;

/// Mobile breakpoint in pixels (matches Tailwind's `md` breakpoint).
pub const MOBILE_BREAKPOINT: u32 = 768;

/// Reactive hook that returns `true` when the viewport is below the mobile breakpoint.
///
/// Equivalent to `use_media_query("(max-width: 767px)")`.
///
/// # Example
/// ```ignore
/// let is_mobile = use_is_mobile();
///
/// view! {
///     {move || if is_mobile.get() {
///         view! { <Drawer>...</Drawer> }.into_any()
///     } else {
///         view! { <Dialog>...</Dialog> }.into_any()
///     }}
/// }
/// ```
pub fn use_is_mobile() -> Signal<bool> {
    use_media_query(&format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1))
}
```
