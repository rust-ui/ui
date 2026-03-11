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
use crate::components::ui::button::Button;
```

```rust
<Button>"Click me"</Button>
```




## Examples

### Reactive Button

Button component that updates dynamically using Leptos signals to respond to state changes. This example demonstrates how to build reactive UI components in Rust that automatically re-render when underlying data changes.

<StaticButtonReactive />


### Variants

Available Button style variants include default, destructive, outline, secondary, ghost, and link. Each variant provides different visual styling while maintaining consistent behavior and accessibility standards across your Leptos application.

<StaticButtonVariants />


### Sizes

Button size options include small, default, large, and icon sizes. This example shows how to implement responsive button sizing in your Rust UI components to match different design requirements and use cases.

<StaticButtonSizes />

### Disabled

Disabled Button state with proper ARIA attributes for accessibility. This example demonstrates the correct way to implement non-interactive button states while maintaining semantic HTML and screen reader compatibility.

<StaticButtonDisabled />


### Overriding Button

Customize Button styles and behavior by overriding default properties with custom classes. This example shows how to extend the base button component while preserving type safety and component composition patterns in Leptos.

<StaticButtonOverride />


### Button with Clx

Use the Clx utility for conditional styling and dynamic class composition in Leptos components. This example demonstrates how to apply conditional Tailwind CSS classes based on component state or props for flexible styling.

<StaticButtonWithClx />


### With Href

Automatic conversion to semantic `a` tag with `data-slot` attribute when using the href prop. This example shows how the button component intelligently switches between button and anchor elements for proper HTML semantics and SEO.

<StaticButtonHref />


### Stateful

Button component that manages its own internal state using Leptos signals. This example demonstrates building self-contained interactive components with local state management, perfect for toggles, loading states, and complex user interactions.

<StaticButtonStateful />

### RTL

Button variants with Arabic labels. Directional arrow icons use `rtl:rotate-180` to preserve their meaning in right-to-left context.

<StaticButtonRtl />

## See Also

- [Button Group](/docs/components/button-group)
- [Button Action](/docs/components/button-action)
- [Pressable](/docs/components/pressable)
