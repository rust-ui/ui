+++
title = "Callout"
description = "A styled alert block for docs and rich content, with Default, Info, and Warning variants."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticCallout />

## Installation

<StaticInstallCallout />

## Usage

```rust
use crate::ui::callout::Callout;
```

```rust
rsx! {
    Callout { title: "Note", "Your message here." }
}
```

## Examples

### Info

<StaticCalloutInfo />

### Warning

<StaticCalloutWarning />

## See Also

- [Alert](/components/alert)
- [Badge](/components/badge)
