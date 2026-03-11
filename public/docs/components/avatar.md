+++
title = "Avatar"
description = "Rust/UI component that displays an avatar with image and fallback support."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticAvatar />




## Installation

<StaticInstallAvatar />




## Components

- **Avatar**: Main wrapper with `size` prop (`Sm` / `Default` / `Lg`)
- **AvatarImage**: Image element with automatic hide-on-error
- **AvatarFallback**: Fallback shown when image fails to load
- **AvatarBadge**: Status dot or icon overlaid on the avatar corner
- **AvatarGroup**: Stacked overlapping row of avatars
- **AvatarGroupCount**: Overflow counter bubble inside an `AvatarGroup`


## Usage

```rust
use crate::components::ui::avatar::{
    Avatar,
    AvatarImage,
    AvatarFallback,
    AvatarBadge,
    AvatarGroup,
    AvatarGroupCount,
};
```

```rust
<Avatar>
    <AvatarImage attr:src="/path/to/image.png" attr:alt="@username" />
    <AvatarFallback>"UN"</AvatarFallback>
</Avatar>
```


## Examples

### Size

Three sizes: `Sm`, `Default`, and `Lg`.

<StaticAvatarSize />

### Badge

Add a status dot by placing `AvatarBadge` inside `Avatar`.

<StaticAvatarBadge />

### Badge with Icon

<StaticAvatarBadgeIcon />

### Group

Overlap multiple avatars with `AvatarGroup`.

<StaticAvatarGroup />

### Group Count

Show an overflow counter with `AvatarGroupCount`.

<StaticAvatarGroupCount />

### Group Count Icon

<StaticAvatarGroupCountIcon />


### RTL

Avatar with an Arabic display name. In grouped usage the stack reads right-to-left automatically.

<StaticAvatarRtl />

## See Also

- [Card](/docs/components/card)
- [Badge](/docs/components/badge)
- [Item](/docs/components/item)
