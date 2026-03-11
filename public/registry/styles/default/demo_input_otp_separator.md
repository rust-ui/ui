---
title: "Demo Input Otp Separator"
name: "demo_input_otp_separator"
cargo_dependencies: []
registry_dependencies: ["input_otp"]
type: "components:demos"
path: "demos/demo_input_otp_separator.rs"
---

# Demo Input Otp Separator

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_otp_separator
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot};

#[component]
pub fn DemoInputOtpSeparator() -> impl IntoView {
    view! {
        <InputOTP max_length=6>
            <InputOTPGroup>
                <InputOTPSlot index=0 />
                <InputOTPSlot index=1 />
            </InputOTPGroup>
            <InputOTPSeparator />
            <InputOTPGroup>
                <InputOTPSlot index=2 />
                <InputOTPSlot index=3 />
            </InputOTPGroup>
            <InputOTPSeparator />
            <InputOTPGroup>
                <InputOTPSlot index=4 />
                <InputOTPSlot index=5 />
            </InputOTPGroup>
        </InputOTP>
    }
}
```
