+++
title = "Installation"
description = "Get started with Rust/UI by installing the CLI tool and setting up your project with minimal dependencies and framework support."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
order = 2
+++

## Minimal Dependencies

Rust/UI is designed to be **as minimal as possible**. We don't rely on third party libraries like Radix UI, instead building components from the ground up to give you full control over your UI.

This approach means:

- **No External Dependencies:** Your components don't carry the weight of large third-party libraries.
- **Full Control:** Every line of code in your components is under your direct control and can be customized.
- **Better Performance:** Minimal overhead means faster load times and better runtime performance.
- **Security:** Fewer dependencies mean fewer potential security vulnerabilities in your application.

_Unlike other UI libraries that bundle heavyweight dependencies, Rust/UI components are built specifically for the Rust ecosystem with performance and simplicity in mind._

## Framework Support

**Currently, we support Leptos** - a modern, reactive web framework for Rust that provides excellent performance and developer experience.

Our Leptos integration offers:

- **Reactive Components:** Built-in support for Leptos signals and reactive primitives
- **Server-Side Rendering:** Full SSR support for better SEO and performance
- **Hydration:** Seamless client-side hydration for interactive components
- **Type Safety:** Full Rust type safety throughout your component tree

**Future Framework Support:**

We have plans to expand support to **Dioxus** in the near future. Dioxus is another excellent Rust web framework that focuses on:

- Cross-platform development (web, desktop, mobile)
- Virtual DOM with excellent performance
- React-like component model familiar to web developers

_Our goal is to provide the same high-quality component experience across multiple Rust web frameworks, giving you the flexibility to choose the right tool for your project._

## Installation

Install the UI CLI tool to get started:

```bash
cargo install ui-cli --force
```

See more details on **CLI section**.