+++
title = "Theme Toggle"
description = "A sleek theme toggle component that smoothly transitions between light and dark modes with animated sun and moon icons."
tags = ["button", "animation"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticThemeToggle />

## Installation

<StaticInstallThemeToggle />

## Usage

```rust
use crate::ui::theme_toggle::ThemeToggle;
```

```rust
rsx! {
    ThemeToggle {}
}
```

## How it works

On mount it reads `localStorage.getItem('darkmode')`, falling back to the system `prefers-color-scheme` preference. On toggle it adds/removes the `.dark` class on `<html>` and persists the choice.

## See Also

- [Button](/components/button)
