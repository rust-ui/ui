+++
title = "Input OTP"
description = "Rust/UI component that displays an OTP input."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticInputOtp />

## Installation

<StaticInstallInputOtp />

## Usage

```rust
use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot, InputOTPSeparator};
```

```rust
rsx! {
    InputOTP { max_length: 6,
        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
            InputOTPSlot { index: 3 }
            InputOTPSlot { index: 4 }
            InputOTPSlot { index: 5 }
        }
    }
}
```

## Examples

### With Separator

<StaticInputOtpSeparator />

## See Also

- [Input](/components/input)
- [Field](/components/field)
