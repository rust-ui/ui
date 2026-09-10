use dioxus::prelude::*;

use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperSeparator, StepperTitle, StepperTrigger,
};

#[component]
pub fn DemoStepper() -> Element {
    rsx! {
        Stepper { total_steps: 3, default_step: 1, class: "w-full max-w-md",
            StepperItem { step: 0,
                StepperTrigger {
                    StepperIndicator {}
                    div { class: "flex flex-col gap-0.5 items-center text-center",
                        StepperTitle { "Account" }
                        StepperDescription { "Create your account" }
                    }
                }
                StepperSeparator {}
            }
            StepperItem { step: 1,
                StepperTrigger {
                    StepperIndicator {}
                    div { class: "flex flex-col gap-0.5 items-center text-center",
                        StepperTitle { "Profile" }
                        StepperDescription { "Complete your profile" }
                    }
                }
                StepperSeparator {}
            }
            StepperItem { step: 2,
                StepperTrigger {
                    StepperIndicator {}
                    div { class: "flex flex-col gap-0.5 items-center text-center",
                        StepperTitle { "Confirmation" }
                        StepperDescription { "Review and confirm" }
                    }
                }
            }
        }
    }
}
