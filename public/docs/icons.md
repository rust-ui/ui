+++
title = "Icons"
description = "Beautiful SVG icons for Rust applications. 1,600+ Lucide icons with full Leptos and Dioxus support, Tailwind CSS integration, and zero dependencies."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
order = 4
+++

A comprehensive collection of **1,600+ SVG icons** for Rust fullstack applications, built specifically for Leptos.

Our `icons` crate provides the complete [Lucide](https://lucide.dev) icon library with native Rust components, Tailwind CSS integration, and zero external dependencies.

## Installation

Add the `icons` crate to your `Cargo.toml`:

```toml
[dependencies]
icons = { version = "0.7", features = ["leptos"] }
```

## Quick Start


```rust
use leptos::prelude::*;
use icons::{ChevronRight, Heart, Star, Menu};

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <ChevronRight />
        <Heart class="text-red-500" />
        <Star class="size-6" />
        <Menu class="size-8 text-gray-700" />
    }
}
```

## Features

- **🎨 1,600+ Icons**: Complete Lucide icon library
- **🚀 Framework Support**: Native Leptos components
- **💨 Tailwind Ready**: Built-in Tailwind CSS class support
- **📦 Zero Dependencies**: No external runtime dependencies

## Browse All Icons

Explore the complete icon collection in **[Icons](https://rust-ui.com/icons)** — search through 1,600+ icons and copy the code directly into your project.

## Styling & Customization

All icons accept standard HTML attributes and Tailwind classes:

```rust
// Size variations
<Star class="size-4" />      // 16px
<Star class="size-6" />      // 24px
<Star class="size-8" />      // 32px

// Colors
<Heart class="text-red-500" />
<Check class="text-green-600" />

// Custom styling
<Menu class="size-6 text-gray-900 hover:text-blue-500" />
```

## Why Choose Our Icons?

- **Performance**: SVG-based with optimal bundle sizes
- **Consistency**: All icons follow the same design system
- **Accessibility**: Built-in accessibility attributes
- **Developer Experience**: Type-safe with excellent IDE support
- **Maintained**: Regular updates


Browse the full collection in **[Icons](https://rust-ui.com/icons)** and start building beautiful interfaces today!
