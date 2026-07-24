use leptos::prelude::*;

use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperOrientation, StepperSeparator, StepperTitle,
    StepperTrigger,
};

#[component]
pub fn DemoStepperVertical() -> impl IntoView {
    view! {
        <Stepper total_steps=3 default_step=1 orientation=StepperOrientation::Vertical class="w-full max-w-xs">
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
        </Stepper>
    }
}
