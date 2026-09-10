---
title: "Use Is Mobile"
name: "use_is_mobile"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_is_mobile.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
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
use dioxus::prelude::*;

use super::use_media_query::use_media_query;

pub const MOBILE_BREAKPOINT: u32 = 768;

pub fn use_is_mobile() -> ReadSignal<bool> {
    use_media_query(&format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1))
}
```
