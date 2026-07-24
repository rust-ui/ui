use leptos::prelude::*;

use crate::hooks::use_stepper::StepperContext;
use crate::ui::button::Button;
use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperSeparator, StepperTitle, StepperTrigger,
};

#[component]
fn StepperControls() -> impl IntoView {
    let ctx = expect_context::<StepperContext>();

    view! {
        <div class="flex gap-2 justify-end mt-6">
            <Button on:click=move |_| ctx.go_prev.run(()) attr:disabled=move || !ctx.can_go_prev.get()>
                "Previous"
            </Button>
            <Button on:click=move |_| ctx.go_next.run(()) attr:disabled=move || !ctx.can_go_next.get()>
                "Next"
            </Button>
        </div>
    }
}

#[component]
pub fn DemoStepperControlled() -> impl IntoView {
    view! {
        <Stepper total_steps=3 class="w-full max-w-md">
            <StepperItem step=0>
                <StepperTrigger>
                    <StepperIndicator />
                    <div class="flex flex-col gap-0.5">
                        <StepperTitle>"Account"</StepperTitle>
                        <StepperDescription>"Create your account"</StepperDescription>
                    </div>
                </StepperTrigger>
                <StepperSeparator />
            </StepperItem>
            <StepperItem step=1>
                <StepperTrigger>
                    <StepperIndicator />
                    <div class="flex flex-col gap-0.5">
                        <StepperTitle>"Profile"</StepperTitle>
                        <StepperDescription>"Complete your profile"</StepperDescription>
                    </div>
                </StepperTrigger>
                <StepperSeparator />
            </StepperItem>
            <StepperItem step=2>
                <StepperTrigger>
                    <StepperIndicator />
                    <div class="flex flex-col gap-0.5">
                        <StepperTitle>"Confirmation"</StepperTitle>
                        <StepperDescription>"Review and confirm"</StepperDescription>
                    </div>
                </StepperTrigger>
            </StepperItem>
            <StepperControls />
        </Stepper>
    }
}
