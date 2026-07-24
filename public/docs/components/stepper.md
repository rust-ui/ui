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




## Components

- **Stepper**: Root provider — builds the shared step state from `total_steps`, `default_step`, and `orientation`
- **StepperItem**: Wraps a single step, taking its `step` index and an optional `disabled` flag
- **StepperTrigger**: Clickable native `<button>` that navigates to its step, marks itself `aria-current="step"` when active
- **StepperIndicator**: Visual dot — shows the step number, a checkmark once completed, or fully custom content
- **StepperTitle**: Step label
- **StepperDescription**: Secondary step text
- **StepperSeparator**: Connecting line between steps


## Usage

```rust
use crate::components::ui::stepper::{
    Stepper,
    StepperItem,
    StepperTrigger,
    StepperIndicator,
    StepperTitle,
    StepperDescription,
    StepperSeparator,
};
```

```rust
<Stepper total_steps=3 default_step=1>
    <StepperItem step=0>
        <StepperTrigger>
            <StepperIndicator />
            <StepperTitle>"Account"</StepperTitle>
        </StepperTrigger>
        <StepperSeparator />
    </StepperItem>
    <StepperItem step=1>
        <StepperTrigger>
            <StepperIndicator />
            <StepperTitle>"Profile"</StepperTitle>
        </StepperTrigger>
    </StepperItem>
</Stepper>
```


## Examples

### Default

Step 0 completed, step 1 active, step 2 pending — all three states visible at once via a non-zero `default_step`.

<StaticStepper />

### Controlled navigation

External `Previous`/`Next` buttons drive the same `StepperContext` that `StepperTrigger` uses internally, read from anywhere inside `Stepper` via `expect_context`.

<StaticStepperControlled />

### Vertical

Vertical step list via `orientation=StepperOrientation::Vertical` on `Stepper`.

<StaticStepperVertical />

## See Also

- [Tabs](/docs/components/tabs)
- [Pagination](/docs/components/pagination)
