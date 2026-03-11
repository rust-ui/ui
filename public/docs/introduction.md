+++
title = "Introduction"
description = "The idea behind Rust/UI is to provide developers with the right tools to build Rust fullstack applications."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
order = 1
+++

**This is not a component library. It is how you build your component library.**

You know how most traditional component libraries work: you install a crate from crates.io, import the components, and use them in your app.

This approach works well until you need to customize a component to fit your design system or require one that isn't included in the library. **Often, you end up wrapping library components, writing workarounds to override styles, or mixing components from different libraries with incompatible APIs.**

This is what Rust/UI aims to solve. It is built around the following principles:

- **Composition:** Every component uses a common, composable interface, making them predictable.
- **Distribution:** A flat-file schema and command-line tool make it easy to distribute components.
- **Beautiful Defaults:** Carefully chosen default styles, so you get great design out-of-the-box.


## Composition

Every component in Rust/UI shares a common, composable interface. **If a component does not exist, we bring it in, make it composable, and adjust its style to match and work with the rest of the design system.**

_A shared, composable interface means it's predictable for both your team and LLMs. You are not learning different APIs for every new component. Even for third-party ones._

## Distribution

Rust/UI is also a code distribution system. It defines a schema for components and a CLI to distribute them.

- **Schema:** A flat-file structure that defines the components, their dependencies, and properties.
- **CLI:** A command-line tool to distribute and install components across projects with cross-framework support.

_You can use the schema to distribute your components to other projects or have AI generate completely new components based on existing schema._

## Beautiful Defaults

Rust/UI comes with a large collection of components that have carefully chosen default styles. They are designed to look good on their own and to work well together as a consistent system:

- **Good Out-of-the-Box:** Your UI has a clean and minimal look without extra work.
- **Unified Design:** Components naturally fit with one another. Each component is built to match the others, keeping your UI consistent.
- **Easily Customizable:** If you want to change something, it's simple to override and extend the defaults.

## AI-Ready

The design of Rust/UI makes it easy for AI tools to work with your code. Its open code and consistent API allow AI models to read, understand, and even generate new components.

_An AI model can learn how your components work and suggest improvements or even create new components that integrate with your existing design._