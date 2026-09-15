# Rust/UI Icons

## Description

A collection of SVG-based icon components for Rust fullstack applications.

Built on top of Tailwind CSS and [tw_merge](https://crates.io/crates/tw_merge).


## Explore the Full Icon Library

Browse the complete collection now at [Rust/UI Icons](https://www.rust-ui.com/icons) — and find the perfect one for your next project!


## Installation

Please make sure to have this in your `Cargo.toml`:

```toml
icons = { version = "0.X", features = ["leptos"] } # For Leptos
icons = { version = "0.X", features = ["dioxus"] } # For Dioxus
```


## Usage

### Leptos

```rust
use leptos::prelude::*;
use icons::{ChevronUpDown, ChevronRight};

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <ChevronUpDown />
        <ChevronRight class="size-8" />
    }
}
```

### Dioxus

```rust
use dioxus::prelude::*;
use icons::Check;

#[component]
pub fn Mycomponent() -> Element {
    rsx! {
        Check { class: "text-green-500" }
    }
}
```


## Roadmap

- [X] `Leptos` port of lucide.dev icons.
- [X] `Dioxus` port of lucide.dev icons.
- [X] Search feature on Rust/UI.
- [ ] Filtering feature on Rust/UI.
- [ ] Full support for `animated` icons (WIP).


## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
