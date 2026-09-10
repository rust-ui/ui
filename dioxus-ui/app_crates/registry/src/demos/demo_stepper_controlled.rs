use dioxus::prelude::*;

use crate::hooks::use_stepper::StepperContext;
use crate::ui::button::Button;
use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperSeparator, StepperTitle, StepperTrigger,
};

#[component]
fn StepperControls() -> Element {
    let ctx = use_context::<StepperContext>();

    rsx! {
        div { class: "flex gap-2 justify-end mt-6 w-full basis-full",
            Button { onclick: move |_| ctx.go_prev(), disabled: !ctx.can_go_prev(), "Previous" }
            Button { onclick: move |_| ctx.go_next(), disabled: !ctx.can_go_next(), "Next" }
        }
    }
}

#[component]
pub fn DemoStepperControlled() -> Element {
    rsx! {
        Stepper { total_steps: 3, class: "w-full max-w-md",
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
            StepperControls {}
        }
    }
}
