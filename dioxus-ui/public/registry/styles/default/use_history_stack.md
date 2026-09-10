---
title: "Use History Stack"
name: "use_history_stack"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_history_stack.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use History Stack

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_history_stack
```

## Component Code

```rust
use dioxus::prelude::*;

pub struct UseHistoryStack<T: Clone + 'static> {
    history: Signal<Vec<T>>,
    index: Signal<usize>,
}

// Signal<T> is Copy regardless of T — derived impls would add T: Copy/PartialEq bounds unnecessarily.
impl<T: Clone + 'static> Copy for UseHistoryStack<T> {}
impl<T: Clone + 'static> Clone for UseHistoryStack<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Clone + 'static> PartialEq for UseHistoryStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.history == other.history && self.index == other.index
    }
}

impl<T: Clone + 'static> UseHistoryStack<T> {
    pub fn new(initial: T) -> Self {
        Self { history: use_signal(|| vec![initial]), index: use_signal(|| 0) }
    }

    pub fn push(&mut self, state: T) {
        let idx = *self.index.read();
        self.history.write().truncate(idx + 1);
        self.history.write().push(state);
        *self.index.write() = idx + 1;
    }

    pub fn undo(&mut self) -> Option<T> {
        let idx = *self.index.read();
        if idx == 0 {
            return None;
        }
        *self.index.write() = idx - 1;
        self.history.read().get(idx - 1).cloned()
    }

    pub fn redo(&mut self) -> Option<T> {
        let idx = *self.index.read();
        let len = self.history.read().len();
        if idx + 1 >= len {
            return None;
        }
        *self.index.write() = idx + 1;
        self.history.read().get(idx + 1).cloned()
    }

    pub fn can_undo(&self) -> bool {
        *self.index.read() > 0
    }

    pub fn can_redo(&self) -> bool {
        let idx = *self.index.read();
        idx + 1 < self.history.read().len()
    }
}
```
