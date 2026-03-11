+++
title = "Alert"
description = "Rust/UI component that displays a callout to the user."
tags = []
is_new = false
image = "/images/thumbnails/alert.webp"
image_dark = "/images/thumbnails/alert-dark.webp"
+++



<StaticAlert />








## Installation

<StaticInstallAlert />




## Components

The Alert component is composed of several subcomponents:

- **Alert**: Main alert wrapper component with variant styling
- **AlertTitle**: Primary heading text for the alert
- **AlertDescription**: Descriptive text explaining the alert message



## Usage

```rust
use crate::components::ui::alert::{
    Alert,
    AlertDescription,
    AlertTitle,
};
```

```rust
<Alert>
    <AlertTitle>"Alert Title"</AlertTitle>
    <AlertDescription>"Alert description goes here"</AlertDescription>
</Alert>
```


## Examples

### Alert with Custom Colors

Alert with semantic color overrides using Tailwind classes. No variant prop needed — pass custom background, border, and text colors directly via `class` to express warning, success, info, or error states.

<StaticAlertColors />



### RTL

Alert components with Arabic copy. Icon placement and text alignment reverse to match the right-to-left reading flow.

<StaticAlertRtl />

## See Also

- [Alert Dialog](/docs/components/alert-dialog)
- [Badge](/docs/components/badge)
- [Toast](/docs/components/toast)
- [Sonner](/docs/components/sonner)
