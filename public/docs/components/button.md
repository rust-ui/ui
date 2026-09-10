+++
title = "Button"
description = "Rust/UI component that displays a button or a component that looks like a button."
tags = ["button"]
is_new = false
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++

<StaticButton />

## Installation

<StaticInstallButton />

## Usage

```rust
use crate::ui::button::Button;
```

```rust
rsx! {
    Button { "Click me" }
}
```

## Examples

### Variants

Available Button style variants include default, secondary, outline, ghost, destructive, and link. Each variant provides different visual styling while maintaining consistent behavior and accessibility across your Dioxus application.

<StaticButtonVariants />

### Sizes

Button size options include small, default, and large. This example shows how to implement responsive button sizing in your Rust UI components to match different design requirements and use cases.

<StaticButtonSizes />

### Disabled

Disabled Button state with proper ARIA attributes for accessibility. This example demonstrates the correct way to implement non-interactive button states while maintaining semantic HTML and screen reader compatibility.

<StaticButtonDisabled />

### Stateful

Button component that manages its own internal state using Dioxus signals. This example demonstrates building self-contained interactive components with local state management, perfect for toggles and loading states.

<StaticButtonStateful />

### Reactive

Button component that updates dynamically using Dioxus signals to respond to state changes. This example demonstrates how to build reactive UI components in Rust that automatically re-render when underlying data changes.

<StaticButtonReactive />

### Overriding Button

Customize Button styles by overriding default properties with custom classes. This example shows how to extend the base button component while preserving type safety and component composition patterns in Dioxus.

<StaticButtonOverride />

### With Href

Automatic conversion to semantic `<a>` tag when using the `href` prop. The button component intelligently switches between button and anchor elements for proper HTML semantics and SEO.

<StaticButtonHref />

### Button Group

Group multiple buttons together with consistent spacing and shared borders. Useful for toolbars, segmented controls, and pagination.

<StaticButtonGroup />

### Button Group with Icons

Button group variant combining text labels with Lucide icons for enhanced visual clarity and compact action sets.

<StaticButtonGroupIcon />

## See Also

- [Input](/components/input)
- [Badge](/components/badge)
