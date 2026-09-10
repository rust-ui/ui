use dioxus::prelude::*;

use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperOrientation, StepperSeparator, StepperTitle,
    StepperTrigger,
};

#[component]
pub fn DemoStepperVertical() -> Element {
    rsx! {
        Stepper {
            total_steps: 3,
            default_step: 1,
            orientation: StepperOrientation::Vertical,
            class: "w-full max-w-xs",
            StepperItem { step: 0,
                StepperTrigger {
                    StepperIndicator {}
                    div { class: "flex flex-col gap-0.5",
                        StepperTitle { "Account" }
                        StepperDescription { "Create your account" }
                    }
                }
                StepperSeparator {}
            }
            StepperItem { step: 1,
                StepperTrigger {
                    StepperIndicator {}
                    div { class: "flex flex-col gap-0.5",
                        StepperTitle { "Profile" }
                        StepperDescription { "Complete your profile" }
                    }
                }
                StepperSeparator {}
            }
            StepperItem { step: 2,
                StepperTrigger {
                    StepperIndicator {}
                    div { class: "flex flex-col gap-0.5",
                        StepperTitle { "Confirmation" }
                        StepperDescription { "Review and confirm" }
                    }
                }
            }
        }
    }
}
