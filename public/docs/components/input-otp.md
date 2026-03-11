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



## Components

The InputOTP component is composed of several subcomponents:

- **InputOTP**: Main wrapper component managing OTP input state
- **InputOTPSlot**: Individual digit input slot



## Usage

```rust
use crate::components::ui::input_otp::{InputOTP, InputOTPSlot};
```

```rust
<InputOTP>
    <InputOTPSlot />
    <InputOTPSlot />
    <InputOTPSlot />
    <InputOTPSlot />
</InputOTP>
```

### RTL

OTP input in an RTL context. Digit slots maintain their entry order while the surrounding label and layout adapt to right-to-left.

<StaticInputOtpRtl />

## See Also

- [Input](/docs/components/input)
- [Form](/docs/components/form)
