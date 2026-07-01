+++
title = "Attachment"
description = "Displays a file or image attachment with media, name, metadata, and optional actions. Use it for files and images in chat composers, message threads, and upload lists."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticAttachment />




## Installation

<StaticInstallAttachment />




## Usage

```rust
use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions,
    AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentTitle, AttachmentTrigger,
};
```

```rust
<Attachment>
    <AttachmentMedia>
        <FileText />
    </AttachmentMedia>
    <AttachmentContent>
        <AttachmentTitle>"sales-dashboard.pdf"</AttachmentTitle>
        <AttachmentDescription>"PDF · 2.4 MB"</AttachmentDescription>
    </AttachmentContent>
    <AttachmentActions>
        <AttachmentAction attr:aria-label="Remove sales-dashboard.pdf">
            <X />
        </AttachmentAction>
    </AttachmentActions>
</Attachment>
```

## Composition

```
Attachment
├── AttachmentMedia        (icon or image)
├── AttachmentContent
│   ├── AttachmentTitle
│   └── AttachmentDescription
├── AttachmentActions
│   └── AttachmentAction   (ghost icon button, icon-xs by default)
└── AttachmentTrigger      (absolute overlay — href → <a>, on_click → <button>)
```

Wrap multiple attachments in `AttachmentGroup` for a horizontally scrollable snap row.

## Examples

### Image

Set `variant=AttachmentMediaVariant::Image` on `AttachmentMedia` and render an `<img>` inside it. Use `orientation=AttachmentOrientation::Vertical` to stack the media above the content.

<StaticAttachmentImage />

### States

Use the `state` prop on `Attachment` to reflect upload progress. Available states: `Idle`, `Uploading`, `Processing`, `Error`, `Done`.

<StaticAttachmentStates />

### Sizes

Use the `size` prop to control padding and icon sizing. Available: `Default`, `Sm`, `Xs`.

<StaticAttachmentSizes />

### Group

Wrap attachments in `AttachmentGroup` to lay them out in a horizontally scrollable, snapping row.

<StaticAttachmentGroup />

### Trigger

Add `AttachmentTrigger` inside an `Attachment` to make the whole card clickable. Pass `href` to render as an `<a>`, or `on_click` for a button trigger. Actions inside `AttachmentActions` remain independently clickable above the trigger.

<StaticAttachmentTrigger />

## See Also

- [Bubble](/docs/components/bubble)
- [Message](/docs/components/message)
