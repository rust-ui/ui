---
title: "Use Random"
name: "use_random"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_random.rs"
---

# Use Random

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_random
```

## Component Code

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

const PREFIX: &str = "rust_ui"; // Must NOT contain "/" or "-"

pub fn use_random_id() -> String {
    format!("_{PREFIX}_{}", generate_hash())
}

pub fn use_random_id_for(element: &str) -> String {
    format!("{}_{PREFIX}_{}", element, generate_hash())
}

pub fn use_random_transition_name() -> String {
    let random_id = use_random_id();
    format!("view-transition-name: {random_id}")
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn generate_hash() -> u64 {
    let mut hasher = DefaultHasher::new();
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    counter.hash(&mut hasher);
    hasher.finish()
}
```
