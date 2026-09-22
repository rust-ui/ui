+++
title = "Build Reusable Dioxus Components with Typed Props"
description = "Design Dioxus component APIs that stay flexible without turning every visual decision into a prop."
category = "Components"
publish_date = "2026-09-20"
last_updated = "2026-09-20"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Reusable Dioxus components"
keywords = ["Dioxus", "Rust", "components", "props", "composition"]
+++

Reusable UI comes from stable contracts. A component should make common cases easy, preserve semantic HTML, and leave room for composition. In Dioxus, typed props make that contract explicit without requiring a runtime schema.

## Start from usage

Write the call site before the implementation. If a card needs a title, description, and optional action, express exactly that shape.

```rust
rsx! {
    FeatureCard {
        eyebrow: "Components",
        title: "Typed by default",
        description: "Build UI APIs the compiler can explain.",
        action: rsx! { a { href: "/docs/components", "Browse components" } },
    }
}
```

This keeps the component readable. The caller describes content and intent; the component owns spacing, typography, borders, and responsive behavior.

## Use optional props for real variation

Not every visual difference deserves a prop. A variant is useful when it represents a repeated semantic choice, such as `Default`, `Outline`, or `Destructive`. A prop named `class` is useful as an escape hatch when the component explicitly supports composition.

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CardTone {
    #[default]
    Default,
    Muted,
    Accent,
}

#[component]
fn FeatureCard(
    eyebrow: String,
    title: String,
    description: String,
    #[props(default)] action: Option<Element>,
    #[props(default)] tone: CardTone,
) -> Element {
    let tone_class = match tone {
        CardTone::Default => "bg-card",
        CardTone::Muted => "bg-muted/40",
        CardTone::Accent => "bg-primary/5",
    };

    rsx! {
        article { class: "rounded-2xl border p-6 {tone_class}",
            p { class: "text-xs font-medium uppercase tracking-wide text-muted-foreground", "{eyebrow}" }
            h2 { class: "mt-3 text-xl font-semibold", "{title}" }
            p { class: "mt-2 text-sm text-muted-foreground", "{description}" }
            if let Some(action) = action {
                div { class: "mt-5", {action} }
            }
        }
    }
}
```

The component owns its tone mapping. Call sites do not need to know which Tailwind tokens create the visual result.

## Preserve composition points

Children are often the best extension point for a compound component. Use named props for required semantics, then allow an `Element` or `children` where users need to place custom content.

```rust
#[component]
fn Panel(title: String, children: Element) -> Element {
    rsx! {
        section { class: "rounded-2xl border bg-card p-6",
            h2 { class: "font-semibold", "{title}" }
            div { class: "mt-5", {children} }
        }
    }
}
```

Avoid prop explosions like `title_class`, `description_class`, `icon_size`, and `button_gap` until repeated consumers prove they need them. A composition slot usually scales better.

## Keep semantics in the primitive

A button should render a button. A navigation item should render a link. A heading should keep heading order. Accessibility becomes much easier when every call site does not need to remember basic semantics.

Typed props do not mean rigid components. They mean the flexible parts are visible. Build a small primitive, use it in two screens, then widen its API only after the second use exposes a real pattern.
