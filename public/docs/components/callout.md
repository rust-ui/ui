+++
title = "Callout"
description = "A styled alert block for docs and rich content, with Default, Info, and Warning variants."
tags = ["utils"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++




<StaticCallout />




## Installation

<StaticInstallCallout />



## Usage

Use the `Callout` component to highlight notes, tips, and warnings inside documentation or content pages.

<Callout title="Markdown support">
Callout can be used directly inside `.md` files — no wrapping component needed.
</Callout>

```rust
use crate::components::ui::callout::{Callout, CalloutVariant};
```

```rust
<Callout title="Note">
    "You can add components using the CLI."
</Callout>
```



## Examples

### Info

Highlight helpful information with a blue accent. Use `CalloutVariant::Info` for tips or non-critical context.

<StaticCalloutInfo />


### Warning

Draw attention to breaking changes or destructive actions. Use `CalloutVariant::Warning` for important caveats.

<Callout title="Breaking change" variant="warning">
The `--surface` token was added in March 2026. Update your theme if you are using a custom token set.
</Callout>

<StaticCalloutWarning />



## See Also

- [Alert](/docs/components/alert)
- [Badge](/docs/components/badge)
