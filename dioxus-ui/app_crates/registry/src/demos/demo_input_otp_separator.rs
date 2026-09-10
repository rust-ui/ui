use dioxus::prelude::*;

use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot};

#[component]
pub fn DemoInputOtpSeparator() -> Element {
    rsx! {
        InputOTP { max_length: 6,
            InputOTPGroup {
                InputOTPSlot { index: 0 }
                InputOTPSlot { index: 1 }
            }
            InputOTPSeparator {}
            InputOTPGroup {
                InputOTPSlot { index: 2 }
                InputOTPSlot { index: 3 }
            }
            InputOTPSeparator {}
            InputOTPGroup {
                InputOTPSlot { index: 4 }
                InputOTPSlot { index: 5 }
            }
        }
    }
}
