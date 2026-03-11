---
title: "Demo Input Otp Rtl"
name: "demo_input_otp_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "input_otp"]
type: "components:demos"
path: "demos/demo_input_otp_rtl.rs"
---

# Demo Input Otp Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_otp_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

#[component]
pub fn DemoInputOtpRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="flex flex-col gap-4 items-center w-full max-w-sm">
            <p class="text-sm text-muted-foreground">"أدخل رمز التحقق المرسل إلى هاتفك"</p>
            <InputOTP max_length=6>
                <InputOTPGroup>
                    <InputOTPSlot index=0 />
                    <InputOTPSlot index=1 />
                    <InputOTPSlot index=2 />
                    <InputOTPSlot index=3 />
                    <InputOTPSlot index=4 />
                    <InputOTPSlot index=5 />
                </InputOTPGroup>
            </InputOTP>
        </DirectionProvider>
    }
}
```
