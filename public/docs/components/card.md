+++
title = "Card"
description = "Rust/UI component that displays a card with header, content and footer."
tags = ["card"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticCard />

## Installation

<StaticInstallCard />

## Components

The Card component is composed of several subcomponents:

- **Card**: Main card wrapper with rounded borders and shadow
- **CardHeader**: Header section containing title and description
- **CardTitle**: Primary heading text for the card
- **CardDescription**: Secondary descriptive text below the title
- **CardContent**: Main content area for the card body
- **CardFooter**: Footer section for actions or additional information

## Usage

```rust
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
```

```rust
rsx! {
    Card {
        CardHeader {
            CardTitle { "Card Title" }
            CardDescription { "Card Description" }
        }
        CardContent {
            p { "Card Content" }
        }
        CardFooter {
            p { "Card Footer" }
        }
    }
}
```

## Examples

### Card with Action

Use the `CardFooter` to place action buttons at the bottom. This example shows how to compose a card with cancel and confirm actions using the [Button](/components/button) component.

<StaticCardAction />

### Card Group

Display multiple Card components in a responsive grid layout with consistent spacing. This example shows how to organize cards using Tailwind CSS grid utilities for building dashboard layouts and content galleries.

<StaticCardGroup />

### Card Reverse

Reverse the visual hierarchy of Card sections by repositioning header and footer elements. This example demonstrates flexible card composition patterns for creating varied content layouts.

<StaticCardReverse />

### Card SM

A compact card variant with reduced padding, ideal for dense UI panels like customizers, sidebars, or settings panels where space is limited.

<StaticCardSm />

## See Also

- [Badge](/components/badge)
- [Separator](/components/separator)
