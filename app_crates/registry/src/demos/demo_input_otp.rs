use dioxus::prelude::*;

use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

#[component]
pub fn DemoInputOtp() -> Element {
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
}
