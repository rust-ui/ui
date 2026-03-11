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

- **Card**: Main card wrapper component with rounded borders and shadow
- **CardHeader**: Header section containing title and description
- **CardTitle**: Primary heading text for the card
- **CardDescription**: Secondary descriptive text below the title
- **CardContent**: Main content area for the card body
- **CardFooter**: Footer section for actions or additional information
- **CardAction**: Optional top-right slot in `CardHeader` for badges, buttons, or trend indicators (uses CSS grid, col 2, spans rows 1–2)



## Usage

```rust
use crate::components::ui::card::{
    Card,
    CardAction,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
};
```

```rust
<Card>
    <CardHeader>
        <CardTitle>"Card Title"</CardTitle>
        <CardDescription>"Card Description"</CardDescription>
    </CardHeader>
    <CardContent>
        <p>"Card Content"</p>
    </CardContent>
    <CardFooter>
        <p>"Card Footer"</p>
    </CardFooter>
</Card>
```



## Examples

### Card Group

Display multiple Card components in a responsive grid layout with consistent spacing. This example shows how to organize cards using Tailwind CSS grid utilities for building dashboard layouts and content galleries in Leptos applications.

<StaticCardGroup />


### Card Reverse

Reverse the visual hierarchy of Card sections by repositioning header and footer elements. This example demonstrates flexible card composition patterns in Rust for creating varied content layouts while maintaining semantic HTML structure.

<StaticCardReverse />


### Card Action

Use `CardAction` inside `CardHeader` to place a badge, trend indicator, or any element in the top-right slot. It spans both title and description rows via CSS grid, keeping it vertically centered against the header content.

<StaticCardAction />


### Card SM

A compact card variant with reduced padding, ideal for dense UI panels like customizers, sidebars, or settings panels where space is limited.

<StaticCardSm />




### RTL

Card with an Arabic login form. Label alignment, flex direction, and logical spacing (`ms-auto`) all adapt to RTL automatically.

<StaticCardRtl />

## See Also

- [Card Carousel](/docs/components/card-carousel)
- [Badge](/docs/components/badge)
- [Avatar](/docs/components/avatar)
