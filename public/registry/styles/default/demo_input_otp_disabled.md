---
title: "Demo Input Otp Disabled"
name: "demo_input_otp_disabled"
cargo_dependencies: []
registry_dependencies: ["input_otp"]
type: "components:demos"
path: "demos/demo_input_otp_disabled.rs"
---

# Demo Input Otp Disabled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_otp_disabled
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot};

#[component]
pub fn DemoInputOtpDisabled() -> impl IntoView {
    view! {
        <InputOTP max_length=6 disabled=true value="123456">
            <InputOTPGroup>
                <InputOTPSlot index=0 />
                <InputOTPSlot index=1 />
                <InputOTPSlot index=2 />
            </InputOTPGroup>
            <InputOTPSeparator />
            <InputOTPGroup>
                <InputOTPSlot index=3 />
                <InputOTPSlot index=4 />
                <InputOTPSlot index=5 />
            </InputOTPGroup>
        </InputOTP>
    }
}
```
