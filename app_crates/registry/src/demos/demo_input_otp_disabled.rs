use leptos::prelude::*;

use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot};

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
