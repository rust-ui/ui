+++
title = "Stepper"
description = "Rust/UI component that displays a set of steps for multi-step workflows like onboarding, checkout, and setup wizards."
tags = ["navigation"]
is_new = true
image = "/images/thumbnails/stepper.webp"
image_dark = "/images/thumbnails/stepper-dark.webp"
+++

<StaticStepper />

## Installation

<StaticInstallStepper />

## Usage

```rust
use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperSeparator, StepperTitle, StepperTrigger,
};
```

```rust
rsx! {
    Stepper { total_steps: 3, default_step: 0,
        StepperItem { step: 0,
            StepperTrigger {
                StepperIndicator {}
                StepperTitle { "Account" }
            }
            StepperSeparator {}
        }
    }
}
```

## Examples

### Vertical

Use the `orientation` prop to switch to a vertical layout.

<StaticStepperVertical />

### Controlled

Read the shared `StepperContext` from context to drive external navigation controls.

<StaticStepperControlled />

## See Also

- [Progress](/components/progress)
- [Tabs](/components/tabs)
