---
title: "Toast Wrapper"
name: "toast_wrapper"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui/toast_custom"
path: "ui/toast_wrapper.rs"
---

# Toast Wrapper

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add toast_wrapper
```

## Component Code

```rust
use super::_builder::ToastBuilder;
use super::_data::{ToastLevel, ToastPosition};
use super::toaster::expect_toaster;

// TODO. The default poition is already BottomRight in the builder.
const DEFAULT_POSITION: ToastPosition = ToastPosition::BottomRight;

#[allow(dead_code)]
pub struct ToastWrapper {
    message: String,
    level: Option<ToastLevel>,
    position: ToastPosition,
}

pub fn show_toast() -> ToastWrapper {
    ToastWrapper { message: String::new(), level: None, position: DEFAULT_POSITION }
}

impl ToastWrapper {
    pub fn success(self, message: impl Into<String>) {
        let toaster = expect_toaster();
        toaster.toast(ToastBuilder::new(message.into()).with_level(ToastLevel::Success).with_position(self.position));
    }

    pub fn error(self, message: impl Into<String>) {
        let toaster = expect_toaster();
        toaster.toast(ToastBuilder::new(message.into()).with_level(ToastLevel::Error).with_position(self.position));
    }

    pub fn info(self, message: impl Into<String>) {
        let toaster = expect_toaster();
        toaster.toast(ToastBuilder::new(message.into()).with_level(ToastLevel::Info).with_position(self.position));
    }

    pub fn warning(self, message: impl Into<String>) {
        let toaster = expect_toaster();
        toaster.toast(ToastBuilder::new(message.into()).with_level(ToastLevel::Warn).with_position(self.position));
    }

    /// Override the default position
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }
}
```
