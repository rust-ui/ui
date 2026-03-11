use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

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
